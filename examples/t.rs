use std::env;

use yatsr::{
  file::save_image,
  geometry::Vec3,
  image::Image,
  image::PixImage,
  image_decoder::{self, Decoder, TGA},
  image_encoder::PPM,
  model::Model,
  pipeline::{render, Fragment, Shader},
};

struct Lambert {
  uniform_color: Vec3<f32>,
  uniform_width: u32,
  uniform_height: u32,
  uniform_light: Vec3<f32>,
  uniform_texture: PixImage,
  varying_uv: [Vec3<f32>;3],
  varying_vert_normals: [Vec3<f32>; 3],
}
impl Lambert {
  fn new(light: Vec3<f32>, color: Vec3<f32>, width: u32, height: u32,texture:PixImage) -> Lambert {
    Lambert {
      uniform_color: color,
      uniform_width: width,
      uniform_height: height,
      uniform_light: light,
      uniform_texture: texture,
      varying_vert_normals: [Vec3::default(), Vec3::default(), Vec3::default()],
      varying_uv: [Vec3::default(), Vec3::default(), Vec3::default()]
    }
  }
}
impl Shader for Lambert {
  fn vertext(&mut self, model: &Model, face: usize, nth_vert: usize) -> Vec3<f32> {
    self.varying_vert_normals[nth_vert] = model.normal(face, nth_vert);
    self.varying_uv[nth_vert] = model.texture_coord(face,nth_vert);
    let p = model.vert(face, nth_vert);
    let p = p + Vec3::new(1., 1., 1.); // [-1,1] => [0,2]
    let mut p = p * 0.5; // [0,2] => [0,1]
    p.x *= self.uniform_width as f32;
    p.y *= self.uniform_height as f32;
    p
  }

  fn fragment(
    &self,
    // 此点坐标
    pos: Vec3<f32>,
    // 此点处的质心坐标
    bar: Vec3<f32>,
  ) -> Fragment {
    //👇👇👇 法向量由顶点法向量插值得到,渲染结果将更平滑
    let normal = (self.varying_vert_normals[0] * bar.x
      + self.varying_vert_normals[1] * bar.y
      + self.varying_vert_normals[2] * bar.z)
      .normalize();
    let intensity = (normal * self.uniform_light).max(0.);
    let uv = (self.varying_uv[0]*bar.x + self.varying_uv[1]*bar.y+self.varying_uv[2]*bar.z);
    let color = self.uniform_texture.get_vec3f(uv.x,uv.y);
    Fragment::Color(self.uniform_color * intensity)
  }
}
fn main() {
  let model_path = env::args()
    .collect::<Vec<_>>()
    .get(1)
    .map(|f| f.clone())
    .unwrap_or(String::from("./models/earth/earth.obj"));
  let texture = TGA.decode(std::fs::read("./models/earth/texture.tga").unwrap());
  save_image("t.ppm",&texture,PPM);

  let mut model = Model::from_file(model_path).expect("Failed to load model:,");
  model.normalize_verts();
  let width = 500;
  let height = 500;
  let mut image = PixImage::new(width, height);
  let mut depth_buff = vec![f32::MIN; (width * height) as usize];
  render(
    &mut image,
    &mut depth_buff,
    &mut Lambert::new(
      Vec3::new(1., 1., 1.).normalize(),
      Vec3::new(0.8, 0.8, 0.8),
      width,
      height,
      texture
    ),
    &model,
    false,
  );
  save_image("./lambert.ppm", &image, PPM).expect("Failed to save image");
}

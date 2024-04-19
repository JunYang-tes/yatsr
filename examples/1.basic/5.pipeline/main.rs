use std::env;

use yatsr::{
  file::save_image,
  geometry::Vec3,
  image::{Image, PixImage},
  image_encoder::PPM,
  model::Object,
};

pub enum Fragment {
  Discard,
  Color(Vec3<f32>),
}

pub trait Shader {
  // 计算顶点在屏幕（渲染结果图像）上的位置
  fn vertext(&mut self, model: &Object, face: usize, nth_vert: usize) -> Vec3<f32>;
  // 对于三角形内部的每点调用fragment计算该点处的颜色.
  // 片元(Fragment) 既栅格化的三角形中的每一个点，如果没做超采样，那么这个点就是
  // 像素，否则就是子像素,是否是像素对于Shader而言不重要
  fn fragment(
    &self,
    // 此点坐标
    pos: Vec3<f32>,
    // 此点处的质心坐标
    bar: Vec3<f32>,
  ) -> Fragment;
}

type Point = Vec3<f32>;

pub fn barycentric(
  a: Vec3<f32>,
  b: Vec3<f32>,
  c: Vec3<f32>,
  x: f32,
  y: f32,
  //p: Vec3<f32>,
) -> (f32, f32, f32) {
  let alpha = ((b.x - x) * (c.y - b.y) + (y - b.y) * (c.x - b.x))
    / ((b.x - a.x) * (c.y - b.y) + (a.y - b.y) * (c.x - b.x));
  let beta = ((c.x - x) * (a.y - c.y) + (y - c.y) * (a.x - c.x))
    / ((c.x - b.x) * (a.y - c.y) + (b.y - c.y) * (a.x - c.x));
  (alpha, beta, 1. - alpha - beta)
}

fn draw_triangle<S: Shader>(
  img: &mut PixImage,
  depth_buff: &mut Vec<f32>,
  a: Point,
  b: Point,
  c: Point,
  shader: &mut S,
) {
  let min_x = a.x.min(b.x).min(c.x) as u32;
  let max_x = a.x.max(b.x).max(c.x).min((img.width - 1) as f32) as u32;
  let min_y = a.y.min(b.y).min(c.y) as u32;
  let max_y = a.y.max(b.y).max(c.y).min((img.height - 1) as f32) as u32;
  for y in min_y..=max_y {
    for x in min_x..=max_x {
      let (alpha, beta, gamma) = barycentric(a, b, c, x as f32, y as f32);
      if alpha < 0. || beta < 0. || gamma < 0. {
        continue;
      }
      let p = a * alpha + b * beta + c * gamma;
      let index = (y * img.height() + x) as usize;
      if p.z > depth_buff[index] {
        // 通过Fragment shader 计算没个像素的颜色
        match shader.fragment(p, Vec3::new(alpha, beta, gamma)) {
          Fragment::Color(c) => {
            depth_buff[index] = p.z;
            img.set_rgb(x, y, c);
          }
          Fragment::Discard => {}
        }
      }
    }
  }
}

fn render<S: Shader>(img: &mut PixImage, depth_buff: &mut Vec<f32>, shader: &mut S, model: &Object) {
  for n in 0..model.face_count() {
    // 通过顶点Shader 计算顶点的位置
    let a = shader.vertext(model, n, 0);
    let b = shader.vertext(model, n, 1);
    let c = shader.vertext(model, n, 2);
    draw_triangle(img, depth_buff, a, b, c, shader)
  }
}

struct FlatLambert {
  // GL 里的一个习惯，uniform 变量由pipeline设定，
  // varying 变量由顶点shader设定，在fragment shader 里使用
  uniform_color: Vec3<f32>,
  uniform_width: u32,
  uniform_height: u32,
  uniform_light: Vec3<f32>,

  varying_verts: [Vec3<f32>; 3],
}
impl FlatLambert {
  fn new(light: Vec3<f32>, color: Vec3<f32>, width: u32, height: u32) -> FlatLambert {
    FlatLambert {
      uniform_color: color,
      uniform_width: width,
      uniform_height: height,
      uniform_light: light,
      varying_verts: [Vec3::default(), Vec3::default(), Vec3::default()],
    }
  }
}
impl Shader for FlatLambert {
  fn vertext(&mut self, model: &Object, face: usize, nth_vert: usize) -> Vec3<f32> {
    let p = model.vert(face, nth_vert);
    self.varying_verts[nth_vert] = p;
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
    let normal = (self.varying_verts[1] - self.varying_verts[0])
      .cross_product(self.varying_verts[2] - self.varying_verts[0])
      .normalize();
    let intensity = (normal * self.uniform_light).max(0.);
    Fragment::Color(self.uniform_color * intensity)
  }
}

struct Lambert {
  uniform_color: Vec3<f32>,
  uniform_width: u32,
  uniform_height: u32,
  uniform_light: Vec3<f32>,
  varying_vert_normals: [Vec3<f32>; 3],
}
impl Lambert {
  fn new(light: Vec3<f32>, color: Vec3<f32>, width: u32, height: u32) -> Lambert {
    Lambert {
      uniform_color: color,
      uniform_width: width,
      uniform_height: height,
      uniform_light: light,
      varying_vert_normals: [Vec3::default(), Vec3::default(), Vec3::default()],
    }
  }
}
impl Shader for Lambert {
  fn vertext(&mut self, model: &Object, face: usize, nth_vert: usize) -> Vec3<f32> {
    self.varying_vert_normals[nth_vert] = model.normal(face, nth_vert);
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
    Fragment::Color(self.uniform_color * intensity)
  }
}

fn main() {
  let model_path = env::args()
    .collect::<Vec<_>>()
    .get(1)
    .map(|f| f.clone())
    .unwrap_or(String::from("./models/girl/D0901D64.obj"));
  let mut model = Object::from_file(model_path).expect("Failed to load model:,");
  model.normalize_verts();
  if !model.has_normal_vector() {
    println!("This model don't includes any vertex");
    std::process::exit(0)
  }
  let width = 500;
  let height = 500;
  let mut image = PixImage::new(width, height);
  let mut depth_buff = vec![f32::MIN; (width * height) as usize];
  render(
    &mut image,
    &mut depth_buff,
    &mut FlatLambert::new(
      Vec3::new(1., 1., 1.).normalize(),
      Vec3::new(0.8, 0.8, 0.8),
      width,
      height,
    ),
    &model,
  );
  save_image("./flat_lambert.ppm", &image, PPM).expect("Failed to save image");

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
    ),
    &model,
  );
  save_image("./lambert.ppm", &image, PPM).expect("Failed to save image");
}

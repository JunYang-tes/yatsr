use yatsr::prelude::*;
use yatsr::{file::save_image, image::Image, image_encoder::PPM, sdl};
struct MyShader {
  pub normal_texture: Texture,
  pub varying_uv: [Vec3<f32>; 3],
  pub varying_normal: [Vec3<f32>; 3],
  pub frag_shader: fn(shader: &MyShader, info: pipeline2::FragmentInfo) -> Fragment,
}
impl<M: Model> pipeline2::Shader<M> for MyShader {
  fn vertext(&mut self, model: &M, face: usize, nth_vert: usize) -> Vec4<f32> {
    let m = transform::rotate_y(45. * 3.14 / 180.);
    //let m = Mat4::identity();
    self.varying_uv[nth_vert] = model.texture_coord(face, nth_vert);
    self.varying_normal[nth_vert] = model.normal(face, nth_vert);
    &m * Vec4::from_point(&model.vert(face, nth_vert))
  }

  fn fragment(&self, info: pipeline2::FragmentInfo) -> Fragment {
    (self.frag_shader)(self, info)
  }
}

fn original_normal(shader: &MyShader, info: pipeline2::FragmentInfo) -> Fragment {
  let light = Vec3::new(1., 1., 1.).normalize();
  let normal = info.barycentric_interpolate(&shader.varying_normal);
  let i = (normal * light).max(0.);
  Fragment::Color(Vec3::new(1., 1., 1.) * i)
}
fn normal_from_texture(shader: &MyShader, info: pipeline2::FragmentInfo) -> Fragment {
  let uv = info.barycentric_interpolate(&shader.varying_uv);
  let normal = shader.normal_texture.get(uv.x, uv.y) * 2. - Vec3::new(1., 1., 1.);
  let m = transform::rotate_y(-45. * 3.14 / 180.).transpose();
  let normal = &m * &normal;
  let light = Vec3::new(1., 1., 1.).normalize();
  let i = (normal * light).max(0.);
  Fragment::Color(Vec3::new(1., 1., 1.) * i)
}

fn main() {
  let frag = std::env::args()
    .collect::<Vec<_>>()
    .get(1)
    .map(|a| String::from(a))
    .unwrap_or(String::from("original"));

  sdl::one_frame("Word space normalmap", 500, 500, |mut img| {
    let mut depth = vec![f32::MIN; img.width() as usize * img.height() as usize];
    let model = Object::from_file("./models/diablo/diablo3_pose.obj").unwrap();


    pipeline2::render(
      &mut img,
      &mut depth,
      &mut MyShader {
        normal_texture: Texture::neareat(util::load_image("./models/diablo/diablo3_pose_nm.tga")),
        frag_shader: if frag == "texture" {
          normal_from_texture
        } else {
          original_normal
        },
        varying_uv: [Vec3::default(), Vec3::default(), Vec3::default()],
        varying_normal: [Vec3::default(), Vec3::default(), Vec3::default()],
      },
      &model,
      0,
    );
    save_image("output.ppm", &img, PPM);
  })
}

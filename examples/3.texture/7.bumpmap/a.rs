// cargo run --example texture_bumpmap_a
use yatsr::prelude::*;

struct MyShader {
  varying_normals: [Vec3<f32>; 3],
  mat: Mat4,
}
impl<M: Model> pipeline2::Shader<M> for MyShader {
  fn vertext(&mut self, model: &M, face: usize, nth_vert: usize) -> Vec4<f32> {
    let p = Vec4::from_point(&model.vert(face, nth_vert));
    self.varying_normals[nth_vert] = model.normal(face, nth_vert);
    &self.mat * p
  }

  fn fragment(&self, info: pipeline2::FragmentInfo) -> Fragment {
    let normal = info.coordinate().normalize();
    let light = Vec3::new(1., 1., 1.).normalize();
    Fragment::Color(Vec3::new(1., 1., 1.) * (light * normal))
  }
}

fn main() {
  sdl::one_frame("A", 500, 500, |mut img| {
    let mut depth = vec![f32::MIN; img.width() as usize * img.height() as usize];
    let model = shape::Plane::new();

    pipeline2::render(
      &mut img,
      &mut depth,
      &mut MyShader {
        varying_normals: [Vec3::default(), Vec3::default(), Vec3::default()],
        mat: Transform::new().build(),
      },
      &model,
      0,
    );
    save_image("output.ppm", &img, PPM)
        .unwrap();
  })
}

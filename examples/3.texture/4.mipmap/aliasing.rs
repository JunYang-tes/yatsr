use yatsr::prelude::*;
struct MyShader {
  texture: Texture,
  mat: Mat4,
  varying_uvs: [Vec3<f32>; 3],
}
impl<M: Model> pipeline2::Shader<M> for MyShader {
  fn vertext(&mut self, model: &M, face: usize, nth_vert: usize) -> Vec4<f32> {
    self.varying_uvs[nth_vert] = model.texture_coord(face, nth_vert);
    &self.mat * Vec4::from_point(&model.vert(face, nth_vert))
  }

  fn fragment(
    &self,
    // 此点坐标
    pos: Vec3<f32>,
    // 此点处的质心坐标
    bar: Vec3<f32>,
  ) -> Fragment {
    let uv =
      self.varying_uvs[0] * bar.x + self.varying_uvs[1] * bar.y + self.varying_uvs[2] * bar.z;

    Fragment::Color(self.texture.get(uv.x, uv.y))
  }
}

fn main() {
  sdl::one_frame("Aliasing", 500, 500, |mut img| {
    let width = img.width();
    let height = img.height();
    let mut depth_buffer = vec![f32::MIN; (img.width() * img.height()) as usize];
    let model = shape::Plane::new();
    pipeline2::render(
      &mut img,
      &mut depth_buffer,
      &mut MyShader {
        texture: Texture::new(util::load_image("./textures/grid2.tga").flip_y()),
        mat: Transform::new()
          .rotate_x(-90. * 3.14 / 180.)
          .camera(
            Vec3::new(0., 1., 0.),
            Vec3::new(0.9, 0.2, 1.1),
            Vec3::new(0., 0., 0.),
          )
          .perspective(75., 1., -0.1, -10000.)
          .viewport(width as f32, height as f32)
          .build(),
        varying_uvs: [Vec3::default(), Vec3::default(), Vec3::default()],
      },
      &model,
      0,
    );
    save_image("output.ppm",&img,PPM);
  })
}

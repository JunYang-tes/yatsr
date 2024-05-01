use yatsr::prelude::*;
struct MyShader {
  texture: PixImage,
  mat: Mat4,
  varying_uvs: [Vec3<f32>; 3],
}
impl<M: Model> Shader<M> for MyShader {
  fn vertext(&mut self, model: &M, face: usize, nth_vert: usize) -> Vec3<f32> {
    self.varying_uvs[nth_vert] = model.texture_coord(face, nth_vert);
    &self.mat * &model.vert(face, nth_vert)
  }

  fn fragment(
    &self,
    // 此点坐标
    pos: Vec3<f32>,
    // 此点处的质心坐标
    bar: Vec3<f32>,
  ) -> Fragment {
    let uv =
      self.varying_uvs[0] * bar[0] + self.varying_uvs[1] * bar[1] + self.varying_uvs[2] * bar[2];
    Fragment::Color(self.texture.get_vec3f(uv.x, uv.y))
  }
}

fn main() {
  sdl::one_frame("mosaic", 600, 600, |mut img| {
    let model = shape::Plane::new();
    render(
      &mut img,
      &mut vec![f32::MIN; 600 * 600],
      &mut MyShader {
        texture: util::load_image("./textures/sui.tga"),
        mat: transform::viewport(600., 600.),
        varying_uvs: [Vec3::default(), Vec3::default(), Vec3::default()],
      },
      &model,
      false,
    );
  })
}

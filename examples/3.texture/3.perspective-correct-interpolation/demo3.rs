use yatsr::prelude::*;
struct MyShader {
  texture: PixImage,
  mat: Mat4,
  varying_uvs: [Vec3<f32>; 3],
  varying_w: [f32; 3],
}
impl<M: Model> Shader<M> for MyShader {
  fn vertext(&mut self, model: &M, face: usize, nth_vert: usize) -> Vec3<f32> {
    self.varying_uvs[nth_vert] = model.texture_coord(face, nth_vert);
    let vert = &self.mat * Vec4::from_point(&model.vert(face, nth_vert));
    self.varying_w[nth_vert] = vert.w;
    vert.to_3d_point()
  }

  fn fragment(
    &self,
    // 此点坐标
    pos: Vec3<f32>,
    // 此点处的质心坐标
    bar: Vec3<f32>,
  ) -> Fragment {
    let uv = self.varying_uvs[0] * (bar.x / self.varying_w[0])
      + self.varying_uvs[1] * (bar.y / self.varying_w[1])
      + self.varying_uvs[2] * (bar.z / self.varying_w[2]);
    let l = (1. / self.varying_w[0]) * bar.x
      + (1. / self.varying_w[1]) * bar.y
      + (1. / self.varying_w[2]) * bar.z;

    Fragment::Color(self.texture.get_vec3f(uv.x / l, uv.y / l))
  }
}

fn main() {
  let mut degree = 0.;
  sdl::one_frame("Interpolation correction", 600, 600, |mut img| {
    let mut depth_buffer = vec![f32::MIN; 600 * 600];
    let model = shape::Plane::new();
    let pos = &transform::rotate_y(degree * 3.15 / 180.) * &Vec3::new(0., 1.5, 1.5);
    render(
      &mut img,
      &mut depth_buffer,
      &mut MyShader {
        texture: util::load_image("./textures/grid2.tga").flip_y(),
        mat: Transform::new()
          .rotate_x(-90. * 3.14 / 180.)
          .camera(Vec3::new(0., 1., 0.), pos, Vec3::new(0., 0., 0.))
          .perspective(65., 1., -1., -4.)
          .viewport(600., 600.)
          .build(),
        varying_uvs: [Vec3::default(), Vec3::default(), Vec3::default()],
        varying_w: [1., 1., 1.],
      },
      &model,
      false,
    );
  })
}

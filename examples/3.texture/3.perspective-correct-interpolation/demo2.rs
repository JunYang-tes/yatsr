use yatsr::prelude::*;
struct MyShader {
  texture: PixImage,
  mat: Mat4,
  varying_uvs: [Vec3<f32>; 3],
  varying_verts: [Vec3<f32>; 3],
}
impl<M: Model> Shader<M> for MyShader {
  fn vertext(&mut self, model: &M, face: usize, nth_vert: usize) -> Vec3<f32> {
    self.varying_uvs[nth_vert] = model.texture_coord(face, nth_vert);
    self.varying_verts[nth_vert] = model.vert(face, nth_vert);
    &self.mat * &model.vert(face, nth_vert)
  }

  fn fragment(
    &self,
    // 此点坐标
    pos: Vec3<f32>,
    // 此点处的质心坐标
    bar: Vec3<f32>,
  ) -> Fragment {
    //pos，为该片元的屏幕坐标， 通过逆矩阵计算该片元的世界坐标
    let p = &self.mat.invert() * &pos;
    // 基于世界坐标计算重心坐标
    let (a, b, c) = yatsr::pipeline::barycentric(
      self.varying_verts[0],
      self.varying_verts[1],
      self.varying_verts[2],
      p.x,
      p.y,
    );
    // 通过顶点的uv坐标以及重心坐标来计算该点的uv坐标
    let uv = self.varying_uvs[0] * a + self.varying_uvs[1] * b + self.varying_uvs[2] * c;
    Fragment::Color(self.texture.get_vec3f(uv.x, uv.y))
  }
}

fn main() {
  let mut degree = 0.;
  sdl::one_frame("World space uv interpolation", 600, 600, |mut img| {
    let mut depth_buffer = vec![f32::MIN; 600 * 600];
    let model = shape::Plane::new();
    let pos = &transform::rotate_y(degree * 3.15 / 180.) * &Vec3::new(0., 1.5, 1.5);
    render(
      &mut img,
      &mut depth_buffer,
      &mut MyShader {
        texture: util::load_image("./textures/grid2.tga"),
        mat: Transform::new()
          .rotate_x(-90. * 3.14 / 180.)
          .camera(Vec3::new(0., 1., 0.), pos, Vec3::new(0., 0., 0.))
          .perspective(65., 1., -1., -4.)
          .viewport(600., 600.)
          .build(),
        varying_uvs: [Vec3::default(), Vec3::default(), Vec3::default()],
        varying_verts: [Vec3::default(), Vec3::default(), Vec3::default()],
      },
      &model,
      false,
    );
  })
}

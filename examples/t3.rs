use yatsr::image_decoder::Decoder;
use yatsr::prelude::*;
use yatsr::sdl::frame;

struct S<'a> {
  texture: &'a PixImage,
  texture_mat: Mat4,
  mat: Mat4,
  varying_uvs: [Vec3<f32>; 3],
}
impl<'a> Shader for S<'a> {
  fn vertext(&mut self, model: &Model, face: usize, nth_vert: usize) -> Vec3<f32> {
    let v = model.vert(face, nth_vert);
    let uv = &self.texture_mat * &v;
    self.varying_uvs[nth_vert] = uv;
    &self.mat * &v
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

    Fragment::Color(self.texture.get_vec3f(uv.x, uv.y))
  }
}

fn main() {
  let mut model = Model::from_file("./models/earth/earth.obj").unwrap();
  model.normalize_verts();
  let loader = yatsr::image_decoder::TGA;
  let texture =
    loader.decode(std::fs::read("/home/yj/projects/sync/tinyrenderer/grid.tga").unwrap());
  frame("Demo", 500, 500, |mut img, _| {
    let mut depth = vec![f32::MIN; (img.1 * img.2) as usize];
    render(
      &mut img,
      &mut depth,
      &mut S {
        texture_mat: Transform::new()
          .then_mat(&transform::orthographic(-2., 2., -2., 2., -2., 2.))
          .translate(1., 1., 1.)
          .scale(0.5, 0.5, 0.5)
          .build(),
        mat: Transform::new()
          .scale(0.8, 0.8, 0.8)
          .then_mat(&transform::camera(
            Vec3::new(0., 1., 0.),
            Vec3::new(1., 1., 1.),
            Vec3::new(0., 0., 0.),
          ))
          .then_mat(&transform::viewport(500., 500.))
          .build(),
        texture: &texture,
        varying_uvs: [Vec3::default(), Vec3::default(), Vec3::default()],
      },
      &model,
      false,
    );
  });
}

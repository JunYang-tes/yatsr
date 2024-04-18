use yatsr::prelude::*;

struct MyShader<'a> {
  texture: &'a PixImage,
  mat: Mat4,
  varying_uvs: [Vec3<f32>; 3],
}
impl<'a> Shader for MyShader<'a> {
  fn vertext(&mut self, model: &Model, face: usize, nth_vert: usize) -> Vec3<f32> {
    let uv = model.texture_coord(face, nth_vert);
    self.varying_uvs[nth_vert] = uv;
    let v = model.vert(face, nth_vert);
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
  let model = Model::from_file("./models/spot/spot_triangulated.obj").unwrap();
  let texture = util::load_image("./models/spot/spot_texture.tga");
  let texture = if texture.image_origin() == yatsr::image::ImageOriginPos::LeftTop {
    texture.flip_y()
  } else {
    texture
  };
  let mut degree = 0.;
  sdl::frame("diffuse texture", 500, 500, |mut img, fps| {
    let mut depth = vec![f32::MIN; (img.1 * img.2) as usize];
    let m = Transform::new().rotate_y(degree * 3.14 / 180.).build();
    let v = transform::camera(
      Vec3::new(0., 1., 0.),
      Vec3::new(1., 1., 1.),
      Vec3::new(0., 0., 0.),
    );
    let p = transform::orthographic(-1.1, 1.1, -1.1, 1.1, -1.1, 1.1);
    let vp = transform::viewport(500., 500.);
    render(
      &mut img,
      &mut depth,
      &mut MyShader {
        texture: &texture,
        mat: &vp * &p * &v * &m,
        varying_uvs: [Vec3::default(), Vec3::default(), Vec3::default()],
      },
      &model,
      false,
    );
    if fps > 0. {
      let t = 1. / fps;
      let speed = 45.; // per seconds
      degree += t * speed;
    }
  })
}


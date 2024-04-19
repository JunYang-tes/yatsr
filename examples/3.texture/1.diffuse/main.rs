use yatsr::prelude::*;

struct MyShader<'a> {
  texture: &'a PixImage,
  invert_transpose: Mat4,
  mat: Mat4,
  varying_uvs: [Vec3<f32>; 3],
  varying_normals: [Vec3<f32>; 3],
}
impl<'a, O: yatsr::model::Model> Shader<O> for MyShader<'a> {
  fn vertext(&mut self, model: &O, face: usize, nth_vert: usize) -> Vec3<f32> {
    let normal = model.normal(face, nth_vert);
    // 从模型中获取该顶点的纹理坐标
    let uv = model.texture_coord(face, nth_vert);
    self.varying_normals[nth_vert] = &self.invert_transpose * &normal;
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
    let normal = (self.varying_normals[0] * bar.x
      + self.varying_normals[1] * bar.y
      + self.varying_normals[2] * bar.z)
      .normalize();
    let i = (normal * Vec3::new(1., 1., 0.).normalize()).max(0.);

    // 通过对顶点纹理坐标插值，得到该点的纹理坐标
    let uv =
      self.varying_uvs[0] * bar.x + self.varying_uvs[1] * bar.y + self.varying_uvs[2] * bar.z;
    let color = self.texture.get_vec3f(uv.x, uv.y) // 用texture做为该点的颜色
        * i; // 乘以此处光的强度
    Fragment::Color(color)
  }
}

fn main() {
  let model = Object::from_file("./models/spot/spot_triangulated.obj").unwrap();
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
        invert_transpose: m.invert().transpose().clone(),
        mat: &vp * &p * &v * &m,
        varying_uvs: [Vec3::default(), Vec3::default(), Vec3::default()],
        varying_normals: [Vec3::default(), Vec3::default(), Vec3::default()],
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


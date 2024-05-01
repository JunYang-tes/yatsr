use yatsr::prelude::*;
struct Texture {
  image: PixImage,
}
impl Texture {
  fn get(&self, u: f32, v: f32) -> Vec3<f32> {
    let x = (self.image.width() - 1) as f32 * u;
    let y = (self.image.height() - 1) as f32 * v;
    let c1 = self.image.get_rgbf(x.floor() as u32, y.floor() as u32);
    let c2 = self.image.get_rgbf(x.ceil() as u32, y.floor() as u32);
    let c3 = util::linear_interpolation(x - x.floor(), c1, c2);
    let c4 = self.image.get_rgbf(x.floor() as u32, y.ceil() as u32);
    let c5 = self.image.get_rgbf(x.ceil() as u32, y.ceil() as u32);
    let c6 = util::linear_interpolation(x - x.floor(), c4, c5);
    util::linear_interpolation(y - y.floor(), c3, c6)
  }
}

struct MyShader {
  texture: Texture,
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
    Fragment::Color(self.texture.get(uv.x, uv.y))
  }
}

fn main() {
  sdl::one_frame("Bilinear", 600, 600, |mut img| {
    let model = shape::Plane::new();
    render(
      &mut img,
      &mut vec![f32::MIN; 600 * 600],
      &mut MyShader {
        texture: Texture {
          image: util::load_image("./textures/sui.tga"),
        },
        mat: transform::viewport(600., 600.),
        varying_uvs: [Vec3::default(), Vec3::default(), Vec3::default()],
      },
      &model,
      false,
    );
  })
}

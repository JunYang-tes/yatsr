use yatsr::prelude::*;
struct SAT {
  texture: Texture,
  data: Vec<Vec3<f32>>,
}

impl SAT {
  fn new(texture: PixImage) -> SAT {
    let width = texture.width() as usize;
    let height = texture.width() as usize;
    let mut data = vec![Vec3::default(); (texture.width() * texture.height()) as usize];
    for row in 0..height {
      for col in 0..width {
        let (a, b, c) = match (row, col) {
          (0, 0) => (Vec3::default(), Vec3::default(), Vec3::default()),
          (0, col) => (
            data[(col - 1) + row * width],
            Vec3::default(),
            Vec3::default(),
          ),
          (row, 0) => (
            Vec3::default(),
            data[col + (row - 1) * width],
            Vec3::default(),
          ),
          (row, col) => (
            data[(col - 1) + row * width],
            data[col + (row - 1) * width],
            data[(col - 1) + (row - 1) * width],
          ),
        };
        data[col + row * width] = a + b - c + texture.get_rgbf(col as u32, row as u32);
      }
    }

    SAT {
      texture: Texture::neareat(texture),
      data,
    }
  }
  fn get(&self, left_bottom: (f32, f32), right_top: (f32, f32)) -> Vec3<f32> {
    let left_bottom = (left_bottom.0.clamp(0., 1.), left_bottom.1.clamp(0., 1.));
    let right_top = (right_top.0.clamp(0., 1.), right_top.1.clamp(0., 1.));
    let (x, y) = (
      left_bottom.0 * (self.texture.image.width() - 1) as f32,
      left_bottom.1 * (self.texture.image.height() - 1) as f32,
    );
    let (rt_x, rt_y) = (
      right_top.0 * (self.texture.image.width() - 1) as f32,
      right_top.1 * (self.texture.image.height() - 1) as f32,
    );

    if (rt_x - x) <= 1. || (rt_y - y) <= 1. {
      return self.texture.get(
        (left_bottom.0 + right_top.0) / 2.,
        (left_bottom.1 + right_top.1) / 2.,
      );
    }
    let width = self.texture.image.width() as usize;
    let area = ((rt_y - y) * (rt_x - x));
    let x = x.round() as usize;
    let y = y.round() as usize;
    let rt_x = rt_x.round() as usize;
    let rt_y = rt_y.round() as usize;
    let (x, rt_x) = if x == rt_x {
      if rt_x == width - 1 {
        (x - 1, rt_x)
      } else {
        (x, rt_x + 1)
      }
    } else {
      (x, rt_x)
    };

    let total =
      self.data[width * rt_y + rt_x] - self.data[width * rt_y + x] - self.data[width * y + rt_x]
        + self.data[width * y + x];
    let c = total * (1. / area);
    if c.z <= 0. {
      return Vec3::new(1., 0., 0.);
    }
    return c;
  }
}

struct MyShader {
  texture: SAT,
  mat: Mat4,
  invert: Mat4,
  screen_size: f32,
  varying_uvs: [Vec3<f32>; 3],
  varying_verts: [Vec4<f32>; 3],
}
fn barycentric(a: Vec4<f32>, b: Vec4<f32>, c: Vec4<f32>, x: f32, y: f32) -> Vec3<f32> {
  let wa = a.w;
  let wb = b.w;
  let wc = c.w;
  let a = a.to_3d_point();
  let b = b.to_3d_point();
  let c = c.to_3d_point();
  let bar = pipeline2::barycentric(a, b, c, x, y);
  let k = 1. / wa * bar.0 + 1. / wb * bar.1 + 1. / wc * bar.2;
  Vec3::new(bar.0 / wa / k, bar.1 / wb / k, bar.2 / wc / k)
}

impl<M: Model> pipeline2::Shader<M> for MyShader {
  fn vertext(&mut self, model: &M, face: usize, nth_vert: usize) -> Vec4<f32> {
    self.varying_uvs[nth_vert] = model.texture_coord(face, nth_vert);
    let p = &self.mat * Vec4::from_point(&model.vert(face, nth_vert));
    self.varying_verts[nth_vert] = p;
    p
  }

  fn fragment(
    &self,
    // 此点坐标
    pos: Vec3<f32>,
    // 此点处的质心坐标
    bar: Vec3<f32>,
  ) -> Fragment {
    let a = barycentric(
      self.varying_verts[0],
      self.varying_verts[1],
      self.varying_verts[2],
      pos.x - 0.5,
      pos.y - 0.5,
    );
    let b = barycentric(
      self.varying_verts[0],
      self.varying_verts[1],
      self.varying_verts[2],
      pos.x - 0.5,
      pos.y + 0.5,
    );
    let c = barycentric(
      self.varying_verts[0],
      self.varying_verts[1],
      self.varying_verts[2],
      pos.x + 0.5,
      pos.y - 0.5,
    );
    let d = barycentric(
      self.varying_verts[0],
      self.varying_verts[1],
      self.varying_verts[2],
      pos.x + 0.5,
      pos.y + 0.5,
    );

    let a_uv = self.varying_uvs[0] * a.x + self.varying_uvs[1] * a.y + self.varying_uvs[2] * a.z;
    let b_uv = self.varying_uvs[0] * b.x + self.varying_uvs[1] * b.y + self.varying_uvs[2] * b.z;
    let c_uv = self.varying_uvs[0] * c.x + self.varying_uvs[1] * c.y + self.varying_uvs[2] * c.z;
    let d_uv = self.varying_uvs[0] * d.x + self.varying_uvs[1] * d.y + self.varying_uvs[2] * d.z;
    // 找texture space的包围盒
    let max_y = a_uv.y.max(b_uv.y).max(c_uv.y).max(d_uv.y);
    let max_x = a_uv.x.max(b_uv.x).max(c_uv.x).max(d_uv.x);
    let min_y = a_uv.y.min(b_uv.y).min(c_uv.y).min(d_uv.y);
    let min_x = a_uv.x.min(b_uv.x).min(c_uv.x).min(d_uv.x);

    Fragment::Color(self.texture.get((min_x, min_y), (max_x, max_y)))
  }
}

fn main() {
  sdl::one_frame("SAT", 600, 600, |mut img| {
    let ripmap = SAT::new(util::load_image("./textures/grid1.tga"));
    let mut depth_buffer = vec![f32::MIN; 600 * 600];
    let model = shape::Plane::new();
    let mat = Transform::new()
      .rotate_x(-90. * 3.14 / 180.)
      .camera(
        Vec3::new(0., 1., 0.),
        Vec3::new(0.9, 0.2, 1.1),
        //Vec3::new(0.9, 0.2, 1.8),
        Vec3::new(0., 0., 0.),
      )
      .perspective(75., 1., -0.1, -10000.)
      .viewport(img.width() as f32, img.height() as f32)
      .build();
    let invert = mat.invert();
    pipeline2::render(
      &mut img,
      &mut depth_buffer,
      &mut MyShader {
        screen_size: 600.,
        texture: ripmap,
        mat,
        invert,
        varying_uvs: [Vec3::default(), Vec3::default(), Vec3::default()],
        varying_verts: [Vec4::default(), Vec4::default(), Vec4::default()],
      },
      &model,
      3,
    );
    save_image("sat.ppm", &img, PPM);
  });
}

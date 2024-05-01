use yatsr::prelude::*;

fn vhalf(img: &PixImage) -> PixImage {
  let height = (img.height() / 2).max(1);
  let width = img.width();
  let mut ret = PixImage::new(width, height);
  for r in (0..img.height() - 1).step_by(2) {
    for c in (0..width) {
      let color = (img.get_rgbf(c, r) + img.get_rgbf(c, r + 1)) * 0.5;
      ret.set_rgb(c, r / 2, color);
    }
  }
  ret
}

fn hhalf(img: &PixImage) -> PixImage {
  let height = img.height();
  let width = (img.width() / 2).max(1);
  let mut ret = PixImage::new(width, height);
  for r in 0..height {
    for c in (0..img.width() - 1).step_by(2) {
      let color = (img.get_rgbf(c, r) + img.get_rgbf(c + 1, r)) * 0.5;
      ret.set_rgb(c / 2, r, color);
    }
  }
  ret
}

struct Ripmap {
  images: Vec<Vec<Texture>>,
}
impl Ripmap {
  fn new(img: PixImage) -> Ripmap {
    let mut height = img.height();
    let mut width = img.width();
    let mut first_row = vec![Texture::neareat(img)];
    let mut img = &first_row[0].image;
    while width > 0 {
      first_row.push(Texture::neareat(hhalf(img)));
      width = width / 2;
      img = &first_row[first_row.len() - 1].image;
    }

    let mut images: Vec<Vec<Texture>> = vec![first_row];
    while height > 1 {
      let mut row: Vec<Texture> = vec![];
      for texture in &images[images.len() - 1] {
        let img: &PixImage = &texture.image;
        row.push(Texture::neareat(vhalf(img)));
      }
      images.push(row);
      height = height / 2;
    }

    // for row in images.iter().enumerate() {
    //     for col in row.1.iter().enumerate() {
    //         save_image(format!("{}-{}.ppm",row.0,col.0),&col.1.image,PPM);
    //
    //     }
    //
    // }

    Ripmap { images }
  }
  fn get(&self, level_x: f32, level_y: f32, u: f32, v: f32) -> Vec3<f32> {
    let level_x = level_x.clamp(0., (self.images[0].len() - 1) as f32);
    let level_y = level_y.clamp(0., (self.images.len() - 1) as f32);

    let c1 = self.images[level_y.floor() as usize][level_x.floor() as usize].get(u, v);
    let c2 = self.images[level_y.ceil() as usize][level_x.floor() as usize].get(u, v);
    let c3 = util::linear_interpolation(level_y - level_y.floor(), c1, c2);
    let c5 = self.images[level_y.floor() as usize][level_x.ceil() as usize].get(u, v);
    let c6 = self.images[level_y.ceil() as usize][level_x.ceil() as usize].get(u, v);
    let c7 = util::linear_interpolation(level_y - level_y.floor(), c5, c6);
    let mut c= util::linear_interpolation(level_x - level_x.floor(), c3, c7);
    c
  }
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

struct MyShader {
  texture: Ripmap,
  mat: Mat4,
  invert: Mat4,
  screen_size: f32,
  varying_uvs: [Vec3<f32>; 3],
  varying_verts: [Vec4<f32>; 3],
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
    let uv =
      self.varying_uvs[0] * bar.x + self.varying_uvs[1] * bar.y + self.varying_uvs[2] * bar.z;
    // 计算相对该点上面的一个点的uv坐标
    let top = barycentric(
      self.varying_verts[0],
      self.varying_verts[1],
      self.varying_verts[2],
      pos.x,
      pos.y - 1.,
    );
    let top_uv =
      self.varying_uvs[0] * top.x + self.varying_uvs[1] * top.y + self.varying_uvs[2] * top.z;
    // 计算相对该点右边一点的uv坐标
    let right = barycentric(
      self.varying_verts[0],
      self.varying_verts[1],
      self.varying_verts[2],
      pos.x + 1.,
      pos.y,
    );
    let right_uv =
      self.varying_uvs[0] * right.x + self.varying_uvs[1] * right.y + self.varying_uvs[2] * right.z;
    let screen_size = self.screen_size;

    let l1 = (((top_uv.x - uv.x) * screen_size).powi(2)
      + (screen_size * (top_uv.y - uv.y)).powi(2))
    .sqrt();
    let l2 = (((right_uv.x - uv.x) * screen_size).powi(2)
      + (screen_size * (right_uv.y - uv.y)).powi(2))
    .sqrt();
    // 如果l1 > l2 , 说明是一个竖的
    //

    //Fragment::Color(self.texture.get(l1.log2(), l1.log2(), uv.x, uv.y))
    //Fragment::Color(self.texture.get(l2.log2(), l2.log2(), uv.x, uv.y))
    //Fragment::Color(self.texture.get(l1.log2(), l2.log2(), uv.x, uv.y))
    Fragment::Color(self.texture.get(l2.log2(), l1.log2(), uv.x, uv.y))
  }
}

fn main() {
  sdl::one_frame("Ripmap", 600, 600, |mut img| {
    let ripmap = Ripmap::new(util::load_image("./textures/grid3.tga"));
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
    save_image("ripmap.ppm", &img, PPM);
  });
}

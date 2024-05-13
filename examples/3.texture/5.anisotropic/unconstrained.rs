use yatsr::{geometry::Vec2, prelude::*};
fn quarter(img: &PixImage) -> PixImage {
  let width = (img.width() / 2).max(1);
  let height = (img.height() / 2).max(1);
  let mut ret = PixImage::new(width, height);
  for r in (0..img.height).step_by(2) {
    for c in (0..img.width).step_by(2) {
      let color = (img.get_rgbf(c, r)
        + img.get_rgbf((c + 1).min(img.width - 1), r)
        + img.get_rgbf((c + 1).min(img.width - 1), (r + 1).min(img.height - 1))
        + img.get_rgbf((c + 1).min(img.width - 1), r))
        * 0.25;
      ret.set_rgb(c / 2, r / 2, color);
    }
  }
  ret
}

struct Mipmap {
  images: Vec<Texture>,
}
impl Mipmap {
  fn new(img: PixImage) -> Mipmap {
    let mut images = vec![Texture::new(img)];
    let mut img = &images[0].image;
    while img.width() > 1 || img.height() > 1 {
      let texture = Texture::new(quarter(&img));
      images.push(texture);
      img = &images[images.len() - 1].image;
    }
    Mipmap { images }
  }
  fn get_by_level(&self, level: f32, u: f32, v: f32) -> Vec3<f32> {
    let c1 = self.images[level.floor() as usize].get(u, v);
    let c2 = self.images[level.ceil() as usize].get(u, v);
    util::linear_interpolation(level - level.floor(), c1, c2)
  }
  fn get(&self, a: Vec2<f32>, b: Vec2<f32>, c: Vec2<f32>, d: Vec2<f32>) -> Vec3<f32> {
    // 四条边的长度
    let ab = (a - b).norm();
    let bc = (b - c).norm();
    let cd = (c - d).norm();
    let da = (d - a).norm();

    // 四条边的中点
    let mid_ab = (a + b) * 0.5;
    let mid_bc = (b + c) * 0.5;
    let mid_cd = (c + d) * 0.5;
    let mid_da = (d + a) * 0.5;

    //以ab边中点为起点，cd边中点为终点的向量
    let ab_cd = mid_cd - mid_ab;
    let bc_da = mid_da - mid_bc;
    let len_ab_cd = ab_cd.norm();
    let len_bc_da = bc_da.norm();
    let (
      sorter,          // 用短边计算mipmap的level
      longer,          // 用长边以及短边确定采样的数量,对应README.md 里的l
      anisotropic_dir, // 沿这个向量的方向采样,对应README.md 里的v
      mid,             // 向量起点边的中点
    ) = if len_ab_cd > len_bc_da {
      (ab.min(cd), bc.max(da), ab_cd.normalize(), mid_ab)
    } else {
      (bc.min(da), ab.max(cd), bc_da.normalize(), mid_bc)
    };
    let sample_count = (longer / sorter).min(16.);
    let level = (sorter * self.images[0].image.width() as f32).log2();
    if sample_count <= 1. {
      return self.get_by_level(level, a.x, a.y);
    }
    // 下面代码以纯色显示不同的采样数量的区域
    // if sample_count <= 2. {
    //   return Vec3::new(1., 0., 0.);
    // } else if sample_count <= 3. {
    //   return Vec3::new(0., 1., 0.);
    // } else if sample_count <= 4. {
    //   return Vec3::new(0., 0., 1.);
    // } else if sample_count <= 5. {
    //   return Vec3::new(1., 1., 0.);
    // } else if sample_count <= 6. {
    //   return Vec3::new(1., 0., 1.);
    // } else if sample_count <= 7. {
    //   return Vec3::new(0., 1., 1.);
    // }

    let mut color = Vec3::default();
    let longer_over_sc = longer / sample_count;
    // 对应README.md 里的 s = m + (l/2c)v
    let start_point = mid + anisotropic_dir * (longer / (2. * sample_count));
    let sample_count = sample_count as u8;
    for i in 0..(sample_count as u8) {
      // 对应README.md 里的 p(i) = s + i(l/c)v
      let uv = start_point + anisotropic_dir * (i as f32 * longer_over_sc);
      color = color + self.get_by_level(level, uv.x, uv.y);
    }
    color * (1. / (sample_count) as f32)
  }
}

struct MyShader {
  texture: Mipmap,
  mat: Mat4,
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
    info: pipeline2::FragmentInfo
  ) -> Fragment {
    let a =info.barycentric(
      info.pos.x - 0.5,
      info.pos.y - 0.5,
    );
    let b =info.barycentric(
      info.pos.x - 0.5,
      info.pos.y + 0.5,
    );
    let c =info.barycentric(
      info.pos.x + 0.5,
      info.pos.y + 0.5,
    );
    let d =info.barycentric(
      info.pos.x + 0.5,
      info.pos.y - 0.5,
    );

    let a_uv = self.varying_uvs[0] * a.x + self.varying_uvs[1] * a.y + self.varying_uvs[2] * a.z;
    let b_uv = self.varying_uvs[0] * b.x + self.varying_uvs[1] * b.y + self.varying_uvs[2] * b.z;
    let c_uv = self.varying_uvs[0] * c.x + self.varying_uvs[1] * c.y + self.varying_uvs[2] * c.z;
    let d_uv = self.varying_uvs[0] * d.x + self.varying_uvs[1] * d.y + self.varying_uvs[2] * d.z;

    Fragment::Color(self.texture.get(
      Vec2::new(a_uv.x, a_uv.y),
      Vec2::new(b_uv.x, b_uv.y),
      Vec2::new(c_uv.x, c_uv.y),
      Vec2::new(d_uv.x, d_uv.y),
    ))
  }
}

fn main() {
  sdl::one_frame("Unconstrained anisotropic", 600, 600, |mut img| {
    let ripmap = Mipmap::new(util::load_image("./textures/grid1.tga"));
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
      .build();
    pipeline2::render(
      &mut img,
      &mut depth_buffer,
      &mut MyShader {
        texture: ripmap,
        mat,
        varying_uvs: [Vec3::default(), Vec3::default(), Vec3::default()],
        varying_verts: [Vec4::default(), Vec4::default(), Vec4::default()],
      },
      &model,
      3,
    );
    save_image("unconstrained.ppm", &img, PPM);
  });
}

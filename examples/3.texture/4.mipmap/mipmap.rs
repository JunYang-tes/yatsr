use std::env;

use yatsr::prelude::*;
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
}

fn mipmap_visual() -> Mipmap {
  let colors = vec![
    Vec3::new(1., 0., 0.), //#ff0000
    Vec3::new(0., 1., 0.), //#00ff00
    Vec3::new(0., 0., 1.), //#0000ff
    Vec3::new(1., 1., 0.), //#ffff00
    Vec3::new(1., 0., 1.), //#ff00ff
    Vec3::new(0., 1., 1.), //#00ffff
  ];
  let len = colors.len();
  Mipmap {
    images: colors
      .iter()
      .enumerate()
      .map(|(idx, color)| {
        let size = (len - idx).pow(2);
        let mut img = PixImage::new(size as u32, size as u32);
        for c in 0..size {
          for r in 0..size {
            img.set_rgb(c as u32, r as u32, *color);
          }
        }
        Texture::new(img)
      })
      .collect(),
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
  texture: Mipmap,
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

  fn fragment(&self, info: pipeline2::FragmentInfo) -> Fragment {
    let uv = info.barycentric_interpolate(&self.varying_uvs);
    // 计算相对该点上面的一个点的uv坐标
    let top_uv = util::barycentric_interpolate(&self.varying_uvs, info.top_barycentric());
    // 计算相对该点右边一点的uv坐标
    let right_uv = util::barycentric_interpolate(&self.varying_uvs, info.right_barycentry());

    let screen_size = self.screen_size;

    // 计算上/右两个点的uv和该点uv坐标之间的距离，取较大的那个作为近似
    // 为什么要乘以screen_size ?
    // 因为uv坐标是没有单位的量，其范围为[0,1],需要将其影射到[0,screen_size]
    // 才能近似求一个屏幕像素覆盖多大区域的纹理
    let l1 = (((top_uv.x - uv.x) * screen_size).powi(2)
      + (screen_size * (top_uv.y - uv.y)).powi(2))
    .sqrt();
    let l2 = (((right_uv.x - uv.x) * screen_size).powi(2)
      + (screen_size * (right_uv.y - uv.y)).powi(2))
    .sqrt();
    let l = l1.max(l2).log2();

    Fragment::Color(self.texture.get_by_level(
      l.clamp(0., (self.texture.images.len() - 1) as f32),
      uv.x,
      uv.y,
    ))
  }
}

fn main() {
  sdl::one_frame("Mipmap", 600, 600, |mut img| {
    let mipmap = env::args()
      .collect::<Vec<_>>()
      .get(1)
      .map(|f| {
        if f == "color" {
          mipmap_visual()
        } else {
          Mipmap::new(util::load_image("./textures/grid1.tga"))
        }
      })
      .unwrap_or(Mipmap::new(util::load_image("./textures/grid1.tga")));
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
    let invert = mat.invert();
    pipeline2::render(
      &mut img,
      &mut depth_buffer,
      &mut MyShader {
        screen_size: 600.,
        texture: mipmap,
        mat,
        invert,
        varying_uvs: [Vec3::default(), Vec3::default(), Vec3::default()],
        varying_verts: [Vec4::default(), Vec4::default(), Vec4::default()],
      },
      &model,
      0,
    );
    save_image("mipmap.ppm", &img, PPM);
  })
}

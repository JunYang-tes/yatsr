use crate::prelude::*;
pub struct Texture {
  pub image: PixImage,
  lookup: fn(img: &PixImage, u: f32, v: f32) -> Vec3<f32>,
}

pub enum Filter {
  Neareat,
  Bilinear,
}

fn neareat(img: &PixImage, u: f32, v: f32) -> Vec3<f32> {
  let u = u.clamp(0., 1.);
  let v = v.clamp(0., 1.);
  let x = ((img.width() - 1) as f32 * u).round();
  let y = ((img.height() - 1) as f32 * v).round();
  img.get_rgbf(x as u32, y as u32)
}
fn bilinear(img: &PixImage, u: f32, v: f32) -> Vec3<f32> {
  let u = u.clamp(0., 1.);
  let v = v.clamp(0., 1.);
  let x = (img.width() - 1) as f32 * u;
  let y = (img.height() - 1) as f32 * v;
  let c1 = img.get_rgbf(x.floor() as u32, y.floor() as u32);
  let c2 = img.get_rgbf(x.ceil() as u32, y.floor() as u32);
  let c3 = util::linear_interpolation(x - x.floor(), c1, c2);
  let c4 = img.get_rgbf(x.floor() as u32, y.ceil() as u32);
  let c5 = img.get_rgbf(x.ceil() as u32, y.ceil() as u32);
  let c6 = util::linear_interpolation(x - x.floor(), c4, c5);
  util::linear_interpolation(y - y.floor(), c3, c6)
}

impl Texture {
  pub fn new(image: PixImage) -> Texture {
    Texture {
      image,
      lookup: bilinear,
    }
  }
  pub fn neareat(image: PixImage) -> Texture {
    Texture {
      image,
      lookup: neareat,
    }
  }
  pub fn get(&self, u: f32, v: f32) -> Vec3<f32> {
    (self.lookup)(&self.image, u, v)
  }
}

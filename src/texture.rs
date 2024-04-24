use crate::prelude::*;
pub struct Texture {
  image: PixImage,
}
impl Texture {
  pub fn new(image: PixImage) -> Texture {
    Texture { image }
  }
  pub fn get(&self, u: f32, v: f32) -> Vec3<f32> {
    let u = u.clamp(0., 1.);
    let v = v.clamp(0., 1.);
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

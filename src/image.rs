use crate::geometry::{Vec3, Vec4};
pub trait Image {
  fn width(&self) -> u32;
  fn height(&self) -> u32;
  fn image_origin(&self) -> ImageOriginPos {
    ImageOriginPos::LeftBottom
  }
  /**
   * 约定图片原点在左下角
   * */
  fn index(&self, x: u32, y: u32) -> usize {
    let w = self.width() as usize;
    let h = self.height() as usize;
    let x = x as usize;
    let y = y as usize;
    match self.image_origin() {
      ImageOriginPos::LeftBottom => (h - 1 - y) * w + x,
      ImageOriginPos::LeftTop => y * w + x,
    }
  }
  fn blending(&mut self, x: u32, y: u32, color: Vec4<f32>) {
    // https://zh.wikipedia.org/wiki/Alpha%E5%90%88%E6%88%90
    let dst = self.get_rgbaf(x, y);
    let alpha = color.w + (1. - color.w);
    let out = (Vec3::new(color.x, color.y, color.z) * color.w
      + Vec3::new(dst.x, dst.y, dst.z) * (dst.w * (1. - color.w)))
      * (1. / alpha);
    self.set_rgba(x, y, Vec4::new(out.x, out.y, out.z, alpha))
  }
  fn set_rgba(&mut self, x: u32, y: u32, color: Vec4<f32>) {
    let r = (color.x * 255.).clamp(0., 255.) as u8;
    let g = (color.y * 255.).clamp(0., 255.) as u8;
    let b = (color.z * 255.).clamp(0., 255.) as u8;
    // 0 标识完全透明，1 表示完全不透明
    let a = (color.w * 255.).clamp(0., 255.) as u8;
    self.set_rgba32(x, y, Vec4::new(r, g, b, a));
  }
  fn set_rgba32(&mut self, x: u32, y: u32, color: Vec4<u8>);
  fn set_rgb24(&mut self, x: u32, y: u32, color: Vec3<u8>) {
    self.set_rgba32(x, y, Vec4::new(color.x, color.y, color.z, 255))
  }
  fn set_rgb(&mut self, x: u32, y: u32, color: Vec3<f32>) {
    self.set_rgba(x, y, Vec4::new(color.x, color.y, color.z, 1.));
  }
  fn get(&self, x: u32, y: u32) -> Vec3<u8> {
    let rgba = self.get_rgba(x, y);
    Vec3::new(rgba.x, rgba.y, rgba.z)
  }
  fn get_rgbf(&self, x: u32, y: u32) -> Vec3<f32> {
    let rgba = self.get_rgba(x, y);
    Vec3::new(
      rgba.x as f32 / 255.,
      rgba.y as f32 / 255.,
      rgba.z as f32 / 255.,
    )
  }
  fn get_rgba(&self, x: u32, y: u32) -> Vec4<u8>;
  fn get_rgbaf(&self, x: u32, y: u32) -> Vec4<f32> {
    let rgba = self.get_rgba(x, y);
    Vec4::new(
      rgba.x as f32 / 255.,
      rgba.y as f32 / 255.,
      rgba.z as f32 / 255.,
      rgba.w as f32 / 255.,
    )
  }
  fn get_by_normalized_coord(&self, x: f32, y: f32) -> Vec3<u8> {
    self.get(
      (x * ((self.width() - 1) as f32)) as u32,
      (y * ((self.height() - 1) as f32)) as u32,
    )
  }
  fn get_vec3f(&self, x: f32, y: f32) -> Vec3<f32> {
    let color = self.get_by_normalized_coord(x, y);
    Vec3::new(
      (color.x as f32) / 255.,
      (color.y as f32) / 255.,
      (color.z as f32) / 255.,
    )
  }
}
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ImageOriginPos {
  LeftTop,
  LeftBottom,
}
pub struct PixImage {
  pub width: u32,
  pub height: u32,
  pub(crate) data: Vec<u8>,
  origin: ImageOriginPos,
}
impl PixImage {
  pub fn new(width: u32, height: u32) -> PixImage {
    let data: Vec<u8> = vec![0; (width * height * 4) as usize];
    PixImage {
      width,
      height,
      data,
      origin: ImageOriginPos::LeftBottom,
    }
  }
  pub fn from_data(data: Vec<u8>, width: u32, height: u32, origin: ImageOriginPos) -> PixImage {
    PixImage {
      width,
      height,
      data,
      origin,
    }
  }
  pub fn flip_y(&self) -> PixImage {
    let mut img = PixImage::new(self.width(), self.height());
    for row in 0..img.height {
      for col in 0..img.width {
        let color = self.get_rgba(col, row);
        img.set_rgba32(col, img.height - row - 1, color);
      }
    }
    img
  }
}

impl Image for PixImage {
  fn width(&self) -> u32 {
    self.width
  }

  fn height(&self) -> u32 {
    self.height
  }
  fn image_origin(&self) -> ImageOriginPos {
    self.origin
  }

  fn get_rgba(&self, x: u32, y: u32) -> Vec4<u8> {
    let ind = 4 * self.index(x, y);
    let r = self.data[ind];
    let g = self.data[ind + 1];
    let b = self.data[ind + 2];
    let a = self.data[ind + 3];
    Vec4::new(r, g, b, a)
  }

  fn set_rgba32(&mut self, x: u32, y: u32, color: Vec4<u8>) {
    let ind = 4 * self.index(x, y);
    if ind >= self.data.len() {
      return;
    }
    self.data[ind] = color.x;
    self.data[ind + 1] = color.y;
    self.data[ind + 2] = color.z;
    self.data[ind + 3] = color.w;
  }
}
impl Image for (&mut [u8], u32, u32) {
  fn width(&self) -> u32 {
    self.1
  }

  fn height(&self) -> u32 {
    self.2
  }

  fn set_rgba32(&mut self, x: u32, y: u32, color: Vec4<u8>) {
    if x >= self.width() || y >= self.height() {
      return;
    }
    let ind = 4 * self.index(x, y);
    self.0[ind] = color.x;
    self.0[ind + 1] = color.y;
    self.0[ind + 2] = color.z;
    self.0[ind + 3] = color.w;
  }

  fn get_rgba(&self, x: u32, y: u32) -> Vec4<u8> {
    let ind = 4 * self.index(x, y);
    let r = self.0[ind];
    let g = self.0[ind + 1];
    let b = self.0[ind + 2];
    let a = self.0[ind + 3];
    Vec4::new(r, g, b, a)
  }
}

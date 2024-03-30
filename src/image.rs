use crate::geometry::Vec3;
pub trait Image {
  fn width(&self) -> u32;
  fn height(&self) -> u32;
  /**
   * 约定图片原点在左下角
   * */
  fn index(&self, x: u32, y: u32) -> usize {
    let w = self.width() as usize;
    let h = self.height() as usize;
    let x = x as usize;
    let y = y as usize;
    (h - 1 - y) * w + x
  }
  fn set_rgb24(&mut self, x: u32, y: u32, color: Vec3<u8>);
  fn set_rgb(&mut self, x: u32, y: u32, color: Vec3<f32>) {
    let r = (color.x * 255.).clamp(0., 255.) as u8;
    let g = (color.y * 255.).clamp(0., 255.) as u8;
    let b = (color.z * 255.).clamp(0., 255.) as u8;
    self.set_rgb24(x, y, Vec3::new(r, g, b));
  }
  fn get(&self, x: u32, y: u32) -> Vec3<u8>;
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
pub struct PixImage {
  pub width: u32,
  pub height: u32,
  pub(crate) data: Vec<u8>,
}
impl PixImage {
  pub fn new(width: u32, height: u32) -> PixImage {
    let data: Vec<u8> = vec![0; (width * height * 3) as usize];
    PixImage {
      width,
      height,
      data,
    }
  }
  pub fn from_data(data: Vec<u8>, width: u32, height: u32) -> PixImage {
    PixImage {
      width,
      height,
      data,
    }
  }
}
impl Image for PixImage {
  fn width(&self) -> u32 {
    self.width
  }

  fn height(&self) -> u32 {
    self.height
  }

  fn set_rgb24(&mut self, x: u32, y: u32, color: Vec3<u8>) {
    let ind = 3 * self.index(x, y);
    self.data[ind] = color.x;
    self.data[ind + 1] = color.y;
    self.data[ind + 2] = color.z;
  }

  fn get(&self, x: u32, y: u32) -> Vec3<u8> {
    let ind = 3 * self.index(x, y);
    let r = self.data[ind];
    let g = self.data[ind+1];
    let b = self.data[ind+2];
    Vec3::new(r, g, b)
  }
}

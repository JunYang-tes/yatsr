use crate::image::Image;
use std::io::Write;
pub trait Encoder {
  fn encode<I: crate::image::Image>(&self, img: &I) -> Vec<u8>;
}

pub struct PPM;
impl Encoder for PPM {
  fn encode<I: crate::image::Image>(&self, img: &I) -> Vec<u8> {
    let mut ret = Vec::with_capacity((img.width() * img.height()) as usize * 3);
    _ = ret.write(b"P3\n");
    _ = ret.write(format!("{} {}\n255\n", img.width(), img.height()).as_bytes());
    for row in 0..img.height() {
      let row = img.height() - 1 - row;
      for col in 0..img.width() {
        let color = img.get(col, row);
        _ = ret.write(format!("{} {} {} ", color.x, color.y, color.z).as_bytes());
      }
      _ = ret.write(b"\n");
    }
    ret
  }
}


use crate::{image::Image, prelude::Vec3};
#[derive(Default, Debug)]
struct Glyph {
  encoding: u32,
  dwidth: (u8, u8),
  bbx: (u32, u32, i32, i32),
  bitmap: Vec<u8>,
  // 一行含有几个字节
  cols: u8,
}
impl Glyph {
  fn height(&self) -> usize {
    self.bbx.1 as usize
  }
  fn x_offset(&self) -> i32 {
    self.bbx.2
  }
  fn y_offset(&self) -> i32 {
    self.bbx.3
  }
}
pub struct BDF {
  // point-size,x-res,y-res
  // size: (u32, u32, u32),
  // width,height,offset x,offset y
  font_bounding_box: (u32, u32, i32, i32),
  glyphs: std::collections::HashMap<char, Glyph>,
}
impl BDF {
  pub fn draw_text<I: Image>(
    &self,
    img: &mut I,
    x: u32,
    y: u32,
    letter_space: u32,
    color: Vec3<f32>,
    text: &str,
  ) {
    let (bb_width, bb_height, _, _) = self.font_bounding_box;
    let mut y = y;
    let init_x = x;
    for line in text.split('\n') {
      let mut x = init_x;
      for ch in line.chars() {
        if let Some(a) = self.glyphs.get(&ch) {
          draw_bitmap_glypha(img, &a, x, y, color);
          x += a.bbx.0 + letter_space;
        } else {
          x += bb_width;
        }
      }
      if y > bb_height {
        y = y - bb_height;
      } else {
        break;
      }
    }
  }
}
pub mod bdf_parser {
  use std::str::Split;
  pub fn parse(content: &str) -> super::BDF {
    let mut bdf = super::BDF {
      font_bounding_box: (0, 0, 0, 0),
      glyphs: std::collections::HashMap::new(),
    };
    let mut lines = content.split('\n');
    while let Some(line) = lines.next() {
      if line.starts_with("FONTBOUNDINGBOX") {
        let parts = line
          .trim_start_matches("FONTBOUNDINGBOX ")
          .split(' ')
          .collect::<Vec<_>>();
        let w = parts[0].parse::<u32>().unwrap();
        let h = parts[1].parse::<u32>().unwrap();
        let ox = parts[2].parse::<i32>().unwrap();
        let oy = parts[3].parse::<i32>().unwrap();
        bdf.font_bounding_box = (w, h, ox, oy);
      } else if line.starts_with("STARTCHAR") {
        let (ch, glyha) = parse_glyph(&mut lines);
        bdf.glyphs.insert(ch, glyha);
      }
    }
    bdf
  }
  fn parse_glyph(it: &mut Split<char>) -> (char, super::Glyph) {
    let mut ch = ' ';
    let mut glyph = super::Glyph::default();
    while let Some(line) = it.next() {
      if line.starts_with("ENDCHAR") {
        break;
      }
      if line.starts_with("ENCODING") {
        let encoding = line
          .trim_start_matches("ENCODING ")
          .parse::<u32>()
          .unwrap_or(0);
        ch = unsafe { char::from_u32_unchecked(encoding) };
      } else if line.starts_with("DWIDTH") {
        let mut dwidth = line
          .trim_start_matches("DWIDTH ")
          .split(' ')
          .map(|i| i.parse::<u8>().unwrap());
        glyph.dwidth.0 = dwidth.next().unwrap();
        glyph.dwidth.1 = dwidth.next().unwrap();
      } else if line.starts_with("BBX") {
        let parts = line
          .trim_start_matches("BBX ")
          .split(' ')
          .collect::<Vec<_>>();
        let w = parts[0].parse::<u32>().unwrap();
        let h = parts[1].parse::<u32>().unwrap();
        let ox = parts[2].parse::<i32>().unwrap();
        let oy = parts[3].parse::<i32>().unwrap();
        glyph.bbx.0 = w;
        glyph.bbx.1 = h;
        glyph.bbx.2 = ox;
        glyph.bbx.3 = oy;
      } else if line.starts_with("BITMAP") {
        let (col, bitmap) = parse_bitmap(it, glyph.bbx.1);
        glyph.cols = col;
        glyph.bitmap = bitmap
      }
    }
    (ch, glyph)
  }
  fn parse_bitmap(it: &mut Split<char>, lines: u32) -> (u8, Vec<u8>) {
    let mut bitmap = vec![];
    let mut cols: u8 = 1;
    let mut i = 0;
    while i < lines {
      i = i + 1;
      let line = it.next().unwrap();

      let mut bytes = line
        .chars()
        .map(|a| a.to_digit(16).unwrap() as u8)
        .collect::<Vec<_>>();
      for i in (0..bytes.len()).step_by(2) {
        let high = bytes[i];
        let low = bytes[i + 1];
        bitmap.push(high << 4 | low)
      }

      cols = (bytes.len() as u8) / 2;
    }
    (cols, bitmap)
  }
}

fn draw_bitmap_glypha<I: Image>(
  img: &mut I,
  a: &crate::font::Glyph,
  x: u32,
  y: u32,
  color: Vec3<f32>,
) {
  let init_x: i64 = x as i64;
  let mut x = init_x;
  let mut y = (a.y_offset() as i64 + y as i64 + a.height() as i64) as u32;
  for r in 0..a.height() {
    x = init_x + (a.x_offset() as i64);
    for byte in a.bitmap[r * a.cols as usize..(r + 1) * a.cols as usize].iter() {
      for i in 0..8 {
        if (byte >> (7 - i)) & 1 == 1 {
          img.set_rgb(x as u32, y, color)
        } else {
        }
        x = x + 1;
      }
    }
    if y > 0 {
      y = y - 1;
    } else {
      break;
    }
  }
}
pub fn get_cal_lite() -> BDF {
  bdf_parser::parse(include_str!("./CalLite24.bdf"))
}

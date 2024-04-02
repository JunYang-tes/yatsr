use crate::image::{Image, ImageOriginPos, PixImage};

pub trait Decoder {
  fn decode(&self, data: Vec<u8>) -> PixImage;
}

pub struct TGA;
#[derive(Debug)]
struct TAGHeader {
  image_id_len: u8,
  has_color_map: bool,
  image_type: TAGImageType,
  image_specification: TAGImageSpecification,
}
#[repr(u8)]
#[derive(Debug)]
enum TAGImageType {
  NoImageData = 0,
  UncompressedColorMapped = 1,
  UncompressedTrueColor = 2,
  UncompressedBlackWhite = 3,
  RunLenEncodedColorMapped = 9,
  RunLenEncodedTrueColor = 10,
  RunLenEncodedBlackWhite = 11,
}
#[derive(Debug)]
struct TAGImageSpecification {
  origin: (i16, i16),
  width: u16,
  height: u16,
  depth: u8,
  descriptor: TAGImageDescriptor,
}
fn decode_image_specification(data: &[u8]) -> TAGImageSpecification {
  let x = i16::from_le_bytes([data[0], data[1]]);
  let y = i16::from_le_bytes([data[2], data[3]]);
  let width = u16::from_le_bytes([data[4], data[5]]);
  let height = u16::from_le_bytes([data[6], data[7]]);
  let depth = data[8];
  TAGImageSpecification {
    origin: (x, y),
    width,
    height,
    depth,
    descriptor: TAGImageDescriptor::new(data[9]),
  }
}
#[derive(Debug)]
struct TAGImageDescriptor {
  attribute_count: u8,
  screen_destination: ScreenDestination,
}
impl TAGImageDescriptor {
  fn new(v: u8) -> TAGImageDescriptor {
    TAGImageDescriptor {
      attribute_count: 0,
      screen_destination: to_screen_destination(v),
    }
  }
}
#[derive(Debug)]
enum ScreenDestination {
  BottomLeft,
  BottomRight,
  TopLeft,
  TopRight,
}
fn to_screen_destination(v: u8) -> ScreenDestination {
  let b5 = v >> 5 & 1;
  let b4 = v >> 4 & 1;
  match (b5, b4) {
    (0, 0) => ScreenDestination::BottomLeft,
    (0, 1) => ScreenDestination::BottomRight,
    (1, 0) => ScreenDestination::TopLeft,
    (1, 1) => ScreenDestination::TopRight,
    _ => {
      panic!("What the hell")
    }
  }
}
fn screen_destination_to_img_origin(d: &ScreenDestination) -> ImageOriginPos {
  match d {
    ScreenDestination::TopLeft => ImageOriginPos::LeftTop,
    ScreenDestination::BottomLeft => ImageOriginPos::LeftBottom,
    _ => {
      panic!("Unsupported screen destination")
    }
  }
}

fn to_image_type(t: u8) -> TAGImageType {
  match t {
    0 => TAGImageType::NoImageData,
    1 => TAGImageType::UncompressedColorMapped,
    2 => TAGImageType::UncompressedTrueColor,
    3 => TAGImageType::UncompressedBlackWhite,
    9 => TAGImageType::RunLenEncodedColorMapped,
    10 => TAGImageType::RunLenEncodedTrueColor,
    11 => TAGImageType::RunLenEncodedBlackWhite,
    _ => panic!("Unknow image type:{}", t),
  }
}
#[test]
fn image_type() {
  assert!(to_image_type(0) as u8 == 0);
  assert!(to_image_type(1) as u8 == 1);
}
fn decode_color_map_specification(data: &[u8]) {}

#[derive(Debug)]
struct Index(usize);
impl Index {
  fn get_then_move(&mut self) -> usize {
    let c = self.0;
    self.0 += 1;
    c
  }
  fn get_range_and_move(&mut self, size: usize) -> std::ops::Range<usize> {
    let r = std::ops::Range {
      start: self.0,
      end: self.0 + size,
    };
    self.0 = r.end;
    r
  }
  fn skip(&mut self, size: usize) -> &mut Self {
    self.0 += size;
    self
  }
}
fn decode_rle_true_color(header: &TAGHeader, data: Vec<u8>, index: &mut Index) -> PixImage {
  let TAGImageSpecification {
    origin: _,
    width,
    height,
    depth,
    descriptor: _,
  } = &header.image_specification;
  if *depth != 24 && *depth != 32 {
    println!("depth:{}", *depth);
    unimplemented!("other depth than 24/32 is not implemented");
  }
  let mut ret = Vec::with_capacity(
    header.image_specification.width as usize * header.image_specification.height as usize * 3,
  );
  let mut decoded = 0;
  let total = (*width) as usize * (*height) as usize;
  enum ElementType {
    RLPacket,
    RawPacket,
  }
  while decoded < total {
    let mut repetition = data[index.get_then_move()];
    let ele_type = if repetition >> 7 == 1 {
      ElementType::RLPacket
    } else {
      ElementType::RawPacket
    };
    repetition = (repetition & 0b0111_1111) + 1;
    match ele_type {
      ElementType::RLPacket => {
        let b = data[index.get_then_move()];
        let g = data[index.get_then_move()];
        let r = data[index.get_then_move()];
        let a = if *depth == 32 {
          data[index.get_then_move()]
        } else {
          255
        };
        for _ in 0..repetition {
          ret.push(r);
          ret.push(g);
          ret.push(b);
          ret.push(a);
        }
        decoded += repetition as usize;
      }
      ElementType::RawPacket => {
        for _ in 0..repetition {
          let b = data[index.get_then_move()];
          let g = data[index.get_then_move()];
          let r = data[index.get_then_move()];
          let a = if *depth == 32 {
            data[index.get_then_move()]
          } else {
            255
          };
          ret.push(r);
          ret.push(g);
          ret.push(b);
          ret.push(a);
        }
        decoded += repetition as usize;
      }
    }
  }
  PixImage::from_data(
    ret,
    (*width) as u32,
    (*height) as u32,
    screen_destination_to_img_origin(&header.image_specification.descriptor.screen_destination),
  )
}
fn decode_rle_black_white(header: &TAGHeader, data: Vec<u8>, index: &mut Index) -> PixImage {
  let TAGImageSpecification {
    origin: _,
    width,
    height,
    depth,
    descriptor: _,
  } = &header.image_specification;
  if *depth != 8 {
    println!("depth:{}", *depth);
    unimplemented!("other depth than 8 is not implemented");
  }
  let mut ret = Vec::with_capacity(
    header.image_specification.width as usize * header.image_specification.height as usize * 4,
  );
  let mut decoded = 0;
  let total = (*width) as usize * (*height) as usize;
  enum ElementType {
    RLPacket,
    RawPacket,
  }
  while decoded < total {
    let mut repetition = data[index.get_then_move()];
    let ele_type = if repetition >> 7 == 1 {
      ElementType::RLPacket
    } else {
      ElementType::RawPacket
    };
    repetition = (repetition & 0b0111_1111) + 1;
    match ele_type {
      ElementType::RLPacket => {
        let r = data[index.get_then_move()];
        for _ in 0..repetition {
          ret.push(r);
          ret.push(r);
          ret.push(r);
          ret.push(255);
        }
        decoded += repetition as usize;
      }
      ElementType::RawPacket => {
        for _ in 0..repetition {
          let r = data[index.get_then_move()];
          ret.push(r);
          ret.push(r);
          ret.push(r);
          ret.push(255);
        }
        decoded += repetition as usize;
      }
    }
  }
  PixImage::from_data(
    ret,
    (*width) as u32,
    (*height) as u32,
    screen_destination_to_img_origin(&header.image_specification.descriptor.screen_destination),
  )
}
fn read_uncompressed_data(header: &TAGHeader, data: Vec<u8>, index: &mut Index) -> PixImage {
  let depth = header.image_specification.depth;
  if depth != 24 && depth != 32 {
    panic!("Unsupported depth:{}", depth);
  }

  let mut ret = Vec::with_capacity(
    (header.image_specification.width as usize) * (header.image_specification.height as usize) * 4,
  );
  for _ in 0..(header.image_specification.width as u32) * (header.image_specification.height as u32)
  {
    let b = data[index.get_then_move()];
    let g = data[index.get_then_move()];
    let r = data[index.get_then_move()];
    let a = if depth == 32 {
      data[index.get_then_move()]
    } else {
      255
    };

    ret.push(r);
    ret.push(g);
    ret.push(b);
    ret.push(a);
  }
  PixImage::from_data(
    ret,
    header.image_specification.width as u32,
    header.image_specification.height as u32,
    screen_destination_to_img_origin(&header.image_specification.descriptor.screen_destination),
  )
}
// Note：不完整的tga解码实现
impl Decoder for TGA {
  fn decode(&self, data: Vec<u8>) -> PixImage {
    let mut index = Index(0);

    let header = TAGHeader {
      image_id_len: data[index.get_then_move()],
      has_color_map: data[index.get_then_move()] == 1,
      image_type: to_image_type(data[index.get_then_move()]),
      // decode_color_map_specification(&data[3..8]);
      image_specification: decode_image_specification(&data[index.skip(5).get_range_and_move(10)]),
    };
    index.skip(header.image_id_len as usize); // skip image id data
    if header.has_color_map {
      todo!("Color map is not implemented")
    }
    match header.image_type {
      TAGImageType::RunLenEncodedTrueColor => decode_rle_true_color(&header, data, &mut index),
      TAGImageType::RunLenEncodedBlackWhite => decode_rle_black_white(&header, data, &mut index),
      TAGImageType::UncompressedTrueColor => read_uncompressed_data(&header, data, &mut index),
      _ => {
        println!("{:?}", header);
        unimplemented!("unimplemented to read {:?}", header.image_type)
      }
    }
  }
}

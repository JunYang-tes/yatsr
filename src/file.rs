use std::path::Path;

use crate::image::Image;
use crate::image_encoder::Encoder;
pub fn save_image<P: AsRef<Path>, E: Encoder, I: crate::image::Image>(
  path: P,
  img: &I,
  encoder: E,
) -> std::io::Result<()> {
  let encoded = encoder.encode(img);
  std::fs::write(path, encoded)
}


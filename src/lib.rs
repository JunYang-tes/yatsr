pub mod file;
pub mod geometry;
pub mod image;
pub mod image_decoder;
pub mod image_encoder;
pub mod mat;
pub mod model;
pub mod pipeline;
pub mod transform;
pub mod font;
pub mod sdl;
pub mod util;
pub mod prelude {
  pub use crate::file::save_image;
  pub use crate::geometry::{Vec3, Vec4};
  pub use crate::image::{Image, PixImage};
  pub use crate::image_encoder::{Encoder, PPM};
  pub use crate::pipeline::{render, Fragment, Shader};
  pub use crate::model::*;
  pub use crate::mat::*;
  pub use crate::transform::Transform;
  pub mod transform {
      pub use crate::transform::*;
  }
  pub use crate::font::get_cal_lite;
  pub use crate::shaders;
  pub use crate::sdl;
  pub use crate::util;
  pub use crate::model::*;
  pub use crate::shape;
}
pub mod shape;
pub mod shaders;

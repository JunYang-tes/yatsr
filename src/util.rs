use crate::prelude::{Vec3, PixImage, Image};

pub fn load_image<P: AsRef<std::path::Path>>(p: P) -> crate::image::PixImage {
  use crate::image_decoder::*;
  let tga = TGA;
  tga.decode(std::fs::read(p).unwrap())
}

pub fn linear_interpolation<S, T>(t: S, a: T, b: T) -> T
where
  T: Copy + std::ops::Sub<Output = T> + std::ops::Mul<S, Output = T> + std::ops::Add<Output = T>,
{
  a + (b - a) * t
}

/*
 * 计算射线和平面点交点
 * */
pub fn intersect_of_plan_line(
  p: Vec3<f32>,  //平面上一点
  n: Vec3<f32>,  //平面法向量
  p0: Vec3<f32>, // 射线顶点
  d: Vec3<f32>,  // 射线方向
) -> Option<Vec3<f32>> {
  let t = (p * n - p0 * n) * (1. / (d * n));
  if t > 0. {
    Some(p0 + (d * t))
  } else {
    None
  }
}

pub fn sub_img(img: &PixImage, x: f32, y: f32, w: f32, h: f32) -> PixImage {
  let mut sub = PixImage::new(
    (w * img.width() as f32) as u32,
    (h * img.height() as f32) as u32,
  );
  let x = (x * img.width() as f32) as u32;
  let y = (y * img.height() as f32) as u32;
  for c in 0..sub.width() {
    for r in 0..sub.height() {
      sub.set_rgba32(c, r, img.get_rgba(x + c, y + r))
    }
  }
  sub
}

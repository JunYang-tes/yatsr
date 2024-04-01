use std::env;

use yatsr::{
  file::save_image,
  geometry::Vec3,
  image::{PixImage, *},
  image_encoder::PPM,
  model::Model,
};
type Point = Vec3<f32>;

pub fn barycentric(
  a: Vec3<f32>,
  b: Vec3<f32>,
  c: Vec3<f32>,
  x: f32,
  y: f32,
  //p: Vec3<f32>,
) -> (f32, f32, f32) {
  let alpha = ((b.x - x) * (c.y - b.y) + (y - b.y) * (c.x - b.x))
    / ((b.x - a.x) * (c.y - b.y) + (a.y - b.y) * (c.x - b.x));
  let beta = ((c.x - x) * (a.y - c.y) + (y - c.y) * (a.x - c.x))
    / ((c.x - b.x) * (a.y - c.y) + (b.y - c.y) * (a.x - c.x));
  (alpha, beta, 1. - alpha - beta)
}

fn draw_triangle(
  img: &mut PixImage,
  depth_buff: &mut Vec<f32>,
  a: Point,
  b: Point,
  c: Point,
  color: Vec3<f32>,
) {
  let min_x = a.x.min(b.x).min(c.x) as u32;
  let max_x = a.x.max(b.x).max(c.x) as u32;
  let min_y = a.y.min(b.y).min(c.y) as u32;
  let max_y = a.y.max(b.y).max(c.y) as u32;
  for y in min_y..=max_y {
    for x in min_x..=max_x {
      let (alpha, beta, gamma) = barycentric(a, b, c, x as f32, y as f32);
      if alpha < 0. || beta < 0. || gamma < 0. {
        continue;
      }
      let p = a * alpha + b * beta + c * gamma;
      let index = (y * img.height() + x) as usize;
      if p.z > depth_buff[index] {
        depth_buff[index] = p.z;
        img.set_rgb(x, y, color);
      }
    }
  }
}
fn world2screen(p: Point, w: f32, h: f32) -> Point {
  let p = p + Vec3::new(1., 1., 1.); // [-1,1] => [0,2]
  let mut p = p * 0.5; // [0,2] => [0,1]
  p.x *= w;
  p.y *= h;
  p
}

fn main() {
  for argument in env::args() {
    println!("{argument}");
  }
  let model_path = env::args()
    .collect::<Vec<_>>()
    .get(1)
    .map(|f| f.clone())
    .unwrap_or(String::from("./models/girl/D0901D64.obj"));
  let mut model = Model::from_file(model_path).expect("Failed to load model:,");
  model.normalize_verts();
  let width = 500;
  let height = 500;
  let mut image = PixImage::new(500, 500);
  let mut depth_buff = vec![f32::MIN; width * height];
  let light = Vec3::new(1., 1., 1.).normalize();
  let color = Vec3::new(0.8, 0.8, 0.8);
  for n in 0..model.face_count() {
    let verts = model.verts_of_face(n);
    let normal = (verts[1] - verts[0])
      .cross_product(verts[2] - verts[0])
      .normalize();
    let intensity = (normal * light).max(0.);
    draw_triangle(
      &mut image,
      &mut depth_buff,
      // 模型顶点坐标范围为[-1,1],将其映射到[0,500]
      world2screen(verts[0], width as f32, height as f32),
      world2screen(verts[1], width as f32, height as f32),
      world2screen(verts[2], width as f32, height as f32),
      color * intensity,
    )
  }
  save_image("basic_z_buffer.ppm", &image, PPM).expect("Failed to save image")
}

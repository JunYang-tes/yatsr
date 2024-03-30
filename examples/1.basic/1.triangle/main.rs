use yatsr::file::save_image;
use yatsr::geometry::Vec2;
use yatsr::geometry::Vec3;
use yatsr::image::Image;
use yatsr::image::PixImage;
use yatsr::image_encoder::PPM;

type Point = Vec2<f32>;
pub fn barycentric(a: Point, b: Point, c: Point, x: f32, y: f32) -> (f32, f32, f32) {
  let alpha = ((b.x - x) * (c.y - b.y) + (y - b.y) * (c.x - b.x))
    / ((b.x - a.x) * (c.y - b.y) + (a.y - b.y) * (c.x - b.x));
  let beta = ((c.x - x) * (a.y - c.y) + (y - c.y) * (a.x - c.x))
    / ((c.x - b.x) * (a.y - c.y) + (b.y - c.y) * (a.x - c.x));
  (alpha, beta, 1. - alpha - beta)
}

fn draw_triangle(img: &mut PixImage, a: Point, b: Point, c: Point) {
  let min_x = a.x.min(b.x).min(c.x) as u32;
  let max_x = a.x.max(b.x).max(c.x) as u32;
  let min_y = a.y.min(b.y).min(c.y) as u32;
  let max_y = a.y.max(b.y).max(c.y) as u32;
  for y in min_y..=max_y {
    for x in min_x..=max_x {
      let (alpha, beta, gamma) = barycentric(a, b, c, x as f32, y  as f32);
      if alpha < 0. || beta < 0. || gamma < 0. {
        continue;
      }
      img.set_rgb24(x, y, Vec3::new(255, 0, 0));
    }
  }
}

fn main() {
  let mut image = PixImage::new(500, 500);
  draw_triangle(
    &mut image,
    Vec2::new(100., 100.),
    Vec2::new(400., 200.),
    Vec2::new(120., 400.),
  );
  save_image("basic_triangle.ppm", &image, PPM).expect("Failed to save image");
}

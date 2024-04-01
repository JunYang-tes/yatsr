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

fn color_interpolation(
  img: &mut PixImage,
  a: Point,
  a_color: Vec3<f32>,
  b: Point,
  b_color: Vec3<f32>,
  c: Point,
  c_color: Vec3<f32>,
) {
  let min_x = a.x.min(b.x).min(c.x) as u32;
  let max_x = a.x.max(b.x).max(c.x) as u32;
  let min_y = a.y.min(b.y).min(c.y) as u32;
  let max_y = a.y.max(b.y).max(c.y) as u32;
  let sub_pix_offset = [(-0.25, -0.25), (0.75, 0.75), (-0.25, 0.75), (0.25, -0.75)];
  for y in min_y..=max_y {
    for x in min_x..=max_x {
      let mut cnt = 0;
      let mut color = Vec3::new(0., 0., 0.);
      for (dx, dy) in sub_pix_offset {
        let (alpha, beta, gamma) = barycentric(a, b, c, (x as f32) + dx, (y as f32) + dy);
        if alpha < 0. || beta < 0. || gamma < 0. {
          continue;
        }
        cnt += 1;
        color = color + (a_color * alpha + b_color * beta + c_color * gamma);
      }
      if cnt > 0 {
        img.set_rgb(x, y, color * (1. / (cnt as f32)));
      }
    }
  }
}

fn main() {
  let mut image = PixImage::new(500, 500);
  color_interpolation(
    &mut image,
    Vec2::new(100., 100.),
    Vec3::new(1., 0., 0.), //Red
    Vec2::new(400., 200.),
    Vec3::new(0., 1., 0.), //Green
    Vec2::new(120., 400.),
    Vec3::new(0., 0., 1.), // blue
  );
  save_image("super-sampling.ppm", &image, PPM).expect("Failed to save image");
}

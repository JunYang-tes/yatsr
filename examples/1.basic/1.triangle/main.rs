use yatsr::file::save_image;
use yatsr::geometry::Vec2;
use yatsr::geometry::Vec3;
use yatsr::image::Image;
use yatsr::image::PixImage;
use yatsr::image_encoder::PPM;

type Point = Vec2<f32>;

// 重心坐标要点：
// 1. P = alpha * A + beta * B + gamma * C
//    alpha + beta + gamma = 1
//    其中，A,B,C 是三角形的三个顶点，P是三角形同一平面上的点
// 2. 重心坐标可以用来判断P点是否在三角形内部：alpha>=0 && beta>=0  && gamma >= 0
// 3. 重心坐标可以用来对三个定点的属性（颜色、法向量等）做插值，即利用三个顶点的属性
//    来求三角行内每点的属性
pub fn barycentric(a: Point, b: Point, c: Point, x: f32, y: f32) -> (f32, f32, f32) {
  let alpha = ((b.x - x) * (c.y - b.y) + (y - b.y) * (c.x - b.x))
    / ((b.x - a.x) * (c.y - b.y) + (a.y - b.y) * (c.x - b.x));
  let beta = ((c.x - x) * (a.y - c.y) + (y - c.y) * (a.x - c.x))
    / ((c.x - b.x) * (a.y - c.y) + (b.y - c.y) * (a.x - c.x));
  (alpha, beta, 1. - alpha - beta)
}

// 遍历三角形的包围盒，对每点判断改点是否在三角形内部
// 对于在三角形内部的点，则在图片上对应的地方画上一个点。
fn draw_triangle(img: &mut PixImage, a: Point, b: Point, c: Point) {
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
      img.set_rgb24(x, y, Vec3::new(0, 0, 255));
    }
  }
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
  for y in min_y..=max_y {
    for x in min_x..=max_x {
      let (alpha, beta, gamma) = barycentric(a, b, c, x as f32, y as f32);
      if alpha < 0. || beta < 0. || gamma < 0. {
        continue;
      }
      let color = a_color * alpha + b_color * beta + c_color * gamma;
      img.set_rgb(x, y, color);
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
  save_image("colorfull_triangle.ppm", &image, PPM).expect("Failed to save image");
}

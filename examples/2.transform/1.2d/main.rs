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
      let (alpha, beta, gamma) = barycentric(a, b, c, x as f32, y as f32);
      if alpha < 0. || beta < 0. || gamma < 0. {
        continue;
      }
      img.set_rgb24(x, y, Vec3::new(0, 0, 255));
    }
  }
}

fn draw_rect(img: &mut PixImage, x: u32, y: u32, w: u32, h: u32, transform: fn(p: Point) -> Point) {
  // 变换四边形的4个顶点，再用变化后的点来绘制四边形
  let p1 = transform(Vec2::new(x as f32, y as f32));
  let p2 = transform(Vec2::new((x + w) as f32, y as f32));
  let p3 = transform(Vec2::new((x + w) as f32, (y + h) as f32));
  let p4 = transform(Vec2::new(x as f32, (y + h) as f32));
  draw_triangle(img, p1, p2, p3);
  draw_triangle(img, p3, p4, p1);
}

fn grid(img: &mut PixImage, w: u32, h: u32, step: usize) {
  for x in (0..w).step_by(step) {
    for y in 0..h {
      img.set_rgb(x, y, Vec3::new(1., 1., 1.))
    }
  }
  for y in (0..h).step_by(step) {
    for x in 0..w {
      img.set_rgb(x, y, Vec3::new(1., 1., 1.))
    }
  }
}

fn main() {
  let mut img = PixImage::new(100, 100);
  draw_rect(&mut img, 0, 0, 40, 40, |p| Vec2::new(p.x, p.y));
  grid(&mut img, 100, 100, 10);
  save_image("./origin.ppm", &img, PPM).unwrap();

  let mut img = PixImage::new(100, 100);
  draw_rect(&mut img, 0, 0, 40, 40, |p| Vec2::new(p.x + p.y, p.x));
  grid(&mut img, 100, 100, 10);
  save_image("./shear.ppm", &img, PPM).unwrap();

  let mut img = PixImage::new(100, 100);
  draw_rect(&mut img, 0, 0, 40, 40, |p| Vec2::new(p.x * 2., p.y * 2.));
  grid(&mut img, 100, 100, 10);
  save_image("./scale.ppm", &img, PPM).unwrap();

  let mut img = PixImage::new(100, 100);
  draw_rect(&mut img, 0, 0, 40, 40, |p| {
    // 以坐标系原点为旋转中心来将该正方形逆时针旋转45度
    // 利用三角函数,可以求出下面的公式
    // x = p.x * cos + p.y * (-sin)
    // y = p.x * sin + p.y * cos

    let angle: f32 = 45. * 3.14 / 180.;
    Vec2::new(
      p.x * angle.cos() - angle.sin() * p.y,
      p.x * angle.sin() + angle.cos() * p.y,
    )
  });
  grid(&mut img, 100, 100, 10);
  save_image("./rotation.ppm", &img, PPM).unwrap();

  let mut img = PixImage::new(100, 100);
  // 以正方行中心为原点进行旋转，可以分解为三步
  // 1. 将正方形中心移动到原点
  // 2. 旋转
  // 3. 将正方形的中点移动到原来的位置
  // 可见以函数的方式来表达变换难以组合，当然更难以从一个组合的变换求出
  // 其逆变换
  draw_rect(&mut img, 0, 0, 40, 40, |p| {
    let translate = |dx: f32, dy: f32, p: Point| Vec2::new(p.x + dx, p.y + dy);
    let rotate = |p: Point| {
      let angle: f32 = 45. * 3.14 / 180.;
      Vec2::new(
        p.x * angle.cos() - angle.sin() * p.y,
        p.x * angle.sin() + angle.cos() * p.y,
      )
    };
    let p = translate(-20., -20., p);
    let p = rotate(p);
    let p = translate(20., 20., p);
    p
  });
  grid(&mut img, 100, 100, 10);
  save_image("./rotation-translate.ppm", &img, PPM).unwrap();
}

use yatsr::{file::save_image, geometry::{Vec2, Vec3}, image::{Image, PixImage}, image_encoder::PPM};

struct Mat3 {
  data: [[f32; 3]; 3],
}
fn dot(a:[f32;3],b:[f32;3])->f32 {
    let [x,y,z] = a;
    let [i,j,k] = b;
    x*i+y*j+z*k
}
impl Mat3 {
  fn row(&self,i:usize) ->[f32;3] {
    self.data[i]
  }
  fn col(&self,i:usize) -> [f32;3] {
      let mut arr=[0.;3];
      arr[0]= self.data[0][i];
      arr[1]= self.data[1][i];
      arr[2]= self.data[2][i];
      arr
  }
  fn mul(&self,other:&Mat3)->Mat3 {
    Mat3 {
        data:[[dot(self.row(0),other.col(0)),dot(self.row(0),other.col(1)),dot(self.row(0),other.col(2))],
              [dot(self.row(1),other.col(0)),dot(self.row(1),other.col(1)),dot(self.row(1),other.col(2))],
              [dot(self.row(2),other.col(0)),dot(self.row(2),other.col(1)),dot(self.row(2),other.col(2))]]
    }
  }
  fn identity() -> Mat3 {
    //#[rustfmt::skip]
    Mat3 {
        data:[[1.,0.,0.],
              [0.,1.,0.],
              [0.,0.,1.]]
    }
  }
  fn shear(sx:f32,sy:f32)->Mat3 {

    Mat3 {
        data:[[1.,sx,0.],
              [sy,1.,0.],
              [0.,0.,1.]]
    }
  }
  fn translate(dx: f32, dy: f32) -> Mat3 {
    //#[rustfmt::skip]
    Mat3 {
        data:[[1.,0.,dx ],
              [0.,1.,dy ],
              [0.,0., 1.]]
    }
  }
  fn scale(sx: f32, sy: f32) -> Mat3 {
    //#[rustfmt::skip]
    Mat3 {
        data:[[sx,0.,0.],
              [0.,sy,0.],
              [0.,0.,1.]]
    }
  }
  fn rotate(angle:f32)->Mat3 {
      Mat3 {
        data:[[angle.cos(),-angle.sin(),0.],
              [angle.sin(),angle.cos(),0.],
              [0.,0.,1.]]

      }
  }
}

struct Transform {
    mat:Mat3
}
impl Transform {
    fn new()->Transform {
        Transform {
            mat:Mat3::identity()
        }
    }
    fn shear(mut self,sx:f32,sy:f32)->Self {
        self.mat = Mat3::shear(sx,sy).mul(&self.mat);
        self
    }
    fn scale(mut self,sx:f32,sy:f32)->Self {
        self.mat = Mat3::scale(sx,sy).mul(&self.mat);
        self
    }
    fn translate(mut self,dx:f32,dy:f32)->Self {
        self.mat = Mat3::translate(dx,dy).mul(&self.mat);
        self
    }
    fn rotate(mut self,angle:f32)->Self {
        self.mat = Mat3::rotate(angle).mul(&self.mat);
        self
    }
    fn rotate_at(self,angle:f32,point:Vec2<f32>)->Self {
        self.translate(-point.x/2.,-point.y/2.)
            .rotate(angle)
            .translate(point.x/2.,-point.y/2.)
    }
    fn apply(&self,point:Vec2<f32>)->Vec2<f32> {
        let p = [point.x,point.y,1.];
        let x = dot(self.mat.row(0),p);
        let y = dot(self.mat.row(1),p);
        Vec2::new(x,y)
    }
}

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

fn draw_rect(img: &mut PixImage, x: u32, y: u32, w: u32, h: u32, transform: Transform) {
  // 变换四边形的4个顶点，再用变化后的点来绘制四边形
  let p1 = transform.apply(Vec2::new(x as f32, y as f32));
  let p2 = transform.apply(Vec2::new((x + w) as f32, y as f32));
  let p3 = transform.apply(Vec2::new((x + w) as f32, (y + h) as f32));
  let p4 = transform.apply(Vec2::new(x as f32, (y + h) as f32));
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
  draw_rect(&mut img, 0, 0, 40, 40, Transform::new());
  grid(&mut img, 100, 100, 10);
  save_image("./origin.ppm", &img, PPM).unwrap();

  let mut img = PixImage::new(100, 100);
  draw_rect(&mut img, 0, 0, 40, 40, Transform::new().shear(1.,0.));
  grid(&mut img, 100, 100, 10);
  save_image("./shear.ppm", &img, PPM).unwrap();


  let mut img = PixImage::new(100, 100);
  draw_rect(&mut img, 0, 0, 40, 40, Transform::new().scale(1.5,1.5));
  grid(&mut img, 100, 100, 10);
  save_image("./scale.ppm", &img, PPM).unwrap();


  let mut img = PixImage::new(100, 100);
  draw_rect(&mut img, 0, 0, 40, 40, Transform::new().rotate(45. * 3.14/180.));
  grid(&mut img, 100, 100, 10);
  save_image("./rotation-translate.ppm", &img, PPM).unwrap();

  let mut img = PixImage::new(100, 100);
  draw_rect(&mut img, 0, 0, 40, 40, Transform::new().rotate_at(45. * 3.14/180.,Vec2::new(20.,20.)));
  grid(&mut img, 100, 100, 10);
  save_image("./rotation-translate.ppm", &img, PPM).unwrap();


}

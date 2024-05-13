use std::env;

use yatsr::prelude::*;
struct Cubemap {
  pub texture: [Texture; 6],
}
impl Cubemap {
  fn new(image: &PixImage, faces: [(f32, f32, f32, f32); 6]) -> Cubemap {
    let c = Cubemap {
      texture: faces.map(|(x, y, w, h)| Texture::new(sub_img(image, x, y, w, h))),
    };
    c
  }
  fn colored() -> Cubemap {
    Cubemap {
      texture: [
        //#ff0000
        Texture::neareat(gen_img(
          Vec3::new(1., 0., 0.),
          Vec3::new(0., 0., 0.),
          "Front",
        )),
        //#00ff00
        Texture::neareat(gen_img(
          Vec3::new(0., 1., 0.),
          Vec3::new(0., 0., 0.),
          "Back",
        )),
        //left #0000ff
        Texture::neareat(gen_img(
          Vec3::new(0., 0., 1.),
          Vec3::new(0., 0., 0.),
          "Left",
        )),
        // right #ff00ff
        Texture::neareat(gen_img(
          Vec3::new(1., 0., 1.),
          Vec3::new(0., 0., 0.),
          "Right",
        )),
        //top #ffff00
        Texture::neareat(gen_img(Vec3::new(1., 1., 0.), Vec3::new(0., 0., 0.), "Top")),
        // bottom #00ffff
        Texture::neareat(gen_img(
          Vec3::new(0., 1., 1.),
          Vec3::new(0., 0., 0.),
          "Bottom",
        )),
      ],
    }
  }
  fn get(&self, point: Vec3<f32>) -> Vec3<f32> {
    if point.x.abs() > point.y.abs() && point.x.abs() > point.z.abs() {
      // left or right
      let x = point.z / point.x.abs();
      let y = point.y / point.x.abs();
      if (point.x > 0.) {
        // 右
        return self.texture[3].get((-x + 1.) / 2., (y + 1.) / 2.);
      }

      // 左
      return self.texture[2].get((x + 1.) / 2., (y + 1.) / 2.);
    }
    if point.z.abs() > point.x.abs() && point.z.abs() > point.y.abs() {
      let x = (point.x / point.z.abs() + 1.) / 2.;
      let y = (point.y / point.z.abs() + 1.) / 2.;
      if point.z > 0. {
        // 前
        return self.texture[0].get(x, y);
      }
      // 后
      let x = (-point.x / point.z.abs() + 1.) / 2.;
      let y = (point.y / point.z.abs() + 1.) / 2.;
      return self.texture[1].get(x, y);
    }
    let u = point.x / point.y.abs();
    let v = point.z / point.y.abs();
    if point.y > 0. {
      // 上
      return self.texture[4].get((-u + 1.) / 2., (v + 1.) / 2.);
    }

    // 下
    return self.texture[5].get((u + 1.) / 2., (v + 1.) / 2.);

    // 通过求交点的方式求投影点
    // let cube = [
    //   // 由一个点和一个法向量定义平面
    //   (Vec3::new(0., 0., 1.), Vec3::new(0., 0., 1.)), // front
    //   (Vec3::new(-1., -1., -1.), Vec3::new(0., 0., -1.)), // back
    //   (Vec3::new(-1., -1., 1.), Vec3::new(-1., 0., 0.)), // left
    //   (Vec3::new(1., -1., 1.), Vec3::new(1., 0., 0.)), // right
    //   (Vec3::new(1., 1., 1.), Vec3::new(0., 1., 0.)), // top
    //   (Vec3::new(1., -1., 1.), Vec3::new(0., -1., 0.)), // bottom
    // ];
    // for (ind, (p, normal)) in cube.iter().enumerate() {
    //   let p = get_intersect(
    //     *p,
    //     *normal,
    //     Vec3::new(0., 0., 0.),
    //     point, // point - Vec3::new(0.,0.,0.)
    //   );
    //   if let Some(p) = p {
    //     if p.x >= -1. && p.x <= 1. && p.y >= -1. && p.y <= 1. && p.z >= -1. && p.z <= 1. {
    //       return match ind {
    //         0 => self.texture[ind].get((p.x + 1.) / 2., (p.y + 1.) / 2.),
    //         1 => self.texture[ind].get((-p.x + 1.) / 2., (p.y + 1.) / 2.),
    //         2 => self.texture[ind].get((p.z + 1.) / 2., (p.y + 1.) / 2.),
    //         // right
    //         3 => self.texture[ind].get((-p.z + 1.) / 2., (p.y + 1.) / 2.),
    //         // top
    //         4 => self.texture[ind].get((-p.x + 1.) / 2., (p.z + 1.) / 2.),
    //         _ => self.texture[ind].get((p.x + 1.) / 2., (p.z + 1.) / 2.),
    //       };
    //     }
    //   }
    // }
    // return Vec3::new(1., 1., 1.);
  }
}
fn sub_img(img: &PixImage, x: f32, y: f32, w: f32, h: f32) -> PixImage {
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
fn gen_img(color: Vec3<f32>, text_color: Vec3<f32>, text: &str) -> PixImage {
  let mut img = PixImage::new(300, 300);
  for r in 0..img.width() {
    for c in 0..img.height() {
      img.set_rgb(c, r, color);
    }
  }
  let font = get_cal_lite();
  font.draw_text(&mut img, 10, 10, 4, text_color, text);
  return img;
}

fn get_intersect(p: Vec3<f32>, n: Vec3<f32>, p0: Vec3<f32>, d: Vec3<f32>) -> Option<Vec3<f32>> {
  let t = (p * n - p0 * n) * (1. / (d * n));
  if t > 0. {
    Some(p0 + (d * t))
  } else {
    None
  }
}

struct MyShader<'a> {
  cubemap: &'a Cubemap,
  mat: Mat4,
  invert: Mat4,
}
impl<'a, M: Model> pipeline2::Shader<M> for MyShader<'a> {
  fn vertext(&mut self, model: &M, face: usize, nth_vert: usize) -> Vec4<f32> {
    &self.mat * Vec4::from_point(&model.vert(face, nth_vert))
  }

  fn fragment(&self, info: pipeline2::FragmentInfo) -> Fragment {
    let p =  &self.invert * &info.coordinate();
    Fragment::Color(self.cubemap.get(p))
  }
}

fn main() {
  let mut degree = 0.;

  let cubemap = env::args()
    .collect::<Vec<_>>()
    .get(1)
    .map(|f| {
      if f == "color-cube" {
        Cubemap::colored()
      } else {
        Cubemap::new(
          &util::load_image("./models/earth/texture.tga"),
          [
            //(x,y,width,height)
            //front
            (0., 1. / 3., 1. / 4., 1. / 3.),
            //back
            (2. / 4., 1. / 3., 1. / 4., 1. / 3.),
            // left
            (3. / 4., 1. / 3., 1. / 4., 1. / 3.),
            // right
            (1. / 4., 1. / 3., 1. / 4., 1. / 3.),
            // top
            (1. / 2., 2. / 3., 1. / 4., 1. / 3.),
            // bottom
            (0., 0., 1. / 4., 1. / 3.),
          ],
        )
      }
    })
    .unwrap_or(Cubemap::new(
      &util::load_image("./models/earth/texture.tga"),
      [
        //front
        (0., 1. / 3., 1. / 4., 1. / 3.),
        //back
        (2. / 4., 1. / 3., 1. / 4., 1. / 3.),
        // left
        (3. / 4., 1. / 3., 1. / 4., 1. / 3.),
        // right
        (1. / 4., 1. / 3., 1. / 4., 1. / 3.),
        // top
        (1. / 2., 2. / 3., 1. / 4., 1. / 3.),
        // bottom
        (0., 0., 1. / 4., 1. / 3.),
      ],
    ));
  let mut model = Object::from_file("./models/earth/earth.obj").unwrap();
  model.normalize_verts();
  sdl::frame("earth", 400, 400, move |mut img, fps| {
    let mut depth = vec![f32::MIN; (img.width() * img.height()) as usize];
    let mat = Transform::new()
      .rotate_y(degree * 3.14 / 180.)
      // .then_mat(&transform::camera(
      //   Vec3::new(1., 1., 0.),
      //   Vec3::new(0., 1., 0.),
      //   Vec3::new(0., 0., 0.),
      // ))
      .build();
    pipeline2::render(
      &mut img,
      &mut depth,
      &mut MyShader {
        cubemap: &cubemap,
        invert: mat.invert(),
        mat,
      },
      &model,
      0,
    );
    if fps > 0. {
      let t = 1. / fps;
      let speed = 90.; // per seconds
      degree += t * speed;
    }
  });
}

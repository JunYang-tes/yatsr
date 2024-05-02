use yatsr::prelude::*;
struct Cubemap {
  pub texture: [Texture; 6],
}
impl Cubemap {
  fn new(image: &PixImage, faces: [(f32, f32, f32, f32); 6]) -> Cubemap {
    let c = Cubemap {
      texture: faces.map(|(x, y, w, h)| Texture::new(sub_img(image, x, y, w, h))),
    };
    save_image("front.ppm", &c.texture[0].image, PPM);
    save_image("back.ppm", &c.texture[1].image, PPM);
    save_image("left.ppm", &c.texture[2].image, PPM);
    save_image("right.ppm", &c.texture[3].image, PPM);
    save_image("top.ppm", &c.texture[4].image, PPM);
    save_image("bottom.ppm", &c.texture[5].image, PPM);
    c
  }
  fn colored() -> Cubemap {
    Cubemap {
      texture: [
        //#ff0000
        Texture::neareat(PixImage::from_data(
          vec![255, 0, 0, 0],
          1,
          1,
          yatsr::image::ImageOriginPos::LeftTop,
        )),
        //#00ff00
        Texture::neareat(PixImage::from_data(
          vec![0, 255, 0, 0],
          1,
          1,
          yatsr::image::ImageOriginPos::LeftTop,
        )),
        //left #0000ff
        Texture::neareat(PixImage::from_data(
          vec![0, 0, 255, 0],
          1,
          1,
          yatsr::image::ImageOriginPos::LeftTop,
        )),
        // right #ff00ff
        Texture::neareat(PixImage::from_data(
          vec![255, 0, 255, 0],
          1,
          1,
          yatsr::image::ImageOriginPos::LeftTop,
        )),
        //top #ffff00
        Texture::neareat(PixImage::from_data(
          vec![255, 255, 0, 0],
          1,
          1,
          yatsr::image::ImageOriginPos::LeftTop,
        )),
        // bottom #00ffff
        Texture::neareat(PixImage::from_data(
          vec![0, 255, 255, 0],
          1,
          1,
          yatsr::image::ImageOriginPos::LeftTop,
        )),
      ],
    }
  }
  fn get(&self, point: Vec3<f32>) -> Vec3<f32> {
    let cube = [
      // 由一个点和一个法向量定义平面
      (Vec3::new(0., 0., 1.), Vec3::new(0., 0., 1.)), // front
      (Vec3::new(-1., -1., -1.), Vec3::new(0., 0., -1.)), // back
      (Vec3::new(-1., -1., 1.), Vec3::new(-1., 0., 0.)), // left
      (Vec3::new(1., -1., 1.), Vec3::new(1., 0., 0.)), // right
      (Vec3::new(1., 1., 1.), Vec3::new(0., 1., 0.)), // top
      (Vec3::new(1., -1., 1.), Vec3::new(0., -1., 0.)), // bottom
    ];
    for (ind, (p, normal)) in cube.iter().enumerate() {
      let p = get_intersect(
        *p, *normal, point, point, // point - Vec3::new(0.,0.,0.)
      );
      if let Some(p) = p {
        if p.x >= -1. && p.x <= 1. && p.y >= -1. && p.y <= 1. && p.z >= -1. && p.z <= 1. {
          return match ind {
            0 => self.texture[ind].get((p.x + 1.) / 2., (p.y + 1.) / 2.),
            1 => self.texture[ind].get((-p.x + 1.) / 2., (p.y + 1.) / 2.),
            2 => self.texture[ind].get((p.z + 1.) / 2., (p.y + 1.) / 2.),
            // right
            3 => self.texture[ind].get((-p.z + 1.) / 2., (p.y + 1.) / 2.),
            // top
            4 => self.texture[ind].get((-p.x + 1.) / 2., (p.z + 1.) / 2.),
            _ => self.texture[ind].get((p.x + 1.) / 2., (p.z + 1.) / 2.),
          };
        }
      }
    }
    return Vec3::new(1., 1., 0.);
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

  fn fragment(
    &self,
    // 此点坐标
    pos: Vec3<f32>,
    // 此点处的质心坐标
    bar: Vec3<f32>,
  ) -> Fragment {
    let p = &self.invert * &pos;
    Fragment::Color(self.cubemap.get(p))
  }
}

fn main() {
  let img = util::load_image("./models/earth/texture.tga");
  save_image("tmp.ppm", &sub_img(&img, 0., 0., 1. / 4., 1. / 3.), PPM);

  let mut degree = 45.;
  let cubemap = Cubemap::new(
    &util::load_image("./models/earth/texture.tga"),
    [
      (0., 1. / 3., 1. / 4., 1. / 3.),
      (2. / 4., 1. / 3., 1. / 4., 1. / 3.),
      // left
      (3. / 4., 1. / 3., 1. / 4., 1. / 3.),
      (1. / 4., 1. / 3., 1. / 4., 1. / 3.),
      // top
      (1. / 2., 2./3., 1. / 4., 1. / 3.),
      (0., 0., 1. / 4., 1. / 3.),
    ],
  );
  let mut model = Object::from_file("./models/earth/earth.obj").unwrap();
  model.normalize_verts();
  sdl::frame("earth", 500, 500, move |mut img, fps| {
    let mut depth = vec![f32::MIN; (img.width() * img.height()) as usize];
    let mat = Transform::new()
      .rotate_y(degree * 3.14 / 180.)
      .viewport(img.width() as f32, img.height() as f32)
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
      let speed = 45.; // per seconds
      degree += t * speed;
    }
  });
}

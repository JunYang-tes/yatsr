use crate::prelude::*;
pub struct Texture {
  pub image: PixImage,
  lookup: fn(img: &PixImage, u: f32, v: f32) -> Vec3<f32>,
}

pub enum Filter {
  Neareat,
  Bilinear,
}

fn neareat(img: &PixImage, u: f32, v: f32) -> Vec3<f32> {
  let u = u.clamp(0., 1.);
  let v = v.clamp(0., 1.);
  let x = ((img.width() - 1) as f32 * u).round();
  let y = ((img.height() - 1) as f32 * v).round();
  img.get_rgbf(x as u32, y as u32)
}
fn bilinear(img: &PixImage, u: f32, v: f32) -> Vec3<f32> {
  let u = u.clamp(0., 1.);
  let v = v.clamp(0., 1.);
  let x = (img.width() - 1) as f32 * u;
  let y = (img.height() - 1) as f32 * v;
  let c1 = img.get_rgbf(x.floor() as u32, y.floor() as u32);
  let c2 = img.get_rgbf(x.ceil() as u32, y.floor() as u32);
  let c3 = util::linear_interpolation(x - x.floor(), c1, c2);
  let c4 = img.get_rgbf(x.floor() as u32, y.ceil() as u32);
  let c5 = img.get_rgbf(x.ceil() as u32, y.ceil() as u32);
  let c6 = util::linear_interpolation(x - x.floor(), c4, c5);
  util::linear_interpolation(y - y.floor(), c3, c6)
}

impl Texture {
  pub fn new(image: PixImage) -> Texture {
    Texture {
      image,
      lookup: bilinear,
    }
  }
  pub fn neareat(image: PixImage) -> Texture {
    Texture {
      image,
      lookup: neareat,
    }
  }
  pub fn get(&self, u: f32, v: f32) -> Vec3<f32> {
    (self.lookup)(&self.image, u, v)
  }
}

pub struct Cubemap {
  texture: [Texture; 6],
}
impl Cubemap {
  pub fn new(image: &PixImage, faces: [(f32, f32, f32, f32); 6]) -> Cubemap {
    let c = Cubemap {
      texture: faces.map(|(x, y, w, h)| Texture::new(util::sub_img(image, x, y, w, h))),
    };
    c
  }
  pub fn colored() -> Cubemap {
    Cubemap {
      texture: [
        //#ff0000
        Texture::neareat(PixImage::from_data(
          vec![255, 0, 0, 0],
          1,
          1,
          crate::image::ImageOriginPos::LeftTop,
        )),
        //#00ff00
        Texture::neareat(PixImage::from_data(
          vec![0, 255, 0, 0],
          1,
          1,
          crate::image::ImageOriginPos::LeftTop,
        )),
        //left #0000ff
        Texture::neareat(PixImage::from_data(
          vec![0, 0, 255, 0],
          1,
          1,
          crate::image::ImageOriginPos::LeftTop,
        )),
        // right #ff00ff
        Texture::neareat(PixImage::from_data(
          vec![255, 0, 255, 0],
          1,
          1,
          crate::image::ImageOriginPos::LeftTop,
        )),
        //top #ffff00
        Texture::neareat(PixImage::from_data(
          vec![255, 255, 0, 0],
          1,
          1,
          crate::image::ImageOriginPos::LeftTop,
        )),
        // bottom #00ffff
        Texture::neareat(PixImage::from_data(
          vec![0, 255, 255, 0],
          1,
          1,
          crate::image::ImageOriginPos::LeftTop,
        )),
      ],
    }
  }
  pub fn get_uv(&self, point: Vec3<f32>) -> Vec3<f32> {
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
      let p = util::intersect_of_plan_line(
        *p, *normal, point, point, // point - Vec3::new(0.,0.,0.)
      );
      if let Some(p) = p {
        if p.x >= -1. && p.x <= 1. && p.y >= -1. && p.y <= 1. && p.z >= -1. && p.z <= 1. {
          let (u, v) = match ind {
            0 => ((p.x + 1.) / 2., (p.y + 1.) / 2.),
            1 => ((-p.x + 1.) / 2., (p.y + 1.) / 2.),
            2 => ((p.z + 1.) / 2., (p.y + 1.) / 2.),
            // right
            3 => ((-p.z + 1.) / 2., (p.y + 1.) / 2.),
            // top
            4 => ((-p.x + 1.) / 2., (p.z + 1.) / 2.),
            _ => ((p.x + 1.) / 2., (p.z + 1.) / 2.),
          };
          return Vec3::new(u, v, 0.);
        }
      }
    }
    return Vec3::new(1., 1., 0.);
  }
  pub fn get(&self, point: Vec3<f32>) -> Vec3<f32> {
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
      let p = util::intersect_of_plan_line(
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

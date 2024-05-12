use yatsr::prelude::*;

struct MyShader<'a> {
  texture: &'a Texture,
  varying_normals: [Vec3<f32>; 3],
  light_dir: Vec3<f32>,
  time: f32,
}
impl<'a, M: Model> pipeline2::Shader<M> for MyShader<'a> {
  fn vertext(&mut self, model: &M, face: usize, nth_vert: usize) -> Vec4<f32> {
    let p = Vec4::from_point(&model.vert(face, nth_vert));
    self.varying_normals[nth_vert] = model.normal(face, nth_vert);
    p
  }

  fn fragment(&self, info: pipeline2::FragmentInfo) -> Fragment {
    let p = info.coordinate();
    let mut x = p.x * 2.; // [-1,1] => [-2,2]
    let mut y = p.y * 2.;

    let v = heart_surface(x, y, 0.);

    if v > 0. {
      return Fragment::Discard;
    }
    if y == 0. {
      y += 0.0001;
    }
    if x == 0. {
      x += 0.0001;
    }
    let z = find_z(x, y);
    // let tx = (x / 2.);// / (z + 0.001);
    // let ty = (y / 2.);// / (z + 0.001);
    let tx = p.x*1.5;
    let ty = p.y*1.5-0.5;
    let color = self.texture.get((tx + 1.) / 2., (ty + 1.) / 2.);

    let light = self.light_dir;
    let normal = normal(x, y, z);
    Fragment::Color(color * (light * normal).max(0.1))
  }
}
fn normal(x: f32, y: f32, z: f32) -> Vec3<f32> {
  let v = heart_surface(x, y, z);
  let d = 0.000001;
  Vec3::new(
    (heart_surface(x + d, y, z) - v) / d,
    (heart_surface(x, y + d, z) - v) / d,
    (heart_surface(x, y, z + d) - v) / d,
  )
  .normalize()
}
fn heart_surface(x: f32, y: f32, z: f32) -> f32 {
  (x.powi(2) + y.powi(2) - 1. + 9. * z.powi(2) / 4.).powi(3)
    - x.powi(2) * y.powi(3)
    - 9. * z.powi(2) * y.powi(3) / 80.
}
fn find_z(x: f32, y: f32) -> f32 {
  return find_z1(x, y);
  // let mut z = 1.0;
  // let mut max = 1000;
  // while heart_surface(x, y, z) > 0. {
  //   z = z - 0.001;
  //   max = max - 1;
  //   if max < 2 {
  //     break;
  //   }
  // }
  // //println!("{}",z);
  // return z;
}
fn find_z1(x: f32, y: f32) -> f32 {
  let mut left = 0.;
  let mut right = 2.;
  let e = 0.0000001;
  while right - left > e {
    let mid = (right + left) / 2.;
    let v = heart_surface(x, y, mid);
    if v == 0. {
      return mid;
    }

    let left_v = heart_surface(x, y, left);
    if left_v * v < 0. {
      right = mid
    } else {
      left = mid
    }
  }
  return (right + left) / 2.;
}

fn main() {
  let light_pos = Vec3::new(1., 1., 1.);
  let mut light_degree = 0.;
  let mut time = 0.;
  let texture = Texture::new(util::load_image("./a.tga"));

  sdl::frame("A", 800, 800, |mut img, fps| {
    let mut depth = vec![f32::MIN; img.width() as usize * img.height() as usize];
    let model = shape::Plane::new();
    let light_pos = &transform::rotate_y(light_degree) * &light_pos;

    pipeline2::render(
      &mut img,
      &mut depth,
      &mut MyShader {
        texture: &texture,
        light_dir: light_pos.normalize(),
        time,
        varying_normals: [Vec3::default(), Vec3::default(), Vec3::default()],
      },
      &model,
      0,
    );
    if fps > 0. {
      let t = 1. / fps;
      let speed = 2.; // per seconds
      time += t;
      //light_degree += t * speed;
    }
  })
}

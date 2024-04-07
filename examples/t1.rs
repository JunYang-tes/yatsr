use yatsr::{
  file::save_image,
  geometry::{Vec3, Vec4},
  image::{Image, PixImage},
  image_encoder::PPM,
  model::Model,
  pipeline::{render, Shader},
};
struct Flat {
  light: Vec3<f32>,
  varying_color: Vec3<f32>,
}
impl Shader for Flat {
  fn vertext(&mut self, model: &Model, face: usize, nth_vert: usize) -> yatsr::geometry::Vec3<f32> {
    if nth_vert == 0 {
      let i = (model.normal_of_face(face) * self.light).max(0.);
      self.varying_color = Vec3::new(1., 0., 0.) * i;
    }
    let p = model.vert(face, nth_vert);
    let p = p + Vec3::new(1., 1., 1.); // [-1,1] => [0,2]
    let mut p = p * 0.5; // [0,2] => [0,1]
    p.x *= 500.;
    p.y *= 500.;
    p
  }

  fn fragment(
    &self,
    // 此点坐标
    pos: yatsr::geometry::Vec3<f32>,
    // 此点处的质心坐标
    bar: yatsr::geometry::Vec3<f32>,
  ) -> yatsr::pipeline::Fragment {
    yatsr::pipeline::Fragment::Color(self.varying_color)
  }
}
struct Gouraud {
  light: Vec3<f32>,
  varying_color: [Vec3<f32>; 3],
}
impl Shader for Gouraud {
  fn vertext(&mut self, model: &Model, face: usize, nth_vert: usize) -> Vec3<f32> {
    let n = model.normal(face, nth_vert);
    let i = (n * self.light).max(0.);
    self.varying_color[nth_vert] = Vec3::new(1., 0., 0.) * i;
    let p = model.vert(face, nth_vert);
    let p = p + Vec3::new(1., 1., 1.); // [-1,1] => [0,2]
    let mut p = p * 0.5; // [0,2] => [0,1]
    p.x *= 500.;
    p.y *= 500.;
    p.x += 500.;
    //p.y += 500.;
    p
  }

  fn fragment(
    &self,
    // 此点坐标
    pos: Vec3<f32>,
    // 此点处的质心坐标
    bar: Vec3<f32>,
  ) -> yatsr::pipeline::Fragment {
    yatsr::pipeline::Fragment::Color(
      self.varying_color[0] * bar.x + self.varying_color[1] * bar.y + self.varying_color[2] * bar.z,
    )
  }
}
struct Phong {
  light: Vec3<f32>,
  varying_normals: [Vec3<f32>; 3],
}
impl Shader for Phong {
  fn vertext(&mut self, model: &Model, face: usize, nth_vert: usize) -> Vec3<f32> {
    let n = model.normal(face, nth_vert);
    self.varying_normals[nth_vert] = n;
    let p = model.vert(face, nth_vert);
    let p = p + Vec3::new(1., 1., 1.); // [-1,1] => [0,2]
    let mut p = p * 0.5; // [0,2] => [0,1]
    p.x *= 500.;
    p.y *= 500.;
    p.x += 1000.;
    p
  }

  fn fragment(
    &self,
    // 此点坐标
    pos: Vec3<f32>,
    // 此点处的质心坐标
    bar: Vec3<f32>,
  ) -> yatsr::pipeline::Fragment {
    let n = self.varying_normals[0] * bar.x
      + self.varying_normals[1] * bar.y
      + self.varying_normals[2] * bar.z;
    let i = (n.normalize() * self.light).max(0.);
    yatsr::pipeline::Fragment::Color(Vec3::new(1., 0., 0.) * i)
  }
}
fn dump_depth_map(data: &Vec<f32>, width: u32, height: u32, path: &str) {
  let max = data.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
  let min = data.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
  let mut img = PixImage::new(width, height);
  for x in 0..width {
    for y in 0..height {
      let i = y * width + x;
      let p = data[i as usize];
      let r = ((p - min) / (max - min) * 255.) as u8;
      img.set_rgb24(x, y, Vec3::new(r, r, r))
    }
  }
  save_image(path, &img, PPM).unwrap();
}

fn main() {
  //let mut model = Model::from_file("./models/spot/spot_triangulated.obj").unwrap();
  let mut model = Model::from_file("./models/earth/earth.obj").unwrap();
  model.normalize_verts();
  let mut img = PixImage::new(1500, 500);
  let mut depth = vec![f32::MIN; (img.width * img.height) as usize];
  render(
    &mut img,
    &mut depth,
    &mut Flat {
      light: Vec3::new(1., 1., 1.).normalize(),
      varying_color: Vec3::new(1., 1., 0.),
    },
    &model,
    false,
  );
  dump_depth_map(&depth, 1500, 500, "d1.ppm");
  render(
    &mut img,
    &mut depth,
    &mut Gouraud {
      light: Vec3::new(1., 1., 1.).normalize(),
      varying_color: [Vec3::default(), Vec3::default(), Vec3::default()],
    },
    &model,
    false,
  );
  dump_depth_map(&depth, 1500, 500, "d2.ppm");
  render(
    &mut img,
    &mut depth,
    &mut Phong {
      light: Vec3::new(1., 1., 1.).normalize(),
      varying_normals: [Vec3::default(), Vec3::default(), Vec3::default()],
    },
    &model,
    false,
  );
  dump_depth_map(&depth, 1500, 500, "d3.ppm");
  save_image("t.ppm", &img, PPM);
}

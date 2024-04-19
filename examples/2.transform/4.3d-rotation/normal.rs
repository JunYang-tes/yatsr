use std::env;
use yatsr::prelude::*;

struct FlatShader {
  uniform_rotation: Mat4,
  uniform_viewport: Mat4,
  varying_color: Vec3<f32>,
}
impl<M:Model> Shader<M> for FlatShader {
  fn vertext(&mut self, model: &M, face: usize, nth_vert: usize) -> Vec3<f32> {
    if nth_vert == 0 {
      let normal = model.normal_of_face(face);
      // 通过乘以“逆的转置“来得到变换后的法向量
      let normal = (&self.uniform_rotation.invert().transpose() * &normal).normalize();
      self.varying_color = Vec3::new(0.05, 0.05, 0.05)
        + Vec3::new(1., 1., 1.) * (normal * Vec3::new(1., 1., 1.).normalize()).max(0.);
    }
    let v = model.vert(face, nth_vert);
    &(&self.uniform_viewport * &self.uniform_rotation) * &v
  }

  fn fragment(
    &self,
    // 此点坐标
    pos: Vec3<f32>,
    // 此点处的质心坐标
    bar: Vec3<f32>,
  ) -> Fragment {
    Fragment::Color(self.varying_color)
  }
}

fn main() {
  let mut img = PixImage::new(1000, 1000);
  let model_path = env::args()
    .collect::<Vec<_>>()
    .get(1)
    .map(|f| f.clone())
    .unwrap_or(String::from("./models/spot/spot_triangulated.obj"));
  let mut model = Object::from_file(model_path).expect("Failed to load model:,");
  model.normalize_verts();
  let cal_lite = yatsr::font::get_cal_lite();

  let mut depth_buffer = vec![f32::MIN; (img.width() * img.height()) as usize];
  render(
    &mut img,
    &mut depth_buffer,
    &mut FlatShader {
      uniform_rotation: Mat4::identity(),
      uniform_viewport: Transform::new()
        .translate(1., 1., 0.) // [-1,1] ==> [0,2]
        .scale(0.5, 0.5, 1.) // [0,2]==> [0,1]
        .scale(500., 500., 1.) // [0,1]=>[0,500]
        .translate(0., 0., 0.)
        .build(),
      varying_color: Vec3::default(),
    },
    &model,
    false,
  );
  cal_lite.draw_text(&mut img, 0, 20, 4, Vec3::new(1., 1., 1.), "Fig. 1 origin");

  render(
    &mut img,
    &mut depth_buffer,
    &mut FlatShader {
      uniform_rotation: transform::rotate_y(45. * 3.14 / 180.),
      uniform_viewport: Transform::new()
        .translate(1., 1., 0.) // [-1,1] ==> [0,2]
        .scale(250., 250., 1.) // [0,2]==> [0,500]
        .translate(500., 0., 0.) // [0,500] => [500,500]
        .build(),
      varying_color: Vec3::default(),
    },
    &model,
    false,
  );
  cal_lite.draw_text(
    &mut img,
    500,
    20,
    4,
    Vec3::new(1., 1., 1.),
    "Fig. 2 rotate around y",
  );

  render(
    &mut img,
    &mut depth_buffer,
    &mut FlatShader {
      uniform_rotation: transform::rotate_x(45. * 3.14 / 180.),
      uniform_viewport: Transform::new()
        .translate(1., 1., 0.) // [-1,1] ==> [0,2]
        .scale(250., 250., 1.) // [0,2]==> [0,500]
        .translate(500., 500., 0.) // [0,500]==> [500,100]
        .build(),
      varying_color: Vec3::default(),
    },
    &model,
    false,
  );
  cal_lite.draw_text(
    &mut img,
    500,
    520,
    4,
    Vec3::new(1., 1., 1.),
    "Fig. 3 rotate around x",
  );

  render(
    &mut img,
    &mut depth_buffer,
    &mut FlatShader {
      uniform_rotation: transform::rotate_z(45. * 3.14 / 180.),
      uniform_viewport: Transform::new()
        .translate(1., 1., 0.) // [-1,1] ==> [0,2]
        .scale(250., 250., 1.) // [0,2]==> [0,500]
        .translate(0., 500., 0.) // [0,500] ==> [0,1000]
        .build(),
      varying_color: Vec3::default(),
    },
    &model,
    false,
  );
  cal_lite.draw_text(
    &mut img,
    0,
    520,
    4,
    Vec3::new(1., 1., 1.),
    "Fig. 4 rotate around z",
  );

  save_image("output.ppm", &img, PPM).unwrap()
}

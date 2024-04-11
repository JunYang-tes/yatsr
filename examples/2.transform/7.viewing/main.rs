use yatsr::{prelude::*, shaders::FlatShader};
fn camera1(up: Vec3<f32>, looking: Vec3<f32>, pos: Vec3<f32>) -> Mat4 {
  let t = transform::translate(-pos.x, -pos.y, -pos.z);
  let x_ = looking.cross_product(up);
  #[rustfmt::skip]
  let m = Mat4([
      x_.x,x_.y,x_.z,0.,
      up.x,up.y,up.z,0.,
      -looking.x,-looking.y,-looking.z,0.,
      0.,0.,0.,1.
    ]);
  &m * &t
}
fn camera2(up: Vec3<f32>, looking: Vec3<f32>, pos: Vec3<f32>) -> Mat4 {
  let up = up.normalize();
  let looking = looking.normalize();
  let x_ = looking.cross_product(up);
  let up = x_.cross_product(looking);
  camera1(up, looking, pos)
}

fn camera3(up: Vec3<f32>, pos: Vec3<f32>, lookat: Vec3<f32>) -> Mat4 {
  camera2(up, lookat - pos, pos)
}

fn main() {
  let which_camera = std::env::args()
    .collect::<Vec<_>>()
    .get(1)
    .map(|s| s.clone())
    .unwrap_or(String::from("c1"));
  let mut img = PixImage::new(500, 500);
  let mut depth_buffer = vec![f32::MIN; (img.width() * img.height()) as usize];
  //let mut model = Model::from_file("./models/cube/cube.obj").expect("Failed to load model:,");
  let mut model =
    Model::from_file("./models/spot/spot_triangulated.obj").expect("Failed to load model:,");
  // 期望看见头在右，尾巴在左的牛的侧面
  let c1 = camera1(
    Vec3::new(0., 1., 0.),
    Vec3::new(-1., 0., 0.),
    Vec3::new(0., 0., 0.),
  );
  let c2 = camera1(
    Vec3::new(0., -1., 0.),
    Vec3::new(-1., 0., 0.),
    Vec3::new(0., 0., 0.),
  );
  let c3 = camera2(
    Vec3::new(0., 1., 0.),
    Vec3::new(-1., -1., -1.),
    Vec3::new(0., 0., 0.),
  );
  let c = match which_camera.as_str() {
    "c1" => c1,
    "c2" => c2,
    "c3" => c3,
    _ => c1,
  };

  render(
    &mut img,
    &mut depth_buffer,
    &mut FlatShader::with_mvp(
      Mat4::identity(),
      c,
      transform::orthographic(-2., 2., -2., 2., -2., 2.),
      transform::viewport(500., 500.),
    ),
    &model,
    false,
  );
  save_image("output.ppm", &img, PPM);
}

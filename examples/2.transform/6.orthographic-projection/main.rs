use yatsr::{prelude::*, shaders::FlatShader};

fn orthographic(left: f32, right: f32, bottom: f32, top: f32, far: f32, near: f32) -> Mat4 {
  Transform::new()
    //[left,right]x[bottom,top]x[far,near] => [0,right-left]x[0,top-bottom]x[0,near - far]
    .translate(-left, -bottom, -far)
    //[0,right-left]x[0,top-bottom]x[0,near - far] => [0,1]x[0,1]x[0,1]
    .scale(1. / (right - left), 1. / (top - bottom), 1. / (near - far))
    //[0,1]x[0,1]x[0,1] => [0,2]x[0,2]x[0,2]
    .scale(2., 2., 2.)
    //[0,2]x[0,2]x[0,2] => [0,1]x[0,1]x[0,1]
    .translate(-1., -1., -1.)
    .build()
}
fn main() {
  let mut img = PixImage::new(500, 500);
  let mut depth_buffer = vec![f32::MIN; (img.width() * img.height()) as usize];
  let mut model =
    Model::from_file("./models/spot/spot_triangulated.obj").expect("Failed to load model:,");
  let orth = orthographic(-2., 2., -2., 2., -2., 2.);
  render(
    &mut img,
    &mut depth_buffer,
    &mut FlatShader::with_transform(
      &orth
        * &Transform::new()
          .rotate_y(-45. * 3.14 / 180.)
          .translate(-1., 0., 0.)
          .build(),
      500.,
      500.,
    ),
    &model,
    false,
  );
  let mut model = Model::from_file("./models/cube/cube.obj").expect("Failed to load model:,");

  render(
    &mut img,
    &mut depth_buffer,
    &mut FlatShader::with_transform(
      Transform::new()
        .rotate_x(45. * 3.14 / 180.)
        .rotate_y(45. * 3.14 / 180.)
        .rotate_z(45. * 3.14 / 180.)
        .scale(0.5, 0.5, 0.5)
        .translate(1., 0., 0.)
        // 应用完上面的变换，再应用下面的正交投影
        .then_mat(&orth)
        .build(),
      500.,
      500.,
    ),
    &model,
    false,
  );
  save_image("output.ppm", &img, PPM);
}

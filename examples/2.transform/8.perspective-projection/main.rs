use yatsr::prelude::*;
fn perspective(left: f32, right: f32, bottom: f32, top: f32, far: f32, near: f32) -> Mat4 {
  #[rustfmt::skip]
    let m = Mat4([
      near, 0.,  0.,  0.,
      0.,near ,  0.,  0.,
      0.,0.,  near+far ,-far*near,
      0.,0.,  1.,  0.
    ]);
  &transform::orthographic(left, right, bottom, top, far, near) * &m
}

fn main() {
  let cube = Model::from_file("./models/cube/cube.obj").expect("Failed to load model:,");
  let model =
    Model::from_file("./models/spot/spot_triangulated.obj").expect("Failed to load model:,");
  let mut angle = 0.;
  let font = get_cal_lite();
  let p = perspective(-1., 1., -1., 1., -3., -1.);
  sdl::frame("Perspective", 500, 500, |mut img, fps| {
    let mut depth_buffer = vec![f32::MIN; 500 * 500];
    let pos = &transform::rotate_y(angle) * &Vec3::new(0., 4., 4.);
    let persp = perspective(-1., 1., -1., 1., -3., -1.);
    render(
      &mut img,
      &mut depth_buffer,
      &mut shaders::FlatShader::with_mvp(
        Transform::new().translate(-2., 0., 0.).build(),
        transform::camera(Vec3::new(0., 1., 0.), pos, Vec3::new(0., 0., 0.)),
        persp.clone(),
        transform::viewport(500., 500.),
      ),
      &cube,
      false,
    );
    render(
      &mut img,
      &mut depth_buffer,
      &mut shaders::FlatShader::with_mvp(
        Transform::new().translate(2., 0., 0.).build(),
        transform::camera(Vec3::new(0., 1., 0.), pos, Vec3::new(0., 0., 0.)),
        persp.clone(),
        transform::viewport(500., 500.),
      ),
      &cube,
      false,
    );
    render(
      &mut img,
      &mut depth_buffer,
      &mut shaders::FlatShader::with_mvp(
        Transform::new().translate(0., 0., 0.).build(),
        transform::camera(Vec3::new(0., 1., 0.), pos, Vec3::new(0., 0., 0.)),
        persp,
        transform::viewport(500., 500.),
      ),
      &model,
      false,
    );

    angle = angle + 0.1;
    font.draw_text(
      &mut img,
      10,
      10,
      4,
      Vec3::new(1., 0., 0.),
      format!("fps:{}", fps).as_str(),
    );
  })
}

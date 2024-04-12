use yatsr::prelude::*;
use yatsr::sdl::frame;
use yatsr::shaders::FlatShader;
fn main() {
  let font = get_cal_lite();
  let model =
    Model::from_file("./models/spot/spot_triangulated.obj").expect("Failed to load model:,");
  let mut angle = 0.;

  let mut depth_buffer = vec![f32::MIN; 500 * 500];
  frame("hello", 500, 500, move |mut img, fps| {
    depth_buffer.fill(f32::MIN);
    let pos = &transform::rotate_y(angle) * &Vec3::new(0., 0., 2.);
    render(
      &mut img,
      &mut depth_buffer,
      &mut FlatShader::with_mvp(
        Mat4::identity(),
        transform::camera(Vec3::new(0., 1., 0.), pos, Vec3::new(0., 0., 0.)),
        Mat4::identity(),
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

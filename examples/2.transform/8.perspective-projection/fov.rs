use yatsr::prelude::*;

fn perspective(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {
  let h = -near * (fov * std::f32::consts::PI / 180. / 2.).tan();
  let top = h;
  let bottom = -h;
  let w = aspect_ratio * h;
  let left = -w;
  let right = w;

  #[rustfmt::skip]
    let m = Mat4([
      near, 0.,  0.,  0.,
      0.,near ,  0.,  0.,
      0.,0.,  near+far ,-far*near,
      0.,0.,  1.,  0.
    ]);
  &transform::orthographic(left, right, bottom, top, far, near) * &m
}

struct Mats {
  model: Mat4,
  camera: Mat4,
  project: Mat4,
  viewport: Mat4,
}

fn draw(models: &[(&Object, Mats)], img: &mut PixImage) {
  let mut depth_buffer = vec![f32::MIN; (img.width() * img.height()) as usize];
  for (model, mats) in models {
    render(
      img,
      &mut depth_buffer,
      &mut shaders::FlatShader::with_mvp(
        mats.model.clone(),
        mats.camera.clone(),
        mats.project.clone(),
        mats.viewport.clone(),
      ),
      model,
      false,
    );
  }
}

fn main() {
  let mut image = PixImage::new(1500, 500);
  let cube = Object::from_file("./models/cube/cube.obj").expect("Failed to load model:,");
  let model =
    Object::from_file("./models/spot/spot_triangulated.obj").expect("Failed to load model:,");
  let camera = transform::camera(
    Vec3::new(0., 1., 0.),
    Vec3::new(4., 4., 0.),
    Vec3::new(0., 0., 0.),
  );
  let font = get_cal_lite();
  draw(
    &[
      (
        &cube,
        Mats {
          model: Mat4::identity(),
          camera: camera.clone(),
          project: perspective(25., 1., -2., -4.),
          viewport: transform::viewport(500., 500.),
        },
      ),
      (
        &cube,
        Mats {
          model: transform::translate(-2.5, 0., 0.),
          camera: camera.clone(),
          project: perspective(25., 1., -2., -4.),
          viewport: transform::viewport(500., 500.),
        },
      ),
      (
        &cube,
        Mats {
          model: transform::translate(-5., 0., 0.),
          camera: camera.clone(),
          project: perspective(25., 1., -2., -4.),
          viewport: transform::viewport(500., 500.),
        },
      ),
    ],
    &mut image,
  );
  font.draw_text(
    &mut image,
    10,
    10,
    4,
    Vec3::new(1., 1., 1.),
    "Fov 25 degree",
  );

  draw(
    &[
      (
        &cube,
        Mats {
          model: Mat4::identity(),
          camera: camera.clone(),
          project: perspective(45., 1., -2., -4.),
          viewport: Transform::new()
            .then_mat(&transform::viewport(500., 500.))
            .translate(500., 0., 0.)
            .build(),
        },
      ),
      (
        &cube,
        Mats {
          model: transform::translate(-2.5, 0., 0.),
          camera: camera.clone(),
          project: perspective(45., 1., -2., -4.),
          viewport: Transform::new()
            .then_mat(&transform::viewport(500., 500.))
            .translate(500., 0., 0.)
            .build(),
        },
      ),
      (
        &cube,
        Mats {
          model: transform::translate(-5., 0., 0.),
          camera: camera.clone(),
          project: perspective(45., 1., -2., -4.),
          viewport: Transform::new()
            .then_mat(&transform::viewport(500., 500.))
            .translate(500., 0., 0.)
            .build(),
        },
      ),
    ],
    &mut image,
  );
  font.draw_text(
    &mut image,
    510,
    10,
    4,
    Vec3::new(1., 1., 1.),
    "Fov 45 degree",
  );


  draw(
    &[
      (
        &cube,
        Mats {
          model: Mat4::identity(),
          camera: camera.clone(),
          project: perspective(65., 1., -2., -4.),
          viewport: Transform::new()
            .then_mat(&transform::viewport(500., 500.))
            .translate(1000., 0., 0.)
            .build(),
        },
      ),
      (
        &cube,
        Mats {
          model: transform::translate(-2.5, 0., 0.),
          camera: camera.clone(),
          project: perspective(65., 1., -2., -4.),
          viewport: Transform::new()
            .then_mat(&transform::viewport(500., 500.))
            .translate(1000., 0., 0.)
            .build(),
        },
      ),
      (
        &cube,
        Mats {
          model: transform::translate(-5., 0., 0.),
          camera: camera.clone(),
          project: perspective(65., 1., -2., -4.),
          viewport: Transform::new()
            .then_mat(&transform::viewport(500., 500.))
            .translate(1000., 0., 0.)
            .build(),
        },
      ),
    ],
    &mut image,
  );
  font.draw_text(
    &mut image,
    1010,
    10,
    4,
    Vec3::new(1., 1., 1.),
    "Fov 65 degree",
  );
  save_image("output.ppm", &image, PPM);
}

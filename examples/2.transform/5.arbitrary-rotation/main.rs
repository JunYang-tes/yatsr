use std::env;

use yatsr::{prelude::*, shaders::FlatShader};

fn rotate(u: Vec3<f32>, angle: f32) -> Mat4 {
  let x = Vec3::new(1., 0., 0.);
  let u = u.normalize();
  let v = x.cross_product(u).normalize();
  let w = u.cross_product(v).normalize();
  let M = Mat4([
    u.x,v.x,w.x, 0.,
    u.y,v.y,w.y, 0.,
    u.z,v.z,w.z, 0.,
    0.,  0.,  0.,  1.
  ]);
  &M.transpose() * &(&transform::rotate_x(angle) * &M)
}

fn main() {
  let model_path = env::args()
    .collect::<Vec<_>>()
    .get(1)
    .map(|f| f.clone())
    .unwrap_or(String::from("./models/spot/spot_triangulated.obj"));
  let mut model = Model::from_file(model_path).expect("Failed to load model:,");
  model.normalize_verts();
    let mut img = PixImage::new(500, 500);
    let mut depth_buffer = vec![f32::MIN; (img.width() * img.height()) as usize];
    render(
      &mut img,
      &mut depth_buffer,
      &mut FlatShader::with_transform(
        rotate(Vec3::new(1., 1., 1.), 90. * 3.14 / 180.),
        500.,
        500.,
      ),
      &model,
      false,
    );
    save_image("output.ppm", &img, PPM);
}

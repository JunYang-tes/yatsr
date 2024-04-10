use std::env;

use yatsr::prelude::*;

struct FlatShader(yatsr::shaders::FlatShader);
impl FlatShader {
  pub fn with_transform(mat: Mat4, w: f32, h: f32) -> FlatShader {
    FlatShader(yatsr::shaders::FlatShader::with_transform(mat, w, h))
  }
}
impl Shader for FlatShader {
  fn vertext(&mut self, model: &Model, face: usize, nth_vert: usize) -> Vec3<f32> {
    let p = self.0.vertext(model, face, nth_vert);
    // 让立方体的每面都有一个相同的颜色，以便观察旋转
    self.0.varying_color = model.normal(face, nth_vert);
    p
  }

  fn fragment(
    &self,
    // 此点坐标
    pos: Vec3<f32>,
    // 此点处的质心坐标
    bar: Vec3<f32>,
  ) -> Fragment {
    self.0.fragment(pos, bar)
  }
}

fn rotate(u: Vec3<f32>, angle: f32) -> Mat4 {
  let x = Vec3::new(1., 0., 0.);
  let u = u.normalize();
  let v = x.cross_product(u).normalize();
  let w = u.cross_product(v).normalize();
  #[rustfmt::skip]
  let M = Mat4([
    u.x,u.y,u.z,0.,
    v.x,v.y,v.z,0.,
    w.x,w.y,w.z,0.,
    0.,0.,0.,1.
  ]);
  // uvw 是一组正交向量，因此M是正交矩阵，所以M的转置就是M的逆
  &M.transpose() * &(&transform::rotate_x(angle) * &M)
}

fn main() {
  let model_path = env::args()
    .collect::<Vec<_>>()
    .get(1)
    .map(|f| f.clone())
    .unwrap_or(String::from("./models/cube/cube.obj"));
  let mut model = Model::from_file(model_path).expect("Failed to load model:,");
  model.normalize_verts();
  let cal = get_cal_lite();
  for i in 0..10 {
    let mut img = PixImage::new(500, 500);
    let mut depth_buffer = vec![f32::MIN; (img.width() * img.height()) as usize];
    cal.draw_text(
      &mut img,
      10,
      10,
      4,
      Vec3::new(1., 1., 1.),
      format!("rotate: {} degree", (i as f32 * 15.)).as_str(),
    );

    render(
      &mut img,
      &mut depth_buffer,
      &mut FlatShader::with_transform(
        &rotate(Vec3::new(1., 1., 1.), (i as f32 * 15.) * 3.14 / 180.)
          * &transform::scale(0.5, 0.5, 0.5),
        500.,
        500.,
      ),
      &model,
      false,
    );
    save_image(format!("output{}.ppm", i), &img, PPM);
  }
}

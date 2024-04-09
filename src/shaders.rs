use crate::{
  mat::Mat4,
  pipeline::{Fragment, Shader},
  prelude::Vec3,
  transform::viewport,
};

pub struct FlatShader {
  uniform_mat: Mat4,
  uniform_vp: Mat4,
  uniform_light: Vec3<f32>,
  color: Vec3<f32>,
  varying_color: Vec3<f32>,
}
impl FlatShader {
  pub fn new(w: f32, h: f32) -> FlatShader {
    FlatShader {
      uniform_mat: Mat4::identity(),
      uniform_vp: viewport(w, h),
      uniform_light: Vec3::new(1., 1., 1.).normalize(),
      color: Vec3::new(1., 1., 1.),
      varying_color: Vec3::default(),
    }
  }
  pub fn with_transform(mat: Mat4, w: f32, h: f32) -> FlatShader {
    FlatShader {
      uniform_vp: viewport(w, h),
      uniform_mat: mat,
      uniform_light: Vec3::new(1., 1., 1.).normalize(),
      color: Vec3::new(1., 1., 1.),
      varying_color: Vec3::default(),
    }
  }
}
impl Shader for FlatShader {
  fn vertext(&mut self, model: &crate::model::Model, face: usize, nth_vert: usize) -> Vec3<f32> {
    if nth_vert == 0 {
      let normal = &self.uniform_mat.invert().transpose() * &model.normal_of_face(face).normalize();
      self.varying_color =
        Vec3::new(0.05, 0.05, 0.05) + self.color * (normal * self.uniform_light).max(0.);
    }
    let v = model.vert(face, nth_vert);
    &(&self.uniform_vp * &self.uniform_mat) * &v
  }

  fn fragment(
    &self,
    // 此点坐标
    pos: Vec3<f32>,
    // 此点处的质心坐标
    bar: Vec3<f32>,
  ) -> crate::pipeline::Fragment {
    Fragment::Color(self.varying_color)
  }
}

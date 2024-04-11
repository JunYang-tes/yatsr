use crate::{
  mat::Mat4,
  pipeline::{Fragment, Shader},
  prelude::Vec3,
  transform::viewport,
};

pub struct FlatShader {
  pub uniform_model: Mat4,
  pub uniform_viewing: Mat4,
  pub uniform_project: Mat4,
  pub uniform_vp: Mat4,
  pub uniform_light: Vec3<f32>,
  pub color: Vec3<f32>,
  pub varying_color: Vec3<f32>,
}
impl FlatShader {
  pub fn new(w: f32, h: f32) -> FlatShader {
    FlatShader {
      uniform_model: Mat4::identity(),
      uniform_project: Mat4::identity(),
      uniform_viewing: Mat4::identity(),
      uniform_vp: viewport(w, h),
      uniform_light: Vec3::new(1., 1., 1.).normalize(),
      color: Vec3::new(1., 1., 1.),
      varying_color: Vec3::default(),
    }
  }
  pub fn with_mvp(m: Mat4, v: Mat4, p: Mat4, vp: Mat4) -> FlatShader {
    FlatShader {
      uniform_vp: vp,
      uniform_project: p,
      uniform_viewing: v,
      uniform_model: m,
      uniform_light: Vec3::new(1., 1., 1.).normalize(),
      color: Vec3::new(1., 1., 1.),
      varying_color: Vec3::default(),
    }
  }
  pub fn with_transform(model: Mat4, w: f32, h: f32) -> FlatShader {
    FlatShader {
      uniform_vp: viewport(w, h),
      uniform_project: Mat4::identity(),
      uniform_viewing: Mat4::identity(),
      uniform_model: model,
      uniform_light: Vec3::new(1., 1., 1.).normalize(),
      color: Vec3::new(1., 1., 1.),
      varying_color: Vec3::default(),
    }
  }
}
impl Shader for FlatShader {
  fn vertext(&mut self, model: &crate::model::Model, face: usize, nth_vert: usize) -> Vec3<f32> {
    if nth_vert == 0 {
      let normal = model.normal_of_face(face);
      let normal = &self.uniform_model.invert().transpose() * &normal;
      self.varying_color = self.color * (normal * self.uniform_light).max(0.);
    }
    let v = model.vert(face, nth_vert);
    &crate::transform::Transform::new()
      .then_mat(&self.uniform_model)
      .then_mat(&self.uniform_viewing)
      .then_mat(&self.uniform_project)
      .then_mat(&self.uniform_vp)
      .build()
      * &v
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

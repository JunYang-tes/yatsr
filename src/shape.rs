use crate::{model::Model, prelude::Vec3};
pub struct Plane {
  verts: [Vec3<f32>; 6],
  uvs: [Vec3<f32>; 6],
}
impl Plane {
  pub fn new() -> Plane {
    Plane {
      verts: [
        //Left Upper triangle
        Vec3::new(-1., 1., 0.),
        Vec3::new(-1., -1., 0.),
        Vec3::new(1., 1., 0.),
        // Right lower triangle
        Vec3::new(1., 1., 0.),
        Vec3::new(-1., -1., 0.),
        Vec3::new(1., -1., 0.),
      ],
      uvs: [
        Vec3::new(0., 1., 0.),
        Vec3::new(0., 0., 0.),
        Vec3::new(1., 1., 0.),
        Vec3::new(1., 1., 0.),
        Vec3::new(0., 0., 0.),
        Vec3::new(1., 0., 0.),
      ],
    }
  }
}
impl Model for Plane {
  fn vert_count(&self) -> usize {
    6
  }

  fn face_count(&self) -> usize {
    2
  }

  fn vert(&self, face: usize, nth_vert: usize) -> Vec3<f32> {
    self.verts[face * 3 + nth_vert]
  }

  fn normal(&self, face: usize, nth_vert: usize) -> Vec3<f32> {
    Vec3::new(0., 0., 1.)
  }

  fn normal_of_face(&self, face: usize) -> Vec3<f32> {
    Vec3::new(0., 0., 1.)
  }

  fn texture_coord(&self, face: usize, nth_vert: usize) -> Vec3<f32> {
    self.uvs[face * 3 + nth_vert]
  }
}

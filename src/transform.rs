use crate::mat::Mat4;
pub fn scale(sx: f32, sy: f32, sz: f32) -> Mat4 {
  #[rustfmt::skip]
  let m = Mat4([sx,0.,0.,0.,
                0.,sy,0.,0.,
                0.,0.,sz,0.,
                0.,0.,0.,1.]);
  m
}
pub fn translate(dx: f32, dy: f32, dz: f32) -> Mat4 {
  // #[rustfmt::skip]
  // Mat4([1.,0.,0.,0.,
  //       0.,1.,0.,0.,
  //       0.,0.,1.,0.,
  //       0.,0.,0.,1. ])
  #[rustfmt::skip]
  let m = Mat4([1.,0.,0.,dx,
                0.,1.,0.,dy,
                0.,0.,1.,dz,
                0.,0.,0.,1. ]);
  m
}

// 绕z旋转
pub fn rotate_z(angle: f32) -> Mat4 {
  let c = angle.cos();
  let s = angle.sin();
  #[rustfmt::skip]
  let m = Mat4([c,-s,0.,0.,
                 s, c,0.,0.,
                 0.,0.,1.,0.,
                 0.,0.,0.,1. ]);
  m
}

pub fn rotate_x(angle: f32) -> Mat4 {
  let c = angle.cos();
  let s = angle.sin();
  #[rustfmt::skip]
  let m = Mat4([1.,0.,0.,0.,
                0.,c ,-s ,0.,
                0.,s,c ,0.,
                0.,0.,0.,1. ]);
  m
}
pub fn rotate_y(angle: f32) -> Mat4 {
  let c = angle.cos();
  let s = angle.sin();
  #[rustfmt::skip]
  let m = Mat4([c, 0.,s,0.,
                0.,1.,0.,0.,
                -s, 0.,c ,0.,
                0.,0.,0.,1. ]);
  m
}

pub struct Transform {
  mat: Mat4,
}
impl Transform {
  pub fn new() -> Transform {
    Transform {
      mat: Mat4::identity(),
    }
  }
  pub fn scale(mut self, sx: f32, sy: f32, sz: f32) -> Transform {
    self.mat = &scale(sx, sy, sz) * &self.mat;
    self
  }
  pub fn translate(mut self, tx: f32, ty: f32, tz: f32) -> Transform {
    self.mat = &translate(tx, ty, tz) * &self.mat;
    self
  }
  pub fn rotate_x(mut self, angle: f32) -> Transform {
    self.mat = &rotate_x(angle) * &self.mat;
    self
  }
  pub fn rotate_y(mut self, angle: f32) -> Transform {
    self.mat = &rotate_y(angle) * &self.mat;
    self
  }
  pub fn rotate_z(mut self, angle: f32) -> Transform {
    self.mat = &rotate_z(angle) * &self.mat;
    self
  }
  pub fn build(self) -> Mat4 {
    self.mat
  }
}

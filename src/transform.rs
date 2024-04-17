use crate::{mat::Mat4, prelude::Vec3};
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
pub fn rotate(u: Vec3<f32>, angle: f32) -> Mat4 {
  let x = Vec3::new(1., 0., 0.);
  let u = u.normalize();
  let v = x.cross_product(u).normalize();
  let w = u.cross_product(v).normalize();
  #[rustfmt::skip]
  let m = Mat4([
    u.x,u.y,u.z,0.,
    v.x,v.y,v.z,0.,
    w.x,w.y,w.z,0.,
    0.,0.,0.,1.
  ]);
  // uvw 是一组正交向量，因此M是正交矩阵，所以M的转置就是M的逆
  &m.transpose() * &(&rotate_x(angle) * &m)
}

pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, far: f32, near: f32) -> Mat4 {
  Transform::new()
    //[left,right]x[bottom,top]x[far,near] => [0,right-left]x[0,top-bottom]x[0,near - far]
    .translate(-left, -bottom, -far)
    //[0,right-left]x[0,top-bottom]x[0,near - far] => [0,2]x[0,2]x[0,2]
    .scale(2. / (right - left), 2. / (top - bottom), 2. / (near - far))
    //[0,2]x[0,2]x[0,2] => [0,1]x[0,1]x[0,1]
    .translate(-1., -1., -1.)
    .build()
}

pub fn viewport(w: f32, h: f32) -> Mat4 {
  Transform::new()
    .translate(1., 1., 0.)
    .scale(w / 2., h / 2., 1.)
    .build()
}

pub fn camera(up: Vec3<f32>, pos: Vec3<f32>, lookat: Vec3<f32>) -> Mat4 {
  let up = up.normalize();
  let looking = (lookat - pos).normalize();
  let x_ = looking.cross_product(up);
  let up = x_.cross_product(looking);
  let t = translate(-pos.x, -pos.y, -pos.z);
  #[rustfmt::skip]
  let m = Mat4([
      x_.x,x_.y,x_.z,0.,
      up.x,up.y,up.z,0.,
      -looking.x,-looking.y,-looking.z,0.,
      0.,0.,0.,1.
    ]);
  &m * &t
}

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
  &orthographic(left, right, bottom, top, far, near) * &m
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
  pub fn rotate(mut self, u: Vec3<f32>, angle: f32) -> Transform {
    self.mat = &rotate(u, angle) * &self.mat;
    self
  }
  pub fn then(mut self, other: &Transform) -> Transform {
    self.mat = &other.build() * &self.mat;
    self
  }
  pub fn then_mat(mut self, other: &Mat4) -> Transform {
    self.mat = other * &self.mat;
    self
  }
  pub fn build(&self) -> Mat4 {
    self.mat.clone()
  }
}

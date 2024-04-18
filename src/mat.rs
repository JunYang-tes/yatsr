use std::{fmt::Display, ops::*};

use crate::geometry::{Vec3, Vec4};
#[derive(Debug, Clone)]
pub struct Mat4(pub [f32; 16]);
impl Mat4 {
  pub fn zero() -> Mat4 {
    Mat4([0.; 16])
  }
  pub fn identity() -> Mat4 {
    Mat4([
      1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
    ])
  }
  pub fn set(&mut self, r: u8, c: u8, val: f32) {
    let ind = c + r * 4;
    self.0[ind as usize] = val;
  }
  pub fn get(&self, r: u8, c: u8) -> f32 {
    let ind = c + r * 4;
    self.0[ind as usize]
  }
  pub fn row(&self, i: u8) -> Vec4<f32> {
    let mut r = Vec4::default();
    for (ind, mat_ind) in ((i * 4)..(i + 1) * 4).enumerate() {
      r.set(ind as u8, self.0[mat_ind as usize])
    }
    r
  }
  pub fn col(&self, i: u8) -> Vec4<f32> {
    let mut r = Vec4::default();
    for (ind, mat_ind) in (i..16).step_by(4).enumerate() {
      r.set(ind as u8, self.0[mat_ind as usize])
    }
    r
  }
  pub fn transpose(&self) -> Mat4 {
    let mut ret = Mat4::zero();
    for r in 0..4 {
      for c in 0..4 {
        ret.set(r, c, self.get(c, r));
      }
    }
    ret
  }
  pub fn invert(&self) -> Mat4 {
    let d = det(&self.0, 4);
    let mut ret = Mat4::zero();
    for r in 0..4 {
      for c in 0..4 {
        let m = sub_mat(&self.0, 4, r, c);
        let sign = i32::pow(-1, (r + c) as u32) as f32;
        ret.set(r, c, sign * det(m.as_slice(), 3) / d);
      }
    }
    ret.transpose()
  }
}

fn det(mat: &[f32], order: u8) -> f32 {
  match order {
    2 => {
      let [a, b, c, d] = mat else {
        panic!("unexpected mat size")
      };
      a * d - b * c
    }
    _ => {
      let mut d = 0.;
      let r = 0;
      for c in 0..order {
        let a = mat[(c + r * order) as usize];
        let sub = sub_mat(mat, order, r, c);
        let sign = i32::pow(-1, (r + c) as u32) as f32;
        d = d + a * sign * det(sub.as_slice(), order - 1);
      }
      d
    }
  }
}
fn sub_mat(mat: &[f32], order: u8, row: u8, col: u8) -> Vec<f32> {
  let mut vec = Vec::with_capacity(((order - 1) * (order - 1)) as usize);
  for r in 0..order {
    if r == row {
      continue;
    }
    for c in 0..order {
      if c == col {
        continue;
      }
      vec.push(mat[(r * order + c) as usize])
    }
  }
  vec
}

impl Display for Mat4 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("\n");
    for r in 0..4 {
      for c in 0..4 {
        f.write_str(format!("{},", self.get(r, c)).as_str());
      }
      f.write_str("\n");
    }
    f.write_str("\n");
    Ok(())
  }
}
impl Mul<Vec4<f32>> for &Mat4 {
  type Output = Vec4<f32>;

  fn mul(self, rhs: Vec4<f32>) -> Self::Output {
    let Mat4([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]) = *self;
    Vec4::new(
      &Vec4::new(a, b, c, d) * &rhs,
      &Vec4::new(e, f, g, h) * &rhs,
      &Vec4::new(i, j, k, l) * &rhs,
      &Vec4::new(m, n, o, p) * &rhs,
    )
  }
}

impl Mul<&Vec3<f32>> for &Mat4 {
  type Output = Vec3<f32>;

  fn mul(self, rhs: &Vec3<f32>) -> Self::Output {
    let p = self * Vec4::from_point(rhs);
    p.to_3d_point()
  }
}
impl PartialEq for Mat4 {
  fn eq(&self, other: &Self) -> bool {
    self.0.iter().zip(other.0.iter()).all(|(a, b)| a == b)
  }
}

impl Mul for Mat4 {
  type Output = Mat4;
  fn mul(self, rhs: Self) -> Self::Output {
    let mut r = [0.0; 16];
    for row in 0..4 {
      for col in 0u8..4 {
        r[(row * 4 + col) as usize] = self.row(row) * rhs.col(col);
      }
    }
    Mat4(r)
  }
}
impl<'a> Mul<&'a Mat4> for Mat4 {
  type Output = Mat4;

  fn mul(self, rhs: &'a Mat4) -> Self::Output {
    self * (rhs.clone())
  }
}
impl<'a> Mul<&'a Mat4> for &'a Mat4 {
  type Output = Mat4;
  fn mul(self, rhs: Self) -> Self::Output {
    let mut r = [0.0; 16];
    for row in 0..4 {
      for col in 0u8..4 {
        r[(row * 4 + col) as usize] = self.row(row) * rhs.col(col);
      }
    }
    Mat4(r)
  }
}

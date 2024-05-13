use crate::{
  geometry::{Vec3, Vec4},
  image::Image,
  model::Object,
  pipeline::Fragment,
};

pub struct FragmentInfo {
  width: f32,
  height: f32,
  pub z: [f32; 3],
  pub vertices: [Vec3<f32>; 3],
  // 屏幕空间内片元坐标
  pub pos: Vec3<f32>,
  // 屏幕空间质心坐标
  pub bar: Vec3<f32>,
}
impl FragmentInfo {
  pub fn barycentric_interpolate(&self, props: &[Vec3<f32>; 3]) -> Vec3<f32> {
    crate::util::barycentric_interpolate(props, self.bar)
  }
  // 同一平面的其它位置的重心坐标
  pub fn barycentric(&self, x: f32, y: f32) -> Vec3<f32> {
    let bar = barycentric(self.vertices[0], self.vertices[1], self.vertices[2], x, y);
    let [wa, wb, wc] = self.z;
    let k = 1. / wa * bar.0 + 1. / wb * bar.1 + 1. / wc * bar.2;
    Vec3::new(bar.0 / wa / k, bar.1 / wb / k, bar.2 / wc / k)
  }
  pub fn top_barycentric(&self) -> Vec3<f32> {
    let x = self.pos.x;
    let y = self.pos.y - 1.;
    self.barycentric(x, y)
  }
  pub fn right_barycentry(&self) -> Vec3<f32> {
    let x = self.pos.x + 1.;
    let y = self.pos.y;
    self.barycentric(x, y)
  }
  pub fn coordinate(&self) -> Vec3<f32> {
    Vec3::new(
      self.pos.x / self.width * 2. - 1.,
      self.pos.y / self.height * 2. - 1.,
      self.pos.z,
    )
  }
}

pub trait Shader<M: crate::model::Model> {
  // 计算顶点在屏幕（渲染结果图像）上的位置
  fn vertext(&mut self, model: &M, face: usize, nth_vert: usize) -> Vec4<f32>;
  // 对于三角形内部的每点调用fragment计算该点处的颜色
  fn fragment(&self, info: FragmentInfo) -> Fragment;
}

type Point = Vec4<f32>;

pub fn barycentric(a: Vec3<f32>, b: Vec3<f32>, c: Vec3<f32>, x: f32, y: f32) -> (f32, f32, f32) {
  let alpha = ((b.x - x) * (c.y - b.y) + (y - b.y) * (c.x - b.x))
    / ((b.x - a.x) * (c.y - b.y) + (a.y - b.y) * (c.x - b.x));
  let beta = ((c.x - x) * (a.y - c.y) + (y - c.y) * (a.x - c.x))
    / ((c.x - b.x) * (a.y - c.y) + (b.y - c.y) * (a.x - c.x));
  (alpha, beta, 1. - alpha - beta)
}

pub fn super_sampling_offsets(m: u32) -> Vec<(f32, f32)> {
  let offsets = vec![(0., 0.); (m * m) as usize];
  let sub_pix_width = 1. / (m * m) as f32;
  offsets
    .iter()
    .enumerate()
    .map(|(idx, _)| {
      let x = idx % (m as usize);
      let y = idx / (m as usize);
      (
        (x + 1) as f32 * sub_pix_width - 0.5,
        (y + 1) as f32 * sub_pix_width - 0.5,
      )
    })
    .collect()
}
#[test]
fn test_supper_sample_offsets() {
  let offsets = super_sampling_offsets(2);
  println!("{:?}", offsets)
}

fn draw_triangle<M: crate::model::Model, S: Shader<M>, I: Image>(
  img: &mut I,
  depth_buff: &mut Vec<f32>,
  // 三角形的三个顶点，假设坐标在[-1,1]
  a: Vec4<f32>,
  b: Vec4<f32>,
  c: Vec4<f32>,
  shader: &mut S,
  super_sampling: &Option<Vec<(f32, f32)>>,
) {
  let wa = a.w;
  let wb = b.w;
  let wc = c.w;
  let vp = crate::transform::viewport(img.width() as f32, img.height() as f32);
  let a = &vp * &a.to_3d_point();
  let b = &vp * &b.to_3d_point();
  let c = &vp * &c.to_3d_point();
  // 映射标准立方体到屏幕空间

  let min_x = a.x.min(b.x).min(c.x) as u32;
  let max_x = a.x.max(b.x).max(c.x).min((img.width() - 1) as f32) as u32;
  let min_y = a.y.min(b.y).min(c.y) as u32;
  let max_y = a.y.max(b.y).max(c.y).min((img.height() - 1) as f32) as u32;
  let width = img.width() as f32;
  let height = img.height() as f32;

  if let Some(sub_pix_offset) = super_sampling {
    for y in min_y..=max_y {
      for x in min_x..=max_x {
        let mut color = Vec4::default();
        let mut cnt = 0;
        for (dx, dy) in sub_pix_offset {
          let (alpha, beta, gamma) = barycentric(a, b, c, (x as f32) + dx, (y as f32) + dy);
          if alpha < 0. || beta < 0. || gamma < 0. {
            continue;
          }
          let p = a * alpha + b * beta + c * gamma;
          let k = 1. / wa * alpha + 1. / wb * beta + 1. / wc * gamma;
          let info = FragmentInfo {
            width,
            height,
            z: [wa, wb, wc],
            vertices: [a, b, c],
            pos: p,
            bar: Vec3::new(alpha / wa / k, beta / wb / k, gamma / wc / k),
          };
          match shader.fragment(info) {
            Fragment::Color(c) => {
              color = color + Vec4::new(c.x, c.y, c.z, 1.);
              cnt += 1;
            }
            Fragment::Rgba(c) => {
              color = color + c;
              cnt += 1;
            }
            Fragment::Discard => {}
          }
        }
        if cnt > 0 {
          let (alpha, beta, gamma) = barycentric(a, b, c, x as f32, y as f32);
          let p = a * alpha + b * beta + c * gamma;
          let index = (y * img.width() + x) as usize;
          if p.z > depth_buff[index] {
            depth_buff[index] = p.z;
            img.blending(x, y, color * (1. / cnt as f32))
          }
        }
      }
    }
  } else {
    for y in min_y..=max_y {
      for x in min_x..=max_x {
        let (alpha, beta, gamma) = barycentric(a, b, c, x as f32, y as f32);
        if alpha < 0. || beta < 0. || gamma < 0. {
          continue;
        }
        let p = a * alpha + b * beta + c * gamma;
        let index = (y * img.width() + x) as usize;
        let k = 1. / wa * alpha + 1. / wb * beta + 1. / wc * gamma;
        if p.z > depth_buff[index] && p.z <= 1.1 && p.z >= -1.1 {
          // 通过Fragment shader 计算每个像素的颜色

          let info = FragmentInfo {
            z: [wa, wb, wc],
            vertices: [a, b, c],
            pos: p,
            width,
            height,
            bar: Vec3::new(alpha / wa / k, beta / wb / k, gamma / wc / k),
          };
          match shader.fragment(info) {
            Fragment::Color(c) => {
              depth_buff[index] = p.z;
              img.set_rgb(x, y, c);
            }
            Fragment::Rgba(c) => {
              depth_buff[index] = p.z;
              img.blending(x, y, c);
            }
            Fragment::Discard => {}
          }
        }
      }
    }
  }
}

pub fn render<S: Shader<M>, I: Image, M: crate::model::Model>(
  img: &mut I,
  depth_buff: &mut Vec<f32>,
  shader: &mut S,
  model: &M,
  super_sampling: u32,
) {
  let super_sampling = if super_sampling > 1 {
    Some(super_sampling_offsets(super_sampling))
  } else {
    None
  };

  for n in 0..model.face_count() {
    // 通过顶点Shader 计算顶点的位置
    let a = shader.vertext(model, n, 0);
    let b = shader.vertext(model, n, 1);
    let c = shader.vertext(model, n, 2);
    draw_triangle(img, depth_buff, a, b, c, shader, &super_sampling)
  }
}

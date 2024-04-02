use crate::{
  geometry::{Vec3, Vec4},
  image::Image,
  model::Model,
};

pub enum Fragment {
  Discard,
  Color(Vec3<f32>),
  Rgba(Vec4<f32>),
}

pub trait Shader {
  // 计算顶点在屏幕（渲染结果图像）上的位置
  fn vertext(&mut self, model: &Model, face: usize, nth_vert: usize) -> Vec3<f32>;
  // 对于三角形内部的每点调用fragment计算该点处的颜色
  fn fragment(
    &self,
    // 此点坐标
    pos: Vec3<f32>,
    // 此点处的质心坐标
    bar: Vec3<f32>,
  ) -> Fragment;
}

type Point = Vec3<f32>;

pub fn barycentric(a: Vec3<f32>, b: Vec3<f32>, c: Vec3<f32>, x: f32, y: f32) -> (f32, f32, f32) {
  let alpha = ((b.x - x) * (c.y - b.y) + (y - b.y) * (c.x - b.x))
    / ((b.x - a.x) * (c.y - b.y) + (a.y - b.y) * (c.x - b.x));
  let beta = ((c.x - x) * (a.y - c.y) + (y - c.y) * (a.x - c.x))
    / ((c.x - b.x) * (a.y - c.y) + (b.y - c.y) * (a.x - c.x));
  (alpha, beta, 1. - alpha - beta)
}

fn draw_triangle<S: Shader, I: Image>(
  img: &mut I,
  depth_buff: &mut Vec<f32>,
  a: Point,
  b: Point,
  c: Point,
  shader: &mut S,
  super_sampling: bool,
) {
  let min_x = a.x.min(b.x).min(c.x) as u32;
  let max_x = a.x.max(b.x).max(c.x).min((img.width() - 1) as f32) as u32;
  let min_y = a.y.min(b.y).min(c.y) as u32;
  let max_y = a.y.max(b.y).max(c.y).min((img.height() - 1) as f32) as u32;
  let sub_pix_offset = [(-0.25, -0.25), (0.75, 0.75), (-0.25, 0.75), (0.25, -0.75)];
  if super_sampling {
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
          match shader.fragment(p, Vec3::new(alpha, beta, gamma)) {
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
          let index = (y * img.height() + x) as usize;
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
        let index = (y * img.height() + x) as usize;
        if p.z > depth_buff[index] {
          // 通过Fragment shader 计算每个像素的颜色
          match shader.fragment(p, Vec3::new(alpha, beta, gamma)) {
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

pub fn render<S: Shader, I: Image>(
  img: &mut I,
  depth_buff: &mut Vec<f32>,
  shader: &mut S,
  model: &Model,
  super_sampling: bool,
) {
  for n in 0..model.face_count() {
    // 通过顶点Shader 计算顶点的位置
    let a = shader.vertext(model, n, 0);
    let b = shader.vertext(model, n, 1);
    let c = shader.vertext(model, n, 2);
    draw_triangle(img, depth_buff, a, b, c, shader, super_sampling)
  }
}

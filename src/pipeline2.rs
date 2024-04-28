use crate::{
  geometry::{Vec3, Vec4},
  image::Image,
  model::Object,
  pipeline::Fragment,
};

pub trait Shader<M: crate::model::Model> {
  // 计算顶点在屏幕（渲染结果图像）上的位置
  fn vertext(&mut self, model: &M, face: usize, nth_vert: usize) -> Vec4<f32>;
  // 对于三角形内部的每点调用fragment计算该点处的颜色
  fn fragment(
    &self,
    // 此点坐标
    pos: Vec3<f32>,
    // 此点处的质心坐标
    bar: Vec3<f32>,
  ) -> Fragment;
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
  println!("{:?}",offsets)
}

fn draw_triangle<M: crate::model::Model, S: Shader<M>, I: Image>(
  img: &mut I,
  depth_buff: &mut Vec<f32>,
  a: Vec4<f32>,
  b: Vec4<f32>,
  c: Vec4<f32>,
  shader: &mut S,
  super_sampling: &Option<Vec<(f32,f32)>>,
) {
  let wa = a.w;
  let wb = b.w;
  let wc = c.w;
  let a = a.to_3d_point();
  let b = b.to_3d_point();
  let c = c.to_3d_point();
  let min_x = a.x.min(b.x).min(c.x) as u32;
  let max_x = a.x.max(b.x).max(c.x).min((img.width() - 1) as f32) as u32;
  let min_y = a.y.min(b.y).min(c.y) as u32;
  let max_y = a.y.max(b.y).max(c.y).min((img.height() - 1) as f32) as u32;
  let sub_pix_offset = [(-0.25, -0.25), (0.75, 0.75), (-0.25, 0.75), (0.25, -0.75)];

  if let Some(sub_pix_offset)= super_sampling {
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
          match shader.fragment(p, Vec3::new(alpha / wa / k, beta / wb / k, gamma / wc / k)) {
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
          match shader.fragment(p, Vec3::new(alpha / wa / k, beta / wb / k, gamma / wc / k)) {
            Fragment::Color(c) => {
              depth_buff[index] = p.z;
              if p.z >= -1.1 && p.z <= 1.1 {
                img.set_rgb(x, y, c * 2.);
              } else {
                //img.set_rgb(x, y, c);
              }
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
  super_sampling: u32
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

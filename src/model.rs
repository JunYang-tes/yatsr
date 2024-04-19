use std::collections::HashMap;
use std::io::*;
use std::path::Path;

//https://en.wikipedia.org/wiki/Wavefront_.obj_file
use crate::geometry::Vec3;

pub trait Model {
  fn vert_count(&self) -> usize;
  fn face_count(&self) -> usize;
  fn vert(&self, face: usize, nth_vert: usize) -> Vec3<f32>;
  fn normal(&self, face: usize, nth_vert: usize) -> Vec3<f32>;
  fn normal_of_face(&self, face: usize) -> Vec3<f32>;
  fn texture_coord(&self, face: usize, nth_vert: usize) -> Vec3<f32>;
}

pub struct Object {
  // 下面三个索引从1起有意义，obj文件定义的索引是从1起的
  verts: Vec<Vec3<f32>>,
  texture_coords: Vec<Vec3<f32>>,
  vert_normals: Vec<Vec3<f32>>,

  vert_normal_idx: Vec<Vec<i32>>,
  face_vert_idx: Vec<Vec<i32>>,
  face_texture_idx: Vec<Vec<i32>>,
  // 某个顶点共享的面
  faces_of_vert: HashMap<usize, Vec<usize>>,
  computed_vert_normals: Vec<Vec3<f32>>,
}

impl Object {
  pub fn vert_count(&self) -> usize {
    self.verts.len() - 1
  }
  pub fn face_count(&self) -> usize {
    self.face_vert_idx.len()
  }
  pub fn has_normal_vector(&self) -> bool {
    self.vert_normals.len() > 1
  }
  pub fn normalize_verts(&mut self) {
    let first = self.verts[0];
    let mut min_x = first.x;
    let mut max_x = first.x;
    let mut min_y = first.y;
    let mut max_y = first.y;
    let mut min_z = first.z;
    let mut max_z = first.z;
    self.verts.iter().for_each(|v| {
      min_x = min_x.min(v.x);
      max_x = max_x.max(v.x);
      min_y = min_y.min(v.y);
      max_y = max_y.max(v.y);
      min_z = min_z.min(v.z);
      max_z = max_z.max(v.z);
    });
    let x = max_x - min_x;
    let y = max_y - min_y;
    let z = max_z - min_z;
    if max_x <= 1. && min_x >= -1. && max_y <= 1. && max_y >= -1. && max_z <= 1. && max_z >= -1. {
      return;
    }

    self.verts.iter_mut().for_each(|v| {
      if max_x > 1. || min_x < -1. {
        v.x = (v.x - min_x) / x * 2. - 1.0;
      }
      if max_y > 1. || min_y < -1. {
        v.y = (v.y - min_y) / y * 2. - 1.0;
      }
      if max_z > 1. || min_z < -1. {
        v.z = (v.z - min_z) / z * 2. - 1.0;
      }
    });
    //
  }
  fn get_vertex_index(&self, v_idx: i32) -> usize {
    if v_idx >= 0 {
      v_idx as usize
    } else {
      let last = self.verts.len() - 1;
      (last as i32 + v_idx) as usize
    }
  }
  pub fn verts_of_face(&self, idx: usize) -> Vec<Vec3<f32>> {
    self.face_vert_idx[idx]
      .iter()
      .map(|v_idx| {
        let v_idx = *v_idx;
        self.verts[self.get_vertex_index(v_idx)]
      })
      .collect()
  }
  pub fn texture_coords_of_face(&self, idx: usize) -> Vec<Vec3<f32>> {
    self.face_texture_idx[idx]
      .iter()
      .map(|v_idx| {
        let v_idx = *v_idx;
        let v_idx = if v_idx >= 0 {
          v_idx as usize
        } else {
          let last = self.texture_coords.len() - 1;
          (last as i32 - v_idx) as usize
        };
        self.texture_coords[v_idx].clone()
      })
      .collect()
  }
  pub fn texture_coord(&self, face: usize, nth_vert: usize) -> Vec3<f32> {
    let v_idx = self.face_texture_idx[face][nth_vert];
    let v_idx = if v_idx >= 0 {
      v_idx as usize
    } else {
      let last = self.texture_coords.len() - 1;
      (last as i32 - v_idx) as usize
    };
    self.texture_coords[v_idx]
  }
  pub fn vert(&self, face: usize, nth_vert: usize) -> Vec3<f32> {
    let v_idx = self.face_vert_idx[face][nth_vert];
    let v_idx = if v_idx >= 0 {
      v_idx as usize
    } else {
      let last = self.verts.len() - 1;
      (last as i32 - v_idx) as usize
    };
    self.verts[v_idx]
  }
  fn get_normal_index(&self, face: usize, vert: usize) -> usize {
    let idx = self.vert_normal_idx[face][vert];
    if idx >= 0 {
      idx as usize
    } else {
      let last = self.vert_normals.len() - 1;
      (last as i32 + idx) as usize
    }
  }
  pub fn normal(&self, face: usize, nth_vert: usize) -> Vec3<f32> {
    if self.has_normal_vector() {
      self.vert_normals[self.get_normal_index(face, nth_vert)]
    } else {
      self.computed_vert_normals[face * 3 + nth_vert]
    }
  }
  pub fn from_file<P: AsRef<Path>>(file: P) -> std::io::Result<Object> {
    let mut verts = vec![Vec3::new(0., 0., 0.)];
    let mut texture_coords = vec![Vec3::new(0., 0., 0.)];
    let mut vert_normals = vec![Vec3::new(0., 0., 0.)];

    let mut face_vert_idx = Vec::new();
    let mut face_texture_idx = Vec::new();
    let mut vert_normal_idx = Vec::new();
    let mut faces_of_vert = HashMap::new();
    let file = std::fs::File::open(file)?;
    BufReader::new(file).lines().for_each(|line| {
      if let Ok(line) = line {
        if line.starts_with("v ") {
          verts.push(parse_vert_or_vn(line));
        } else if line.starts_with("vt") {
          texture_coords.push(parse_vt(line));
        } else if line.starts_with("vn") {
          vert_normals.push(parse_vert_or_vn(line));
        } else if line.starts_with("f") {
          let (verts, vts, vns) = parse_face(line);
          let face_idx = face_vert_idx.len();
          for vertex_idx in &verts {
            // 为了简单起见，假设所有顶点已经读取到了verts里面
            let vertex_idx = if *vertex_idx > 0 {
              *vertex_idx as usize
            } else {
              let last = (verts.len() - 1) as i32;
              (last + (*vertex_idx as i32)) as usize
            };
            let faces = faces_of_vert.entry(vertex_idx).or_insert(Vec::new());
            faces.push(face_idx)
          }
          face_vert_idx.push(verts);
          face_texture_idx.push(vts);
          vert_normal_idx.push(vns);
        }
      } else if let Err(e) = line {
        eprintln!("Readline error:{:?}", e);
      }
    });

    let mut m = Object {
      verts,
      vert_normal_idx,
      vert_normals,
      face_vert_idx,
      face_texture_idx,
      texture_coords,
      faces_of_vert,
      computed_vert_normals: vec![],
    };
    //有些模型里没有定点的法响亮，那么计算出所有的顶点法向量备用
    if !m.has_normal_vector() {
      m.compute_vertex_normal();
    }
    Ok(m)
  }
  pub fn normal_of_face(&self, face: usize) -> Vec3<f32> {
    let verts = self.verts_of_face(face);
    return (verts[1] - verts[0])
      .cross_product(verts[2] - verts[0])
      .normalize();
  }
  fn compute_vertex_normal(&mut self) {
    let mut computed_vert_normals = vec![Vec3::default(); self.face_count() * 3];
    for vertex_idx in 1..self.verts.len() {
      let faces = self.faces_of_vert.get(&(vertex_idx));
      if let Some(faces) = faces {
        let sum_vector = faces
          .iter()
          .map(|f| self.normal_of_face(*f))
          .reduce(|acc, v| acc + v)
          .unwrap();
        let normal = sum_vector * (1. / (faces.len() as f32));
        for face in faces {
          let vertex_index = &self.face_vert_idx[*face];
          let nth_vert = if vertex_idx == self.get_vertex_index(vertex_index[0]) {
            0
          } else if vertex_idx == self.get_vertex_index(vertex_index[1]) {
            1
          } else if vertex_idx == self.get_vertex_index(vertex_index[2]) {
            2
          } else {
            panic!("")
          };
          computed_vert_normals[face * 3 + nth_vert] = normal;
        }
      } else {
        println!("{}", vertex_idx);
        panic!("No faces found, this is a bug")
      }
    }
    self.computed_vert_normals = computed_vert_normals;
  }
}

impl Model for Object {
  fn vert_count(&self) -> usize {
    self.verts.len() - 1
  }

  fn face_count(&self) -> usize {
    self.face_vert_idx.len()
  }

  fn vert(&self, face: usize, nth_vert: usize) -> Vec3<f32> {
    let v_idx = self.face_vert_idx[face][nth_vert];
    let v_idx = if v_idx >= 0 {
      v_idx as usize
    } else {
      let last = self.verts.len() - 1;
      (last as i32 - v_idx) as usize
    };
    self.verts[v_idx]
  }

  fn normal(&self, face: usize, nth_vert: usize) -> Vec3<f32> {
    if self.has_normal_vector() {
      self.vert_normals[self.get_normal_index(face, nth_vert)]
    } else {
      self.computed_vert_normals[face * 3 + nth_vert]
    }
  }

  fn texture_coord(&self, face: usize, nth_vert: usize) -> Vec3<f32> {
    let v_idx = self.face_texture_idx[face][nth_vert];
    let v_idx = if v_idx >= 0 {
      v_idx as usize
    } else {
      let last = self.texture_coords.len() - 1;
      (last as i32 - v_idx) as usize
    };
    self.texture_coords[v_idx]
  }

  fn normal_of_face(&self, face: usize) -> Vec3<f32> {
    let verts = self.verts_of_face(face);
    return (verts[1] - verts[0])
      .cross_product(verts[2] - verts[0])
      .normalize();
  }
}

fn parse_vt(line: String) -> Vec3<f32> {
  let vs = line
    .trim_matches(|ch| ch == 'v' || ch == ' ' || ch == 't' || ch == 'n')
    .split(" ")
    .map(|i| {
      i.parse::<f32>()
        .expect(format!("Expect float number,got {:?}", i).as_str())
    })
    .collect::<Vec<_>>();
  Vec3::new(vs[0], vs[1], 0.)
}
fn parse_vert_or_vn(line: String) -> Vec3<f32> {
  let vs = line
    .trim_matches(|ch| ch == 'v' || ch == ' ' || ch == 't' || ch == 'n')
    .split(" ")
    .map(|i| {
      i.parse::<f32>()
        .expect(format!("Expect float number,got {:?}", i).as_str())
    })
    .collect::<Vec<_>>();
  if vs.len() < 3 {
    println!("line:{}", line);
  }
  Vec3::new(vs[0], vs[1], vs[2])
}
fn parse_face(line: String) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
  let parts = line
    // f v v v
    // f v/vt/vn xx/xx/xx xx/xx/xx
    // f v/vt v/vt v/vt
    // f v//vn v//vn v//vn
    .trim_matches(|ch| ch == 'f' || ch == ' ')
    .split(" ");
  let mut verts = vec![];
  let mut vts = vec![];
  let mut vert_normals = vec![];
  parts.for_each(|item| {
    let parts = item.split('/').collect::<Vec<_>>();
    match parts.len() {
      1 => {
        let v_idx = parts[0];
        verts.push(v_idx.parse::<i32>().expect("Expect i32 as a vert index"));
      }
      2 => {
        let v_idx = parts[0];
        let vt_idx = parts[1];
        verts.push(v_idx.parse::<i32>().expect("Expect i32 as a vert index"));
        vts.push(
          vt_idx
            .parse::<i32>()
            .expect("Expect i32 as a texture index"),
        );
      }
      3 => {
        let v_idx = parts[0];
        let vt_idx = parts[1];
        let vn_idx = parts[2];
        verts.push(v_idx.parse::<i32>().expect("Expect i32 as a vert index"));
        if !vt_idx.is_empty() {
          vts.push(
            vt_idx
              .parse::<i32>()
              .expect("Expect i32 as a texture index"),
          );
        }
        vert_normals.push(
          vn_idx
            .parse::<i32>()
            .expect("Expect i32 as vert normal vector index"),
        );
      }
      _ => {
        panic!("Invalid model file:{} ", line)
      }
    }
  });
  (verts, vts, vert_normals)
}

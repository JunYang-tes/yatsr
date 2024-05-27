// 切空间法向量与模型变换


use yatsr::prelude::*;

struct MyShader {
  texture: Texture,
  varying_tangent: Vec3<f32>,
  varying_bitangent: Vec3<f32>,
  varying_normals: [Vec3<f32>;3],
  varying_uv: [Vec3<f32>;3],
  rotate: f32,
}
impl<M: Model> pipeline2::Shader<M> for MyShader {
  fn vertext(&mut self, model: &M, face: usize, nth_vert: usize) -> Vec4<f32> {
    let rotate = transform::rotate_y(self.rotate * 3.14/180.);
    if nth_vert == 0 {
      let p1 = &rotate * &model.vert(face, 0);
      let p2 = &rotate * &model.vert(face, 1);
      let p3 = &rotate * &model.vert(face, 2);
      let e1 = p2 - p1;
      let e2 = p3 - p1;
      let uv1 = model.texture_coord(face, 0);
      let uv2 = model.texture_coord(face, 1);
      let uv3 = model.texture_coord(face, 2);
      let v1 = uv1.y;
      let v2 = uv2.y;
      let v3 = uv3.y;
      let u1 = uv1.x;
      let u2 = uv2.x;
      let u3 = uv3.x;
      // let a = v2 - v1;
      // let b = u2 - u1;
      // let c = v3 - v1;
      // let d = u3 - u1;

      let a = u2-u1;
      let b = v2-v1;
      let c = u3-u1;
      let d = v3-v1;
      let k = a * d - c * b;
      // 因为没有实现2阶矩阵，所以把它嵌入4阶矩阵中计算
      #[rustfmt::skip]
      let m1 = Mat4([
        d / k,     -b / k, 0., 0.,
        -c / k,   a / k,   0., 0.,
        0.,          0.,   0., 0.,
        0.,          0.,   0., 0.,
      ]);
      #[rustfmt::skip]
      let m2 = Mat4([
        e1.x, e1.y, e1.z, 0., 
        e2.x, e2.y, e2.z, 0., 
        0., 0., 0., 0., 
        0., 0., 0., 0.,
      ]);
      let m3 = &m1 * &m2;
      self.varying_tangent = m3.row(0).to_3d_vector().normalize();
      self.varying_bitangent = m3.row(1).to_3d_vector().normalize();
    }
    self.varying_normals[nth_vert] = model.normal(face,nth_vert);
    self.varying_uv [nth_vert] = model.texture_coord(face,nth_vert);
    &rotate * Vec4::from_point(&model.vert(face, nth_vert))
  }

  fn fragment(&self, info: pipeline2::FragmentInfo) -> Fragment {
      #[allow(non_snake_case)]
      let N = info.barycentric_interpolate(&self.varying_normals).normalize();
      let invert_transpose = transform::rotate_y(-self.rotate*3.14/180.).transpose();
      #[allow(non_snake_case)]
      let N = &invert_transpose * &N;
      #[allow(non_snake_case)]
      let T = self.varying_tangent;
      #[allow(non_snake_case)]
      let B = self.varying_bitangent;



      // 以下，未使用bitangent,渲染结果稍有差异
      // #[allow(non_snake_case)]
      // let T = self.varying_tangant.cross_product(N).normalize();
      // #[allow(non_snake_case)]
      // let B = T.cross_product(N);



      let tbn = Mat4([
          T.x,B.x,N.x,0.,
          T.y,B.y,N.y,0.,
          T.z,B.z,N.z,0.,
          0., 0. ,0. ,1.,
      ]);
      let uv = info.barycentric_interpolate(&self.varying_uv);
      let n = self.texture.get(uv.x,uv.y);
      let n = n*2. - Vec3::new(1.,1.,1.);
      let normal = (&tbn * &n).normalize();
      let light = Vec3::new(1.,0.,0.).normalize();
      Fragment::Color(Vec3::new(1.,1.,1.) * (light*normal).max(0.))
  }
}

fn main() {
  sdl::one_frame("Tanget", 500, 500, |mut img| {
    let mut depth = vec![f32::MIN; img.width() as usize * img.height() as usize];
    let model = Object::from_file("./models/diablo/diablo3_pose.obj").unwrap();
    let mut shader = MyShader{
        texture: Texture::neareat(util::load_image(
          "./models/diablo/diablo3_pose_nm_tangent.tga",
        )),
        rotate: 45.,
        varying_uv: [Vec3::default(), Vec3::default(), Vec3::default()],
        varying_normals: [Vec3::default(), Vec3::default(), Vec3::default()],
        varying_tangent:Vec3::default(),
        varying_bitangent:Vec3::default(),
    };

    pipeline2::render(
      &mut img,
      &mut depth,
      &mut shader ,
      &model,
      0,
    );
    save_image("tangent.ppm",&img,PPM).unwrap();
  })
}

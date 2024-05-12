use std::cell::{RefCell, RefMut};

use yatsr::prelude::*;

struct MyShader {
  texture: Texture,
  img:RefCell<PixImage>,
  varying_uv: [Vec3<f32>; 3],
  varying_normals:[Vec3<f32>;3],
  varying_tangant: Vec3<f32>,
  varying_p :Vec3<f32>,
  varying_q :Vec3<f32>,
  mat: Mat4,
}
impl<M: Model> pipeline2::Shader<M> for MyShader {
  fn vertext(&mut self, model: &M, face: usize, nth_vert: usize) -> Vec4<f32> {
    let p = Vec4::from_point(&model.vert(face, nth_vert));
    if nth_vert == 0 {
      let p1 = model.vert(face, 0);
      let p2 = model.vert(face, 1);
      let p3 = model.vert(face, 2);
      let uv1 = model.texture_coord(face, 0);
      let uv2 = model.texture_coord(face, 1);
      let uv3 = model.texture_coord(face, 2);
      let e1 = p2 - p1;
      let e2 = p3 - p1;
    self.varying_p=e1.normalize();
    self.varying_q=e2.normalize();
      let a = uv2.y - uv1.y;
      let b = uv2.x - uv1.x;
      let c = uv3.y - uv1.y;
      let d = uv3.x - uv1.x;
      let k = a * d - c * b;
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

      let t = m3.row(1);
      let t = Vec3::new(t.x, t.y, t.z).normalize();
      self.varying_tangant = t;
    }
    self.varying_uv[nth_vert] = model.texture_coord(face, nth_vert);
    self.varying_normals[nth_vert]=model.normal(face,nth_vert);
    &self.mat * p
  }

  fn fragment(&self, info: pipeline2::FragmentInfo) -> Fragment {
    let uv = info.barycentric_interpolate(&self.varying_uv);
    // 原本的法向量
    let n = info.barycentric_interpolate(&self.varying_normals);
    //return Fragment::Color(n);
    let t = self.varying_tangant.cross_product(n).normalize();
    let b = n.cross_product(t);
    let tbn = Mat4([
      t.x, t.y, t.z, 0., b.x, b.y, b.z, 0., n.x, n.y, n.z, 0., 0., 0., 0., 1.,
    ]).transpose();
    let pqn = Mat4([
      self.varying_p.x,self.varying_p.y,self.varying_p.z,0.,
      self.varying_q.x,self.varying_q.y,self.varying_q.z,0.,
      n.x, n.y, n.z, 0., 0., 0., 0., 1.,
    ]).transpose();
    // tbn 空间的扰动后的法向量
    let n = self.texture.get(uv.x, uv.y) * 2. - Vec3::new(1., 1., 1.);
    let abc = &(&pqn.invert() * &tbn) * &n;
    let abc= (abc+ Vec3::new(1.,1.,1.))*0.5;
    let mut c = self.img.borrow_mut();
    write(c,uv.x*1024.,uv.y*1024.,Vec3::new(abc.x,abc.y,abc.z)
    );
    Fragment::Discard
    //Fragment::Color(&pqn.invert() * &n)
  }
}

fn write(mut img: RefMut<PixImage>,x:f32,y:f32,color:Vec3<f32>) {
    for i in [
        (x.floor() as u32, y.floor() as u32),
        (x.ceil() as u32,y.ceil() as u32),
    (x.ceil() as u32,y.floor() as u32),
        (x.floor() as u32, y.floor() as u32),
        (x.floor() as u32,y.ceil() as u32)
    ] {
        let c = img.get(i.0,i.1);
        if c.x==0 && c.y==0 && c.z ==0 {
            img.set_rgb(i.0,i.1,color)
        }

    }

}

fn main() {
  sdl::one_frame("A", 1024, 1024, |mut img| {
    let mut depth = vec![f32::MIN; img.width() as usize * img.height() as usize];
    let model = Object::from_file("/home/yj/projects/sync/tinyrenderer/african_head.obj").unwrap();
    let out = PixImage::new(1024,1024);
    let mut shader = MyShader{

          img:RefCell::new(out),
        texture: Texture::new(util::load_image(
          "/home/yj/Downloads/african_head_nm_tangent.tga",
        )),
        varying_uv: [Vec3::default(), Vec3::default(), Vec3::default()],
        varying_normals: [Vec3::default(), Vec3::default(), Vec3::default()],
        varying_tangant:Vec3::default(),
        varying_q:Vec3::default(),
        varying_p:Vec3::default(),
        mat: Transform::new()
          //.rotate_y((180. - 30.) * 3.14 / 180.)
          .build(),
    };

    pipeline2::render(
      &mut img,
      &mut depth,
      &mut shader ,
      &model,
      0,
    );
    shader.mat = Transform::new()
        .rotate_y(90. * 3.14 / 180.)
        .build();

    pipeline2::render(
      &mut img,
      &mut depth,
      &mut shader ,
      &model,
      0,
    );
    shader.mat = Transform::new()
        .rotate_y(270. * 3.14 / 180.)
        .build();
    pipeline2::render(
      &mut img,
      &mut depth,
      &mut shader ,
      &model,
      0,
    );

    shader.mat = Transform::new()
        .rotate_x(90. * 3.14 / 180.)
        .build();
    pipeline2::render(
      &mut img,
      &mut depth,
      &mut shader ,
      &model,
      0,
    );

    let p =shader.img.get_mut();
    save_image("output_tangant.ppm",p,PPM);
  })
}

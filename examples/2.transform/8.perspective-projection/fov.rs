
fn perspective(fov:f32,aspect_ratio:f32,near:f32,far:f32) {
    let h = -near * (fov * 3.14/180./2.).tan();
    let top = h;
    let bottom = -h;
    let w = r*h;
    let left = -w;
    let right = w;

  #[rustfmt::skip]
    let m = Mat4([
      near, 0.,  0.,  0.,
      0.,near ,  0.,  0.,
      0.,0.,  near+far ,-far*near,
      0.,0.,  1.,  0.
    ]);
  &transform::orthographic(left, right, bottom, top, far, near) * &m
}

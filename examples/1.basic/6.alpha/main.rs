use yatsr::{
  file::save_image,
  geometry::Vec4,
  image::{Image, PixImage},
  image_encoder::PPM,
};

fn draw_cicle(img: &mut PixImage, cx: u32, cy: u32, radius: u32, color: Vec4<f32>) {
  for x in (cx - radius)..(cx + radius) {
    for y in (cy - radius)..(cy + radius) {
      let x = x as i32;
      let y = y as i32;
      let cx = cx as i32;
      let cy = cy as i32;
      if ((x - cx).pow(2) as i32 + (y - cy).pow(2) as i32 - radius.pow(2) as i32) < 0 {
        img.blending(x as u32, y as u32, color);
      }
    }
  }
}

fn main() {
  let mut img = PixImage::new(500, 500);
  for x in 0..500 {
    for y in 0..500 {
      img.set_rgba(x, y, yatsr::geometry::Vec4::new(1., 1., 1., 1.));
    }
  }
  let cx = 250;
  let cy = 300;
  let r = 100;
  draw_cicle(&mut img, cx, cy, r, Vec4::new(1., 0., 0., 0.5));
  draw_cicle(
    &mut img,
    cx - (r / 2),
    cy - (r as f32 * (3 as f32).sqrt() / 2.) as u32,
    r,
    Vec4::new(0., 1., 0., 0.5),
  );
  draw_cicle(
    &mut img,
    cx + (r / 2),
    cy - (r as f32 * (3 as f32).sqrt() / 2.) as u32,
    r,
    Vec4::new(0., 0., 1., 0.5),
  );
  save_image("alpha.ppm", &img, PPM);
}

use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, rect::Rect};

pub fn frame<F: FnMut((&mut [u8], u32, u32), f32) -> ()>(title: &str, w: u32, h: u32, mut draw: F) {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let window = video_subsystem
    .window(title, w, h)
    .position_centered()
    .build()
    .unwrap();
  let mut canvas = window.into_canvas().build().unwrap();
  let texture_creator = canvas.texture_creator();
  let mut texture = texture_creator
    .create_texture(
      PixelFormatEnum::RGBA32,
      sdl2::render::TextureAccess::Streaming,
      w,
      h,
    )
    .expect("Failed to create texture");
  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut fps = 0.;
  'running: loop {
    let start = std::time::Instant::now();
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'running,
        _ => {}
      }
    }
    texture
      .with_lock(Rect::new(0, 0, w, h), |data, _| {
        data.fill(0);
        let img = (data, w, h);
        draw(img, fps);
      })
      .unwrap();

    canvas
      .copy(&texture, Rect::new(0, 0, w, h), Rect::new(0, 0, w, h))
      .unwrap();

    canvas.present();
    let duration = start.elapsed();
    fps = 1000. / (duration.as_millis() as f32);
    //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }
}
pub fn one_frame<F: FnMut((&mut [u8], u32, u32)) -> ()>(title: &str, w: u32, h: u32, mut draw: F) {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let window = video_subsystem
    .window(title, w, h)
    .position_centered()
    .build()
    .unwrap();
  let mut canvas = window.into_canvas().build().unwrap();
  let texture_creator = canvas.texture_creator();
  let mut texture = texture_creator
    .create_texture(
      PixelFormatEnum::RGBA32,
      sdl2::render::TextureAccess::Streaming,
      w,
      h,
    )
    .expect("Failed to create texture");
  texture
    .with_lock(Rect::new(0, 0, w, h), |data, _| {
      data.fill(0);
      let img = (data, w, h);
      draw(img);
    })
    .unwrap();

  canvas
    .copy(&texture, Rect::new(0, 0, w, h), Rect::new(0, 0, w, h))
    .unwrap();
  canvas.present();
  let mut event_pump = sdl_context.event_pump().unwrap();
  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'running,
        _ => {}
      }
    }
  }
}

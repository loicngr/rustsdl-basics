extern crate sdl2;

use sdl2::image::{InitFlag, LoadTexture};
use std::path::Path;
use std::time::Duration;
use sdl2::rect::Rect;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let _sdl2_context_image = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("test kappa image", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let png = Path::new("assets/kappa.png");
    let texture = texture_creator.load_texture(png)?;

    let mut texture_rect = Rect::new(0, 0, 400, 300);
    canvas.copy(&texture, None, texture_rect);
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Option::Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'main_loop,
                _ => {}
            }
        }


        let new_y = texture_rect.y() + 1;
        if (new_y + 300) < canvas.window().size().1 as i32 {
            canvas.clear();
            texture_rect.set_y(new_y);
            canvas.copy(&texture, None, texture_rect);
        } else {
            let new_x = texture_rect.x() + 1;

            if (new_x + 400) < canvas.window().size().0 as i32 {
                canvas.clear();
                texture_rect.set_x(new_x);
                canvas.copy(&texture, None, texture_rect);
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

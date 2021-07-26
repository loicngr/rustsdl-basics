extern crate sdl2;

use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use std::path::Path;
use std::time::Duration;

const TILES_SIZE: u32 = 16;
const TILES_MARGIN: u32 = 1;

const TILE_ITEM_DIRT_POS: u32 = ((TILES_SIZE + TILES_MARGIN) * 6);

/// Draw map in canvas
fn draw_map(canvas: &mut WindowCanvas, texture_game_sheet: &Texture) {
    let mut canvas_pos_y_count = 0;
    let mut canvas_pos_x_count = 0;
    let (canvas_width, canvas_height) = canvas.window().size();

    // Boucle dans les pixels en Y de la window
    for canvas_pos_y in 0..canvas_height {
        if canvas_pos_y == canvas_pos_y_count {
            // Boucle dans les pixels en X de la window
            for canvas_pos_x in 0..canvas_width {
                if canvas_pos_x == canvas_pos_x_count {
                    canvas.copy(
                        &texture_game_sheet,
                        Rect::new(TILE_ITEM_DIRT_POS as i32, 0, TILES_SIZE, TILES_SIZE),
                        Rect::new(
                            canvas_pos_x as i32,
                            canvas_pos_y as i32,
                            TILES_SIZE,
                            TILES_SIZE,
                        ),
                    );

                    canvas_pos_x_count += TILES_SIZE;
                }
            }

            canvas_pos_x_count = 0;
            canvas_pos_y_count += TILES_SIZE;
        }
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let _sdl2_context_image = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("unknowsystem", 1200, 720)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    canvas.clear();

    let game_sheet = Path::new("assets/Spritesheet/roguelikeSheet_transparent.png");
    let texture_game_sheet = texture_creator.load_texture(game_sheet)?;

    // Draw de la map
    draw_map(&mut canvas, &texture_game_sheet);

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

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

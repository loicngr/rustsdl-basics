extern crate sdl2;

use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use std::net::Shutdown::Read;
use std::path::Path;
use std::time::Duration;

const WINDOW_SIZES: (u32, u32) = (1200, 720);

const TILES_SIZE: u32 = 16;
const TILES_MARGIN: u32 = 1;

const TILE_ITEM_DIRT_POS: (i32, i32) = (
    ((TILES_SIZE + TILES_MARGIN) * 6) as i32,
    ((TILES_SIZE + TILES_MARGIN) * 18) as i32,
);
const TILE_ITEM_PLAYER_TURN_LEFT_POS: i32 = ((TILES_SIZE + TILES_MARGIN) * 23) as i32;
const TILE_ITEM_PLAYER_TURN_RIGHT_POS: i32 = ((TILES_SIZE + TILES_MARGIN) * 26) as i32;
const TILE_ITEM_PLAYER_TURN_UP_POS: i32 = ((TILES_SIZE + TILES_MARGIN) * 25) as i32;
const TILE_ITEM_PLAYER_TURN_DOWN_POS: i32 = ((TILES_SIZE + TILES_MARGIN) * 24) as i32;

struct PlayerState {
    x: i32,
    y: i32,
    entity: Rect,
}

struct LevelState {
    x: i32,
    y: i32,
    texture: (i32, i32),
}

impl PlayerState {
    fn update_x(&mut self, new_x: i32) {
        self.x = self.entity.x();
        self.entity.set_x(new_x);
    }

    fn update_y(&mut self, new_y: i32) {
        self.y = self.entity.y();
        self.entity.set_y(new_y);
    }
}

struct AppSate {
    player: PlayerState,
    level: Vec<LevelState>,
}

impl AppSate {
    fn new() -> AppSate {
        let player_x = 0 as i32;
        let player_y = 0 as i32;

        AppSate {
            player: PlayerState {
                x: player_x,
                y: player_y,
                entity: Rect::new(player_x, player_y, TILES_SIZE, TILES_SIZE),
            },
            level: vec![],
        }
    }
}

/// Draw map in canvas
fn draw_map(app_state: &mut AppSate, canvas: &mut WindowCanvas, sheet: &Texture) {
    let mut canvas_pos_y_count = 0;
    let mut canvas_pos_x_count = 0;
    let (canvas_width, canvas_height) = canvas.window().size();
    app_state.level.clear();

    // Boucle dans les pixels en Y de la window
    for canvas_pos_y in 0..canvas_height {
        if canvas_pos_y == canvas_pos_y_count {
            // Boucle dans les pixels en X de la window
            for canvas_pos_x in 0..canvas_width {
                if canvas_pos_x == canvas_pos_x_count {
                    app_state.level.push(LevelState {
                        x: canvas_pos_x as i32,
                        y: canvas_pos_y as i32,
                        texture: TILE_ITEM_DIRT_POS,
                    });
                    canvas.copy(
                        &sheet,
                        Rect::new(
                            TILE_ITEM_DIRT_POS.0,
                            TILE_ITEM_DIRT_POS.1,
                            TILES_SIZE,
                            TILES_SIZE,
                        ),
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

/// Draw player in canvas
fn draw_player(app_state: &mut AppSate, canvas: &mut WindowCanvas, sheet: &Texture) {
    canvas.copy(
        &sheet,
        Rect::new(
            TILE_ITEM_PLAYER_TURN_DOWN_POS as i32,
            0,
            TILES_SIZE,
            TILES_SIZE,
        ),
        app_state.player.entity,
    );

    let player_x = app_state.player.entity.x();
    let player_y = app_state.player.entity.y();

    let old_player_x = app_state.player.x;
    let old_player_y = app_state.player.y;

    // Player move from x position
    // so we need to redraw behind player
    if old_player_x != player_x {
        if let Some(level_position) = app_state
            .level
            .iter()
            .position(|lvl| lvl.x == old_player_x)
        {
            let level = app_state.level.get(level_position).unwrap();
            canvas.copy(
                &sheet,
                Rect::new(level.texture.0, level.texture.1, TILES_SIZE, TILES_SIZE),
                Rect::new(old_player_x, player_y, TILES_SIZE, TILES_SIZE),
            );
        }
    }
    // Player move from y position
    // so we need to redraw behind player
    else if old_player_y != player_y {
        if let Some(level_position) = app_state
            .level
            .iter()
            .position(|lvl| lvl.y == old_player_y)
        {
            let level = app_state.level.get(level_position).unwrap();
            canvas.copy(
                &sheet,
                Rect::new(level.texture.0, level.texture.1, TILES_SIZE, TILES_SIZE),
                Rect::new(player_x, old_player_y, TILES_SIZE, TILES_SIZE),
            );
        }
    }

    // Reset old player position
    app_state.player.update_x(player_x);
    app_state.player.update_y(player_y);
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let _sdl2_context_image = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let video_subsystem = sdl_context.video()?;
    let mut app_state = AppSate::new();

    let window = video_subsystem
        .window("unknowsystem", WINDOW_SIZES.0, WINDOW_SIZES.1)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    canvas.clear();

    let game_sheet = Path::new("assets/tilemap/tilemap.png");
    let texture_game_sheet = texture_creator.load_texture(game_sheet)?;

    // Draw de la map
    draw_map(&mut app_state, &mut canvas, &texture_game_sheet);

    // Draw du player
    draw_player(&mut app_state, &mut canvas, &texture_game_sheet);

    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    'main_loop: loop {
        for event in event_pump.poll_iter() {
            let player_entity = &app_state.player.entity;

            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Option::Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'main_loop,
                sdl2::event::Event::KeyDown {
                    keycode: Option::Some(sdl2::keyboard::Keycode::Down),
                    ..
                } => {
                    let new_pos = player_entity.y() + TILES_SIZE as i32;
                    if new_pos < (canvas.window().size().1 - TILES_SIZE) as i32 {
                        app_state.player.update_y(new_pos);
                        draw_player(&mut app_state, &mut canvas, &texture_game_sheet);
                    }
                }
                sdl2::event::Event::KeyDown {
                    keycode: Option::Some(sdl2::keyboard::Keycode::Up),
                    ..
                } => {
                    let new_pos = player_entity.y() - TILES_SIZE as i32;
                    if (player_entity.y() - TILES_SIZE as i32) > 0 {
                        app_state.player.update_y(new_pos);
                        draw_player(&mut app_state, &mut canvas, &texture_game_sheet);
                    }
                }
                sdl2::event::Event::KeyDown {
                    keycode: Option::Some(sdl2::keyboard::Keycode::Left),
                    ..
                } => {
                    let new_pos = player_entity.x() - TILES_SIZE as i32;
                    if (player_entity.x() - TILES_SIZE as i32) > 0 {
                        app_state.player.update_x(new_pos);
                        draw_player(&mut app_state, &mut canvas, &texture_game_sheet);
                    }
                }
                sdl2::event::Event::KeyDown {
                    keycode: Option::Some(sdl2::keyboard::Keycode::Right),
                    ..
                } => {
                    let new_pos = player_entity.x() + TILES_SIZE as i32;
                    if (player_entity.x() + TILES_SIZE as i32)
                        < (canvas.window().size().0 - TILES_SIZE) as i32
                    {
                        app_state.player.update_x(new_pos);
                        draw_player(&mut app_state, &mut canvas, &texture_game_sheet);
                    }
                }
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

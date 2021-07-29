extern crate sdl2;

use rand::Rng;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use std::net::Shutdown::Read;
use std::path::Path;
use std::time::Duration;

const WINDOW_SIZES: (u32, u32) = (640, 640);

const TILES_SIZE: u32 = 16;
const TILES_MARGIN: u32 = 1;

const TILE_ITEM_DIRT_POS: (i32, i32) = (
    ((TILES_SIZE + TILES_MARGIN) * 6) as i32,
    ((TILES_SIZE + TILES_MARGIN) * 18) as i32,
);
const TILE_ITEM_SAND_POS: (i32, i32) = (
    ((TILES_SIZE + TILES_MARGIN) * 8) as i32,
    ((TILES_SIZE + TILES_MARGIN) * 18) as i32,
);
const TILE_ITEM_PLAYER_TURN_LEFT_POS: i32 = ((TILES_SIZE + TILES_MARGIN) * 23) as i32;
const TILE_ITEM_PLAYER_TURN_RIGHT_POS: i32 = ((TILES_SIZE + TILES_MARGIN) * 26) as i32;
const TILE_ITEM_PLAYER_TURN_UP_POS: i32 = ((TILES_SIZE + TILES_MARGIN) * 25) as i32;
const TILE_ITEM_PLAYER_TURN_DOWN_POS: i32 = ((TILES_SIZE + TILES_MARGIN) * 24) as i32;

const MAP_SIZES: (i32, i32) = ((WINDOW_SIZES.0 as i32 * 4), (WINDOW_SIZES.1 as i32 * 4));

#[derive(Debug, Copy, Clone)]
struct PlayerCamera {
    sizes: (u32, u32),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum PlayerAnimation {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct PlayerState {
    x: i32,
    y: i32,
    animation: Option<PlayerAnimation>,
    entity: Rect,
    camera: PlayerCamera,
}

#[derive(Debug, Copy, Clone)]
struct LevelState {
    x: i32,
    y: i32,
    texture: (Rect, Rect),
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

#[derive(Debug)]
struct AppSate {
    player: PlayerState,
    level: Vec<LevelState>,
}

impl AppSate {
    fn new() -> AppSate {
        let player_x = 320 as i32;
        let player_y = 320 as i32;

        AppSate {
            player: PlayerState {
                x: player_x,
                y: player_y,
                animation: Some(PlayerAnimation::Down),
                entity: Rect::new(player_x, player_y, TILES_SIZE, TILES_SIZE),
                camera: PlayerCamera {
                    sizes: (WINDOW_SIZES.0 / 5, WINDOW_SIZES.1 / 5),
                },
            },
            level: vec![],
        }
    }
}

/// Generate the map and save it in vec
fn generate_map(app_state: &mut AppSate, canvas: &mut WindowCanvas, sheet: &Texture) {
    let mut map_pos_y_count = 0;
    let mut map_pos_x_count = 0;
    let (map_width, map_height) = MAP_SIZES;

    let start_x = 0;
    let end_x = map_width;

    let start_y = 0;
    let end_y = map_height;

    let mut rng = rand::thread_rng();
    for map_pos_y in start_y..end_y {
        if map_pos_y == map_pos_y_count {
            for map_pos_x in start_x..end_x {
                if map_pos_x == map_pos_x_count {
                    let random_number: bool = rng.gen();

                    let mut tile: (i32, i32);
                    if random_number {
                        tile = TILE_ITEM_DIRT_POS;
                    } else {
                        tile = TILE_ITEM_SAND_POS
                    }

                    let texture_src: Rect = Rect::new(tile.0, tile.1, TILES_SIZE, TILES_SIZE);
                    let texture_dst: Rect =
                        Rect::new(map_pos_x as i32, map_pos_y as i32, TILES_SIZE, TILES_SIZE);

                    app_state.level.push(LevelState {
                        x: map_pos_x as i32,
                        y: map_pos_y as i32,
                        texture: (texture_src, texture_dst),
                    });

                    map_pos_x_count += TILES_SIZE as i32;
                }
            }

            map_pos_x_count = 0;
            map_pos_y_count += TILES_SIZE as i32;
        }
    }
}

/// Draw map in canvas
fn draw_camera_map(app_state: &mut AppSate, canvas: &mut WindowCanvas, sheet: &Texture) {
    let start_x: i32 = app_state.player.x as i32 - (app_state.player.camera.sizes.0 as i32 / 2);
    let end_x: i32 = app_state.player.x as i32 + (app_state.player.camera.sizes.0 as i32 / 2);

    let start_y: i32 = app_state.player.y as i32 - (app_state.player.camera.sizes.0 as i32 / 2);
    let end_y: i32 = app_state.player.y as i32 + (app_state.player.camera.sizes.0 as i32 / 2);

    let mut view_pos_y_count: i32 = start_y;
    let mut view_pos_x_count: i32 = start_x;

    for view_pos_y in start_y..end_y {
        if view_pos_y == view_pos_y_count {
            for view_pos_x in start_x..end_x {
                if view_pos_x == view_pos_x_count {
                    let current_position = (view_pos_x, view_pos_y);
                    if let Some(find_level) = app_state
                        .level
                        .iter()
                        .find(|&lvl| lvl.x == current_position.0 && lvl.y == current_position.1)
                    {
                        let level_texture_src = find_level.texture.0.clone();
                        let level_texture_dst = find_level.texture.1.clone();
                        canvas.copy(&sheet, level_texture_src, level_texture_dst);
                    }

                    view_pos_x_count += TILES_SIZE as i32;
                }
            }

            view_pos_x_count = start_x;
            view_pos_y_count += TILES_SIZE as i32;
        }
    }
}

/// Draw player in canvas
fn draw_player(app_state: &mut AppSate, canvas: &mut WindowCanvas, sheet: &Texture) {
    let player_x = app_state.player.entity.x();
    let player_y = app_state.player.entity.y();

    let old_player_x = app_state.player.x;
    let old_player_y = app_state.player.y;

    let player_tile: i32 = {
        if PlayerAnimation::Up == app_state.player.animation.unwrap() {
            TILE_ITEM_PLAYER_TURN_UP_POS as i32
        } else if PlayerAnimation::Left == app_state.player.animation.unwrap() {
            TILE_ITEM_PLAYER_TURN_LEFT_POS as i32
        } else if PlayerAnimation::Right == app_state.player.animation.unwrap() {
            TILE_ITEM_PLAYER_TURN_RIGHT_POS as i32
        } else {
            TILE_ITEM_PLAYER_TURN_DOWN_POS as i32
        }
    };

    canvas.copy(
        &sheet,
        Rect::new(player_tile, 0, TILES_SIZE, TILES_SIZE),
        app_state.player.entity,
    );

    // Redraw map behind player
    if old_player_x != player_x || old_player_y != player_y {
        let current_position = (old_player_x, old_player_y);
        let find_level = app_state
            .level
            .iter()
            .find(|&lvl| lvl.x == current_position.0 && lvl.y == current_position.1)
            .expect("cant find level position for redraw map behind player.");
        let level_texture_src = find_level.texture.0.clone();
        let level_texture_dst = find_level.texture.1.clone();
        canvas.copy(&sheet, level_texture_src, level_texture_dst);
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

    // Generate map
    generate_map(&mut app_state, &mut canvas, &texture_game_sheet);

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
                        app_state.player.animation = Some(PlayerAnimation::Down);
                    }
                }
                sdl2::event::Event::KeyDown {
                    keycode: Option::Some(sdl2::keyboard::Keycode::Up),
                    ..
                } => {
                    let new_pos = player_entity.y() - TILES_SIZE as i32;
                    if (player_entity.y() - TILES_SIZE as i32) > 0 {
                        app_state.player.update_y(new_pos);
                        app_state.player.animation = Some(PlayerAnimation::Up);
                    }
                }
                sdl2::event::Event::KeyDown {
                    keycode: Option::Some(sdl2::keyboard::Keycode::Left),
                    ..
                } => {
                    let new_pos = player_entity.x() - TILES_SIZE as i32;
                    if (player_entity.x() - TILES_SIZE as i32) > 0 {
                        app_state.player.update_x(new_pos);
                        app_state.player.animation = Some(PlayerAnimation::Left);
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
                        app_state.player.animation = Some(PlayerAnimation::Right);
                    }
                }
                _ => {}
            }
        }

        draw_camera_map(&mut app_state, &mut canvas, &texture_game_sheet);
        draw_player(&mut app_state, &mut canvas, &texture_game_sheet);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

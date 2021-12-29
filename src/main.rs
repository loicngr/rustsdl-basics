extern crate sdl2;

use rand::Rng;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
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

const MAP_SIZES: (i32, i32) = ((WINDOW_SIZES.0 as i32), (WINDOW_SIZES.1 as i32));

#[derive(Debug, Copy, Clone)]
struct PlayerCamera {
    sizes: (u32, u32),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum PlayerDirection {
    None,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct PlayerState {
    x: i32,
    y: i32,
    direction: Option<PlayerDirection>,
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
                direction: Some(PlayerDirection::None),
                entity: Rect::new(player_x, player_y, TILES_SIZE, TILES_SIZE),
                camera: PlayerCamera {
                    sizes: WINDOW_SIZES,
                },
            },
            level: vec![],
        }
    }
}

/// Generate the map and save it in vec
fn generate_map(app_state: &mut AppSate, _canvas: &mut WindowCanvas, _sheet: &Texture) {
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

                    let tile: (i32, i32);
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

// fn draw_camera_map(app_state: &mut AppSate, canvas: &mut WindowCanvas, sheet: &Texture) {
//     let start_x: i32 = app_state.player.x as i32 - (app_state.player.camera.sizes.0 as i32 / 2);
//     let end_x: i32 = app_state.player.x as i32 + (app_state.player.camera.sizes.0 as i32 / 2);

//     let start_y: i32 = app_state.player.y as i32 - (app_state.player.camera.sizes.0 as i32 / 2);
//     let end_y: i32 = app_state.player.y as i32 + (app_state.player.camera.sizes.0 as i32 / 2);

//     let mut view_pos_y_count: i32 = start_y;
//     let mut view_pos_x_count: i32 = start_x;

//     for view_pos_y in start_y..end_y {
//         if view_pos_y == view_pos_y_count {
//             for view_pos_x in start_x..end_x {
//                 if view_pos_x == view_pos_x_count {
//                     let current_position = (view_pos_x, view_pos_y);

//                     if let Some(find_level) = find_level_width_position(app_state, current_position)
//                     {
//                         let level_texture_src = find_level.texture.0.clone();
//                         let level_texture_dst = find_level.texture.1.clone();
//                         canvas.copy(&sheet, level_texture_src, level_texture_dst).unwrap();
//                     }

//                     view_pos_x_count += TILES_SIZE as i32;
//                 }
//             }

//             view_pos_x_count = start_x;
//             view_pos_y_count += TILES_SIZE as i32;
//         }
//     }
// }

/// Draw map from player camera position
fn draw_map_from_camera(app_state: &mut AppSate, canvas: &mut WindowCanvas, sheet: &Texture) {
    let player_x = app_state.player.entity.x();
    let player_y = app_state.player.entity.y();

    let x_from = ((app_state.player.camera.sizes.0 / 2) as i32) - player_x;
    let x_to = app_state.player.camera.sizes.0 as i32;
    
    let y_from = ((app_state.player.camera.sizes.1 / 2) as i32) - player_y;
    let y_to = app_state.player.camera.sizes.1 as i32;

    for level in app_state.level.iter() {
        let level_texture_src = level.texture.0.clone();
        let level_texture_dst = level.texture.1.clone();

        if level.x >= x_from && level.x <= x_to || level.y >= y_from && level.y <= y_to {
            canvas.copy(&sheet, level_texture_src, level_texture_dst).unwrap();
        }
    }
}

/// Return current level item with player position in level
fn find_level_width_position(app_state: &AppSate, position: (i32, i32)) -> Option<&LevelState> {
    if let Some(find_level) = app_state
        .level
        .iter()
        .find(|&lvl| lvl.x == position.0 && lvl.y == position.1)
    {
        return Some(find_level);
    }

    None
}

/// Draw player in canvas
fn draw_player(app_state: &mut AppSate, canvas: &mut WindowCanvas, sheet: &Texture) {
    let player_x = app_state.player.entity.x();
    let player_y = app_state.player.entity.y();

    let old_player_x = app_state.player.x;
    let old_player_y = app_state.player.y;

    let player_tile: i32 = {
        if PlayerDirection::Up == app_state.player.direction.unwrap() {
            TILE_ITEM_PLAYER_TURN_UP_POS as i32
        } else if PlayerDirection::Left == app_state.player.direction.unwrap() {
            TILE_ITEM_PLAYER_TURN_LEFT_POS as i32
        } else if PlayerDirection::Right == app_state.player.direction.unwrap() {
            TILE_ITEM_PLAYER_TURN_RIGHT_POS as i32
        } else {
            TILE_ITEM_PLAYER_TURN_DOWN_POS as i32
        }
    };

    canvas.copy(
        &sheet,
        Rect::new(player_tile, 0, TILES_SIZE, TILES_SIZE),
        app_state.player.entity,
    ).unwrap();

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
        canvas.copy(&sheet, level_texture_src, level_texture_dst).unwrap();
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
    draw_map_from_camera(&mut app_state, &mut canvas, &texture_game_sheet);

    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    'main_loop: loop {
        // canvas.clear(); // < perf issue (need to redraw map)
        let player_entity = app_state.player.entity;
        let player_current_pos = (player_entity.x(), player_entity.y());

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Option::Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'main_loop,
                // Down arrow key
                sdl2::event::Event::KeyDown {
                    keycode: Option::Some(sdl2::keyboard::Keycode::Down),
                    ..
                } => {
                    let pos_y = player_current_pos.1 + TILES_SIZE as i32;
                    if let Some(_) = find_level_width_position(
                        &app_state,
                        (
                            player_current_pos.0,
                            pos_y
                        ),
                    ) {
                        app_state.player.update_y(pos_y);
                        app_state.player.direction = Some(PlayerDirection::Down);
                    }
                }
                // Up arrow key
                sdl2::event::Event::KeyDown {
                    keycode: Option::Some(sdl2::keyboard::Keycode::Up),
                    ..
                } => {
                    let pos_y = player_current_pos.1 - TILES_SIZE as i32;
                    if let Some(_) = find_level_width_position(
                        &app_state,
                        (
                            player_current_pos.0,
                            pos_y
                        ),
                    ) {
                        app_state.player.update_y(pos_y);
                        app_state.player.direction = Some(PlayerDirection::Up);
                    }
                }
                // Left arrow key
                sdl2::event::Event::KeyDown {
                    keycode: Option::Some(sdl2::keyboard::Keycode::Left),
                    ..
                } => {
                    let pos_x = player_current_pos.0 - TILES_SIZE as i32;
                    if let Some(_) = find_level_width_position(
                        &app_state,
                        (
                            pos_x,
                            player_current_pos.1,
                        ),
                    ) {
                        app_state.player.update_x(pos_x);
                        app_state.player.direction = Some(PlayerDirection::Left);
                    }
                }
                // Right arrow key
                sdl2::event::Event::KeyDown {
                    keycode: Option::Some(sdl2::keyboard::Keycode::Right),
                    ..
                } => {
                    let pos_x = player_current_pos.0 + TILES_SIZE as i32;
                    if let Some(_) = find_level_width_position(
                        &app_state,
                        (
                            pos_x,
                            player_current_pos.1,
                        ),
                    ) {
                        app_state.player.update_x(pos_x);
                        app_state.player.direction = Some(PlayerDirection::Right);
                    }
                }
                _ => {}
            }
        }

        
        if Some(PlayerDirection::None) != app_state.player.direction {
            // draw_camera_map(&mut app_state, &mut canvas, &texture_game_sheet);
            // app_state.player.direction = Some(PlayerDirection::None);
        }
        draw_player(&mut app_state, &mut canvas, &texture_game_sheet);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

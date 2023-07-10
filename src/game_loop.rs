extern crate device_query;
use device_query::{ DeviceState, Keycode };
use ascii_opengl_rust::engine::core::Game;

use crate::game_event::GameEvent;

pub fn game_loop(
    device_state: &DeviceState,
    terminal_res: (u32, u32),
    game: &mut Game,
    display: &glium::Display,
    cam_offset: [f32; 3],
    state: &mut i8,
    game_events: &mut Vec<GameEvent>,
    last_keys: &mut Vec<Keycode>,
    acc: &mut f32,
    dt: &mut f32
) {
    let scene_index = game.get_scene_index();

    match scene_index {
        0 => crate::menu::game_loop(device_state, game, display,terminal_res, game_events, last_keys, dt),
        1 => crate::game::game_loop(device_state, terminal_res, game, display, cam_offset, state, game_events, last_keys, acc),
        _ => ()
    }

    *dt += 1.0;
}

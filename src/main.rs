#[macro_use]
extern crate ascii_opengl_rust;

pub mod game_init;
pub mod game_loop;
pub mod game_event;

pub mod game;
pub mod menu;

fn main() {
    let cam_offset = [0.0, 1.35, -5.0];

    let mut game_events: Vec<game_event::GameEvent> = Vec::new();

    let mut last_keys: Vec<device_query::Keycode> = Vec::new();

    let mut state = 0;

    let mut acc = 0.0;

    let mut dt = 0.0;

    init_engine!(
        game_loop!(
            game_loop::game_loop,
            cam_offset,
            &mut state,
            &mut game_events,
            &mut last_keys,
            &mut acc,
            &mut dt
        ),
        game_init!(game_init::game_init, cam_offset),
        "assets"
    );
}

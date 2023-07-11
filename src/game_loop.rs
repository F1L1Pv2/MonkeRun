extern crate device_query;
use device_query::{ DeviceState, DeviceQuery, Keycode };
use ascii_opengl_rust::engine::core::Game;
use ascii_opengl_rust::engine::matrices::model_matrix;
use crate::game_event::{GameEvent, KeyDownEvent, KeyUpEvent};

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

    // keys handler ---------------------------------------------------------------

    let keys: Vec<Keycode> = device_state.get_keys();

    keys.iter().for_each(|key| {
        if !last_keys.contains(key) {
            game_events.push(GameEvent::KeyDown(KeyDownEvent { key: key.clone() }));
        }
    });

    last_keys.iter().for_each(|key| {
        if !keys.contains(key) {
            game_events.push(GameEvent::KeyUp(KeyUpEvent { key: key.clone() }));
        }
    });

    *last_keys = keys.clone();

    // additional stuff -----------------------------------------------------------

    let mut rotate_objs = game.get_scene_mut().get_mut_objects_by_tags(vec!["rotate"]);

    rotate_objs.iter_mut().for_each(|obj| {
        let obj_pos = obj.model[3];

        obj.model = model_matrix(&[obj_pos[0], obj_pos[1], obj_pos[2]], &[0.0, *dt/60.0, 0.0], &[1.0, 1.0, 1.0]);
    });


    // scene handler ---------------------------------------------------------------

    let scene_index = game.get_scene_index();

    match scene_index {
        0 => crate::scenes::menu::game_loop(game, display,terminal_res, game_events, state,acc),
        1 => crate::scenes::game::game_loop(terminal_res, game, display, cam_offset, state, game_events, acc),
        2 => crate::scenes::win::game_loop(game, display,terminal_res, game_events),
        3 => crate::scenes::lose::game_loop(game, display,terminal_res, game_events),
        _ => ()
    }

    game_events.clear();

    *dt += 1.0;
}

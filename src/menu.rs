extern crate device_query;
use ascii_opengl_rust::engine::object::TextureFilter;
use ascii_opengl_rust::engine::ui::draw_rect;
use device_query::{ DeviceState, DeviceQuery, Keycode };
use ascii_opengl_rust::engine::core::Game;
use ascii_opengl_rust::engine::matrices::model_matrix;

use crate::game_event::GameEvent;
use crate::game_event::KeyUpEvent;
use crate::game_event::KeyDownEvent;

pub fn game_loop(
    device_state: &DeviceState,
    game: &mut Game,
    display: &glium::Display,
    terminal_size: (u32,u32),
    game_events: &mut Vec<GameEvent>,
    last_keys: &mut Vec<Keycode>,
    dt: &mut f32,
    state: &mut i8,
    acc: &mut f32
) {

    let mut break_run = false;

    game.camera.player_pos = [0.0, 0.0, 0.0];

    game.camera.update_self(terminal_size);

    // game events ----------------------------------------------------------------

    game_events.iter().for_each(|event| {
        match event {
            GameEvent::KeyDown(key_down_event) => {
                match key_down_event.key {
                    Keycode::Space => {

                        game.get_scene_by_index_mut(1).get_mut_objects_by_tags(vec!["player"])[0].model = model_matrix(&[0.0, 1.0, 0.0], &[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0]);
                        *state = 0;
                        *acc = 0.0;


                        game.set_scene(1);
                        break_run = true;
                    }

                    _ => (),
                }
            }

            _ => (),
        }
    });

    game_events.clear();

    if break_run {
        return;
    }

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


    let monke = &mut game.get_scene_mut().get_mut_objects_by_tags(vec!["rotate"])[0];

    let monke_pos = monke.model[3];

    monke.model = model_matrix(&[monke_pos[0], monke_pos[1], monke_pos[2]], &[0.0, *dt/60.0, 0.0], &[1.0, 1.0, 1.0]);



    game.get_ui_elems_mut().elems.clear();



    let assets_path = game.assets_path.clone();


    game.add_ui_elem(draw_rect(0.25, 0.0, 0.5, 0.25, (assets_path.clone()+"/sprites/title.png").as_str(),TextureFilter::Linear, display));
    game.add_ui_elem(draw_rect(0.0, 0.3, 0.35, 0.45, (assets_path.clone()+"/sprites/keybindings.png").as_str(),TextureFilter::Linear, display));
    game.add_ui_elem(draw_rect(0.30, 0.85, 0.4, 0.1, (assets_path+"/sprites/startbutton.png").as_str(),TextureFilter::Linear, display));

    // let font = Font::from_bytes(
    //     include_bytes!("../assets/fonts/Roboto-Regular.ttf") as &[u8],
    //     fontdue::FontSettings::default()
    // ).unwrap();

    // game.add_ui_elems(draw_text(0.0, 0.0, "Press_Space_To_Start", 1.5, &font, display));


}
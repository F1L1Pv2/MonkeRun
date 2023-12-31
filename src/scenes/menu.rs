extern crate device_query;
use ascii_opengl_rust::engine::object::TextureFilter;
use ascii_opengl_rust::engine::ui::draw_rect;
use device_query::Keycode;
use ascii_opengl_rust::engine::core::Game;
use ascii_opengl_rust::engine::matrices::model_matrix;
use crate::game_event::GameEvent;

pub fn game_loop(
    game: &mut Game,
    display: &glium::Display,
    terminal_size: (u32,u32),
    game_events: &mut Vec<GameEvent>,
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

    if break_run {
        return;
    }

    game.get_ui_elems_mut().elems.clear();

    let assets_path = game.assets_path.clone();

    game.add_ui_elem(draw_rect(0.25, 0.0, 0.5, 0.25, (assets_path.clone()+"/sprites/title.png").as_str(),TextureFilter::Nearest, display));
    game.add_ui_elem(draw_rect(0.0, 0.3, 0.35, 0.45, (assets_path.clone()+"/sprites/keybindings.png").as_str(),TextureFilter::Nearest, display));
    game.add_ui_elem(draw_rect(0.30, 0.85, 0.4, 0.1, (assets_path+"/sprites/startbutton.png").as_str(),TextureFilter::Nearest, display));

}
extern crate device_query;
use ascii_opengl_rust::engine::ui::draw_text;
use device_query::{ DeviceState, DeviceQuery, Keycode };
use ascii_opengl_rust::engine::core::Game;
use ascii_opengl_rust::engine::object::AABB;
use fontdue::Font;

use crate::game_event::GameEvent;
use crate::game_event::KeyUpEvent;
use crate::game_event::KeyDownEvent;

fn check_aabb_col(a: AABB, b: AABB) -> bool {
    a.min[0] <= b.max[0] &&
        a.max[0] >= b.min[0] &&
        a.min[1] <= b.max[1] &&
        a.max[1] >= b.min[1] &&
        a.min[2] <= b.max[2] &&
        a.max[2] >= b.min[2]
}

pub fn game_loop(
    device_state: &DeviceState,
    terminal_res: (u32, u32),
    game: &mut Game,
    display: &glium::Display,
    cam_offset: [f32; 3],
    state: &mut i8,
    game_events: &mut Vec<GameEvent>,
    last_keys: &mut Vec<Keycode>,
    acc: &mut f32
) {
    let gravity = -0.01;

    *acc += gravity;

    let mut jump = false;

    // game events ----------------------------------------------------------------

    game_events.iter().for_each(|event| {
        match event {
            GameEvent::KeyDown(key_down_event) => {
                match key_down_event.key {
                    Keycode::A => {
                        if *state > -1 {
                            *state -= 1;
                        }
                    }

                    Keycode::D => {
                        if *state < 1 {
                            *state += 1;
                        }
                    }

                    Keycode::Space => {
                        jump = true;
                    }

                    _ => (),
                }
            }

            _ => (),
        }
    });

    game_events.clear();

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

    // player movement -----------------------------------------------------------

    let mut player_model;
    let player_aabb;
    {
        let player = &mut game.get_scene_mut().get_mut_objects_by_tags(vec!["player"])[0];
        player.model[3][0] = (*state as f32) * 2.0;
        player.model[3][2] += 0.1;
        player_model = player.model.clone();
        player_aabb = player.get_aabb_acc((0.0, *acc, 0.0));
    }

    // physics --------------------------------------------------------------------

    let physics_objs = game.get_scene().get_objects_by_tags(vec!["physic"]);

    physics_objs.iter().for_each(|obj| {
        if check_aabb_col(player_aabb, obj.get_aabb()) {
            //check if tag "die" is present
            if obj.tags.contains(&"die".to_string()) {
                std::process::exit(69);

                //TODO: add game over screen
            }

            *acc = 0.0;

            if jump {
                *acc = 0.2;
            }
        }
    });

    player_model[3][1] += *acc;

    {
        let player = &mut game.get_scene_mut().get_mut_objects_by_tags(vec!["player"])[0];
        player.model = player_model;
    }

    // player.model[3][1] += *acc;

    // camera ---------------------------------------------------------------------

    let player_pos = game.get_scene().get_objects_by_tags(vec!["player"])[0].model[3];
    game.camera.player_pos = [
        // player_pos[0] + cam_offset[0],
        game.camera.player_pos[0],
        player_pos[1] + cam_offset[1],
        player_pos[2] + cam_offset[2],
    ];

    // physics_objs.iter().for_each(|obj| {

    // if !player.check_aabb_collision_acc((0.0,*acc,0.0), *obj){
    // player.model[3][1] += *acc;
    // }else{
    // *acc = 0.0;
    // }
    // });

    // win ------------------------------------------------------------------------

    if game.camera.player_pos[2] >= 130.0{
        std::process::exit(0);

        //TODO: add win screen
    }


    // debug ui -------------------------------------------------------------------

    let camera_pos = game.camera.view_matrix();

    //clear ui elements
    game.get_ui_elems_mut().elems.clear();

    // let camPosText = format!("{},{},{}", cameraPos[3][0], cameraPos[3][1], cameraPos[3][2]);
    // write floats in .3 format
    let cam_pos_text = format!(
        "{:.3},{:.3},{:.3}",
        camera_pos[3][0],
        camera_pos[3][1],
        camera_pos[3][2]
    );

    let font = Font::from_bytes(
        include_bytes!("../assets/fonts/Roboto-Regular.ttf") as &[u8],
        fontdue::FontSettings::default()
    ).unwrap();

    game.add_ui_elems(draw_text(0.0, 0.0, &cam_pos_text, 2.0, &font, display));

    game.camera.update(terminal_res, game.camera.player_pos, game.camera.player_rot);
}

extern crate glium;
use ascii_opengl_rust::engine::core::Game;
use ascii_opengl_rust::engine::scene::Scene;
// use ascii_opengl_rust::engine::object::{Object,TextureFilter};


pub fn game_init(_terminal_res: (u32, u32),game: &mut Game, display: &glium::Display,_cam_offset: [f32;3]) {
    
    game.add_scene(Scene::load_from_json("/scenes/menu.json",game.assets_path.as_str(), display).unwrap());

    game.add_scene(Scene::load_from_json("/scenes/baller.json",game.assets_path.as_str(), display).unwrap());
    
    
    // let player_pos = game.get_scene().get_objects_by_tags(vec!["player"])[0].model[3];

    // game.camera.player_pos = [cam_offset[0],cam_offset[1],cam_offset[2]];

    // println!("playerPos: {:?}", playerPos);

}

use bevy::prelude::{ Plugin, App };

mod setup;
mod size_camera;
mod control_camera;
mod game_of_life;
mod update_image;
mod draw_control;


pub struct SystemsPlugin {}
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup::setup)
            .add_system(size_camera::size_camera)
            .add_system(control_camera::control_camera)
            .add_plugin(game_of_life::GameOfLifePlugin {})
            .add_system(update_image::update_image)
            .add_system(draw_control::draw_control);
    }
}

use bevy::prelude::*;

mod setup;
mod size_camera;
mod control_camera;
mod game_of_life;
mod update_image;
mod draw_control;
// #[cfg(debug_assertions)]
// mod debug_systems;


pub struct SystemsPlugin {}
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup::setup)
            .add_plugins(game_of_life::GameOfLifePlugin {})
            .add_systems(Update, (
                size_camera::size_camera,
                control_camera::control_camera,
                update_image::update_image,
                draw_control::draw_control
            ));

        // #[cfg(debug_assertions)]
        // app.add_plugins(debug_systems::Debugger {});
    }
}

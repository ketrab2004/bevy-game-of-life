use bevy::prelude::{ Plugin, App };

mod setup;
mod size_camera;
mod control_camera;


pub struct SystemsPlugin {}
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup::setup)
            .add_system(size_camera::size_camera)
            .add_system(control_camera::control_camera);
    }
}

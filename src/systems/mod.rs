use bevy::prelude::{ Plugin, App };

mod size_camera;


pub struct SystemsPlugin {}
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(size_camera::size_camera);
    }
}

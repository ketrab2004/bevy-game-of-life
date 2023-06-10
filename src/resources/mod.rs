use bevy::prelude::{ Plugin, App };

pub mod controls;


pub struct ResourcesPlugin {}
impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<controls::Controls>();
    }
}

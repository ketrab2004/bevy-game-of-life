use bevy::prelude::Resource;


#[derive(Resource, Debug)]
pub struct Controls {
    mouse_sensitivity: f32
}

impl Default for Controls {
    fn default() -> Self {
        Controls {
            mouse_sensitivity: 0.1
        }
    }
}

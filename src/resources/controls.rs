use bevy::prelude::Resource;


#[derive(Resource, Debug)]
pub struct Controls {
    pub key_sensitivity: f32,

    pub scroll_line_sensitivity: f32,
    pub scroll_pixels_sensitivity: f32,

    pub ctrl_key_multiplier: f32,
    pub ctrl_scroll_multiplier: f32
}

impl Default for Controls {
    fn default() -> Self {
        Controls {
            key_sensitivity: 1.,

            scroll_line_sensitivity: 0.05,
            scroll_pixels_sensitivity: 0.01,

            ctrl_key_multiplier: 1.5,
            ctrl_scroll_multiplier: 2.
        }
    }
}

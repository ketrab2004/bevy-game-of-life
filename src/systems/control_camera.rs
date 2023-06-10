use bevy::{
    prelude::*,
    input::mouse::{ MouseMotion, MouseWheel, MouseScrollUnit }
};
use crate::resources::controls::Controls;


pub fn control_camera(
    time: Res<Time>,
    control_settings: Res<Controls>,
    windows: Query<&Window>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    // mut touch_reader: EventReader<TouchInput>, //TODO support touch screen
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut mouse_reader: EventReader<MouseMotion>,
    mut scroll_reader: EventReader<MouseWheel>
) {
    let delta = time.delta_seconds();
    let ctrl_pressed = keys.pressed(KeyCode::LControl) || keys.pressed(KeyCode::RControl);

    let window = windows.iter().next().expect("Failed to find a window");


    let movement = {
        let mut movement = Vec2::ZERO;

        if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up) {
            movement += Vec2::Y
        }
        if keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down) {
            movement -= Vec2::Y
        }
        if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left) {
            movement -= Vec2::X
        }
        if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right) {
            movement += Vec2::X
        }

        movement *=
            control_settings.key_sensitivity
            * delta
            * if ctrl_pressed {
                control_settings.ctrl_key_multiplier
            } else {1.};


        let smallest_axis = u32::min(window.physical_width(), window.physical_height()) as f32;

        if (keys.pressed(KeyCode::LAlt) && buttons.pressed(MouseButton::Left))
            || buttons.pressed(MouseButton::Middle) {
            for ev in mouse_reader.iter() {
                movement += (ev.delta * Vec2::new(-1., 1.)) / smallest_axis;
            }
        }

        movement
    };


    let scroll = {
        let mut scroll = 0.;

        for ev in scroll_reader.iter() {
            scroll -= match ev.unit {
                MouseScrollUnit::Line => (ev.x + ev.y) * control_settings.scroll_line_sensitivity,
                MouseScrollUnit::Pixel => (ev.x + ev.y) * control_settings.scroll_pixels_sensitivity
            }
        }

        scroll * if ctrl_pressed {
            control_settings.ctrl_scroll_multiplier
        } else {1.}
    };


    for (mut transform, mut projection) in query.iter_mut() {
        let scaled_scroll = scroll * projection.scale;
        projection.scale = (projection.scale + scaled_scroll).clamp(0.001, 1.);

        let scale_limit = (1. - projection.scale) / 2.;
        let scaled_movement = movement * projection.scale;

        transform.translation = (
            transform.translation + Vec3::new(scaled_movement.x, scaled_movement.y, 0.)
        ).clamp(
            Vec3::new(-scale_limit, -scale_limit, -f32::INFINITY),
            Vec3::new(scale_limit, scale_limit, f32::INFINITY)
        );
    }
}

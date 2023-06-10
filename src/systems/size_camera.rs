use bevy::{
    prelude::*,
    window::WindowResized,
    render::camera::ScalingMode
};


pub fn size_camera(
    mut events: EventReader<WindowResized>,
    mut query: Query<&mut OrthographicProjection, With<Camera2d>>
) {
    let Some(first) = events.iter().next() else {
        return;
    };


    let scale_mode = if first.width > first.height {
        ScalingMode::FixedVertical(1.)
    } else {
        ScalingMode::FixedHorizontal(1.)
    };


    for mut projection in query.iter_mut() {
        projection.scaling_mode = scale_mode.clone()
    }
}

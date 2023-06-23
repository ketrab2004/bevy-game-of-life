use bevy::{
    prelude::*,
    window::PrimaryWindow
};
use super::game_of_life::images_holder::ImagesHolder;


#[derive(PartialEq, Eq, Debug)]
enum DrawAction {
    None,
    Add,
    Remove
}

#[allow(clippy::too_many_arguments)]
pub fn draw_control(
    // mut commands: Commands,

    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,

    images_holder: Res<ImagesHolder>,
    image_assets: Res<Assets<Image>>,

    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Transform, &OrthographicProjection), With<Camera2d>>,
    frames: Query<(&Transform, &Sprite)>
) {
    let action = {
        if buttons.pressed(MouseButton::Left) {
            DrawAction::Add
        } else if buttons.pressed(MouseButton::Right) {
            DrawAction::Remove
        } else {
            DrawAction::None
        }
    };


    if keys.pressed(KeyCode::LAlt) {
        return
    }
    if action == DrawAction::None {
        return
    }


    let window = windows.single();
    let Some(window_mouse_pos) = window.cursor_position() else {
        return
    };


    let screen_size = Vec2::new(window.width(), window.height());
    let screen_rect = Rect::from_corners(Vec2::ZERO, screen_size);

    if !screen_rect.contains(window_mouse_pos) {
        return
    }


    let min_screen_axis = screen_size.x.min(screen_size.y);
    let screen_pos = window_mouse_pos / Vec2::splat(min_screen_axis);

    let (camera_transform, camera_projection) = cameras.iter().next().unwrap();
    let camera_pos = Vec2::from_slice(&camera_transform.translation.to_array());

    let (frame_transform, frame) = frames.iter().next().unwrap();
    let frame_pos = Vec2::from_slice(&frame_transform.translation.to_array());

    let frame_rect = Rect::from_center_size(frame_pos, frame.custom_size.unwrap());


    let hit_pos = camera_pos
        + (screen_pos -
            ((screen_size / min_screen_axis)
                * Vec2::splat(0.5)
            )
        ) * camera_projection.scale;

    if !frame_rect.contains(hit_pos) {
        return
    }


    let texture_size = image_assets.get(&images_holder.a).unwrap().size();

    let texture_pos = ((hit_pos + Vec2::splat(0.5)) * texture_size).floor();


    dbg!(texture_pos);
    //TODO send pos to gpu
}

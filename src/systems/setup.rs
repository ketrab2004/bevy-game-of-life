use bevy::prelude::*;
use super::game_of_life::images_holder::{
    ImagesHolder,
    ImagesHolderState
};


pub fn setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    images_holder: Res<ImagesHolder>
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: f32::EPSILON,
            far: 5.,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0., 0., 1.),
            ..default()
        },
        ..default()
    });

    commands.spawn(SpriteBundle {
        // texture: asset_server.load("uv_grid.png"),
        texture: match images_holder.state {
            ImagesHolderState::ImageA => images_holder.a.clone(),
            ImagesHolderState::ImageB => images_holder.b.clone()
        },

        sprite: Sprite {
            custom_size: Some(Vec2::ONE),
            ..default()
        },
        ..default()
    });
}

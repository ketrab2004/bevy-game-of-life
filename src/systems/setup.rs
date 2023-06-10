use bevy::prelude::*;


pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: f32::EPSILON,
            far: 5.,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            ..default()
        },
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("uv_grid.png"),

        sprite: Sprite {
            custom_size: Some(Vec2::ONE),
            ..default()
        },
        ..default()
    });
}

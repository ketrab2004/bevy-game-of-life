use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    render::camera::ScalingMode
};


pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: f32::EPSILON,
            far: 5.,
            scaling_mode: ScalingMode::FixedVertical(1.), //TODO remove (because of the size_camera system)
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            ..default()
        },
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::ONE).into())
            .into(),

        material: materials.add(ColorMaterial::from(Color::CYAN)),

        ..default()
    });
}

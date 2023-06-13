use bevy::{
    prelude::*,
    render::render_resource::{
        Extent3d,
        TextureDimension,
        TextureFormat,
        TextureUsages
    }
};
use super::{
    types::{
        ImagesHolder,
        ImagesHolderState
    },
    SIZE
};


pub fn startup(
    mut commands: Commands,
    mut assets: ResMut<Assets<Image>>
) {
    let mut image = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8Unorm
    );

    image.texture_descriptor.usage =
        TextureUsages::COPY_DST
        | TextureUsages::STORAGE_BINDING
        | TextureUsages::TEXTURE_BINDING;

    commands.insert_resource(ImagesHolder {
        state: ImagesHolderState::ImageA,
        a: assets.add(image.clone()),
        b: assets.add(image)
    });
}

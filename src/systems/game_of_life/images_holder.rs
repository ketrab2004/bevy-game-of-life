use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}
    }
};
use super::SIZE;


#[derive(Debug, Clone, Copy)]
pub enum ImagesHolderState {
    ImageA,
    ImageB
}

#[derive(Resource, ExtractResource, Debug, Clone)]
pub struct ImagesHolder {
    pub state: ImagesHolderState,
    pub a: Handle<Image>,
    pub b: Handle<Image>
}

impl FromWorld for ImagesHolder {
    fn from_world(world: &mut World) -> Self {
        let mut assets = world.get_resource_mut::<Assets<Image>>()
            .expect("resource Assets<Image> somehow doesn't exist");

        let mut image = Image::new_fill(
            Extent3d {
                width: SIZE.0,
                height: SIZE.1,
                depth_or_array_layers: 1
            },
            TextureDimension::D2,
            &[0, 0, 0, 255],
            TextureFormat::Rgba8Unorm
        );

        image.texture_descriptor.usage =
            TextureUsages::COPY_DST
            | TextureUsages::STORAGE_BINDING
            | TextureUsages::TEXTURE_BINDING;

        ImagesHolder {
            state: ImagesHolderState::ImageA,
            a: assets.add(image.clone()),
            b: assets.add(image)
        }
    }
}

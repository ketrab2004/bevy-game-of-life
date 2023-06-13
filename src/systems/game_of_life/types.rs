use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        render_resource::{Extent3d, TextureDimension, TextureFormat}
    }
};
use super::{
    SIZE
};


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

        let image = Image::new_fill(
            Extent3d {
                width: SIZE.0,
                height: SIZE.1,
                depth_or_array_layers: 1
            },
            TextureDimension::D2,
            &[0],
            TextureFormat::R8Unorm
        );

        ImagesHolder {
            state: ImagesHolderState::ImageA,
            a: assets.add(image.clone()),
            b: assets.add(image)
        }
    }
}

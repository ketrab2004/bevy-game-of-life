use bevy::{
    prelude::*,
    render::{
        render_resource::{
            BindGroupLayout,
            BindGroupLayoutEntry,
            BindGroupLayoutDescriptor,
            ShaderStages,
            BindingType,
            StorageTextureAccess,
            TextureFormat,
            TextureViewDimension,
            BindGroup,
            BindGroupEntry,
            BindGroupDescriptor,
            BindingResource
        },
        renderer::RenderDevice,
        render_asset::RenderAssets
    }
};
use crate::systems::game_of_life::images_holder::ImagesHolder;
use super::{
    LayoutHolderCreator,
    LayoutHolder,
    GroupHolderCreator,
    GroupHolder,
    LayoutsHolder
};


pub struct ImagesLayout(BindGroupLayout);
impl LayoutHolderCreator for ImagesLayout {
    fn new(world: &mut World) -> Self {
        let render_device = world.get_resource::<RenderDevice>().unwrap();

        let image_layout_template = BindGroupLayoutEntry {
            binding: u32::MAX,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::StorageTexture {
                access: StorageTextureAccess::ReadWrite,
                format: TextureFormat::Rgba8Unorm,
                view_dimension: TextureViewDimension::D2,
            },
            count: None,
        };

        Self(
            render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    ..image_layout_template
                }, BindGroupLayoutEntry {
                    binding: 1,
                    ..image_layout_template
                }]
            })
        )
    }
}
impl LayoutHolder for ImagesLayout {
    fn get_bind_group_layout(&self) -> BindGroupLayout {
        self.0.to_owned()
    }
}


#[derive(Clone)]
pub struct Images(BindGroup);
impl GroupHolderCreator for Images {
    fn new(world: &mut World) -> Self {
        let render_device = world.get_resource::<RenderDevice>().unwrap();
        let layouts = world.get_resource::<LayoutsHolder>().unwrap();
        let gpu_images = world.get_resource::<RenderAssets<Image>>().unwrap();
        let image_holder = world.get_resource::<ImagesHolder>().unwrap();

        Self(
            render_device.create_bind_group(&BindGroupDescriptor {
                label: None,
                layout: &layouts.images.get_bind_group_layout(),
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&gpu_images[&image_holder.a].texture_view),
                }, BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&gpu_images[&image_holder.b].texture_view)
                }],
            })
        )
    }
}
impl GroupHolder for Images {
    fn get_bind_group(&self) -> BindGroup {
        self.0.to_owned()
    }
}

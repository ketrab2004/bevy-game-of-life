use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        renderer::RenderDevice,
        render_resource::{
            BindGroupDescriptor,
            BindGroupEntry,
            BindingResource,
            BindGroup,
            BindGroupLayoutDescriptor,
            BindGroupLayoutEntry,
            ShaderStages,
            BindingType,
            StorageTextureAccess,
            TextureFormat,
            TextureViewDimension,
            BindGroupLayout
        }
    }
};
use super::images_holder::ImagesHolder;



#[derive(Resource, Debug)]
pub struct BindGroupLayouts {
    pub images: BindGroupLayout
}

impl FromWorld for BindGroupLayouts {
    fn from_world(world: &mut World) -> Self {
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

        Self {
            images: render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    ..image_layout_template
                }, BindGroupLayoutEntry {
                    binding: 1,
                    ..image_layout_template
                }]
            })
        }
    }
}



#[derive(Resource, Debug)]
pub struct BindGroups {
    pub images: BindGroup
}

impl FromWorld for BindGroups {
    fn from_world(world: &mut World) -> Self {
        let gpu_images = world.get_resource::<RenderAssets<Image>>().unwrap();
        let render_device = world.get_resource::<RenderDevice>().unwrap();

        let bind_group_layouts = world.get_resource::<BindGroupLayouts>().unwrap();
        let image_holder = world.get_resource::<ImagesHolder>().unwrap();


        Self {
            images: render_device.create_bind_group(&BindGroupDescriptor {
                label: None,
                layout: &bind_group_layouts.images,
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&gpu_images[&image_holder.a].texture_view),
                }, BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&gpu_images[&image_holder.b].texture_view)
                }],
            })
        }
    }
}

pub fn queue_bind_groups( mut commands: Commands ) {
    commands.init_resource::<BindGroups>()
}

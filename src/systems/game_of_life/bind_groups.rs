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
            BindGroupLayout,
            BufferBindingType,
            BufferInitDescriptor,
            BufferUsages
        }
    }
};
use super::images_holder::{
    ImagesHolder,
    ImagesHolderState
};



#[derive(Resource, Debug)]
pub struct BindGroupLayouts {
    pub images: BindGroupLayout,
    pub current_image: BindGroupLayout
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
                label: Some("Binding group 0 layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    ..image_layout_template
                }, BindGroupLayoutEntry {
                    binding: 1,
                    ..image_layout_template
                }]
            }),
            current_image: render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Binding group 1 layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        has_dynamic_offset: false,
                        min_binding_size: None,
                        ty: BufferBindingType::Uniform
                    },
                    count: None
                }]
            })
        }
    }
}



#[derive(Resource, Debug)]
pub struct BindGroups {
    pub images: BindGroup,
    current_image_a: BindGroup,
    current_image_b: BindGroup
}

impl BindGroups {
    pub fn get_current_image_from_state(&self, state: ImagesHolderState) -> &BindGroup {
        match state {
            ImagesHolderState::ImageA => &self.current_image_a,
            ImagesHolderState::ImageB => &self.current_image_b
        }
    }
}

impl FromWorld for BindGroups {
    fn from_world(world: &mut World) -> Self {
        let gpu_images = world.get_resource::<RenderAssets<Image>>().unwrap();
        let render_device = world.get_resource::<RenderDevice>().unwrap();

        let bind_group_layouts = world.get_resource::<BindGroupLayouts>().unwrap();
        let image_holder = world.get_resource::<ImagesHolder>().unwrap();


        Self {
            images: render_device.create_bind_group(&BindGroupDescriptor {
                label: Some("Binding group 0"),
                layout: &bind_group_layouts.images,
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&gpu_images[&image_holder.a].texture_view),
                }, BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&gpu_images[&image_holder.b].texture_view)
                }],
            }),
            current_image_a: render_device.create_bind_group(&BindGroupDescriptor {
                label: Some("Binding group 1 (ImageA)"),
                layout: &bind_group_layouts.current_image,
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(render_device.create_buffer_with_data(&BufferInitDescriptor {
                        label: Some("binding 0"),
                        contents: &(ImagesHolderState::ImageA as u32).to_be_bytes(),
                        usage: BufferUsages::UNIFORM
                    }).as_entire_buffer_binding())
                }]
            }),
            current_image_b: render_device.create_bind_group(&BindGroupDescriptor {
                label: Some("Binding group 1 (ImageB)"),
                layout: &bind_group_layouts.current_image,
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(render_device.create_buffer_with_data(&BufferInitDescriptor {
                        label: Some("binding 0"),
                        contents: &(ImagesHolderState::ImageB as u32).to_be_bytes(),
                        usage: BufferUsages::UNIFORM
                    }).as_entire_buffer_binding())
                }]
            })
        }
    }
}

pub fn queue_bind_groups( mut commands: Commands ) {
    commands.init_resource::<BindGroups>()
}

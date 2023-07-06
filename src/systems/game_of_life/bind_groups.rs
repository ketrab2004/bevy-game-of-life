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
            BufferUsages,
            AsBindGroup,
            PreparedBindGroup
        }, texture::FallbackImage
    }
};
use super::{
    images_holder::{
        ImagesHolder,
        ImagesHolderState
    },
    actions_holder::ActionsHolder
};



#[derive(Resource, Debug)]
pub struct BindGroupLayouts {
    pub images: BindGroupLayout,
    pub current_image: BindGroupLayout,
    pub actions: BindGroupLayout
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
            }),
            current_image: render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
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
            }),
            actions: ActionsHolder::bind_group_layout(render_device)
            // actions: render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            //     label: None,
            //     entries: &[BindGroupLayoutEntry {
            //         binding: 0,
            //         visibility: ShaderStages::COMPUTE,
            //         ty: BindingType::Buffer {
            //             has_dynamic_offset: true,
            //             min_binding_size: None,
            //             ty: BufferBindingType::Storage {
            //                 read_only: true
            //             }
            //         },
            //         count: Some(std::num::NonZeroU32::new(1).unwrap())
            //     }]
            // })
        }
    }
}



#[derive(Resource)]
pub struct BindGroups {
    pub images: BindGroup,
    current_image_a: BindGroup,
    current_image_b: BindGroup,
    pub actions: PreparedBindGroup<()>
}

impl BindGroups {
    pub fn get_current_image_from_state(&self, state: ImagesHolderState) -> &BindGroup {
        match state {
            ImagesHolderState::ImageA => &self.current_image_a,
            ImagesHolderState::ImageB => &self.current_image_b
        }
    }

    // pub fn get_actions_bind_group(world: &World, actions: Vec<Action>) -> BindGroup {
    //     let render_device = world.get_resource::<RenderDevice>().unwrap();
    //     let bind_group_layouts = world.get_resource::<BindGroupLayouts>().unwrap();

    //     render_device.create_bind_group(&BindGroupDescriptor {
    //         label: None,
    //         layout: &bind_group_layouts.actions,
    //         entries: &[BindGroupEntry {
    //             binding: 0,
    //             resource: BindingResource::BufferArray(&actions.iter().map(|action| {
    //                 let data = [
    //                     (action.action as u32).to_be_bytes().as_slice(),
    //                     action.pos.x.to_be_bytes().as_slice(),
    //                     action.pos.y.to_be_bytes().as_slice()
    //                 ].concat().as_slice();

    //                 BufferBinding {
    //                     buffer: &render_device.create_buffer_with_data(&BufferInitDescriptor {
    //                         label: None,
    //                         contents: data,
    //                         usage: BufferUsages::STORAGE
    //                     }),
    //                     offset: 0,
    //                     size: std::num::NonZeroU64::new(data.len() as u64)
    //                 }
    //             }).collect())
    //         }]
    //     })
    // }
}

impl FromWorld for BindGroups {
    fn from_world(world: &mut World) -> Self {
        let gpu_images = world.get_resource::<RenderAssets<Image>>().unwrap();
        let fallback_image = world.get_resource::<FallbackImage>().unwrap();
        let render_device = world.get_resource::<RenderDevice>().unwrap();

        let bind_group_layouts = world.get_resource::<BindGroupLayouts>().unwrap();
        let image_holder = world.get_resource::<ImagesHolder>().unwrap();
        let actions_holder = world.get_resource::<ActionsHolder>().unwrap();


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
            }),
            current_image_a: render_device.create_bind_group(&BindGroupDescriptor {
                label: None,
                layout: &bind_group_layouts.current_image,
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(render_device.create_buffer_with_data(&BufferInitDescriptor {
                        label: None,
                        contents: &(ImagesHolderState::ImageA as u32).to_be_bytes(),
                        usage: BufferUsages::UNIFORM
                    }).as_entire_buffer_binding())
                }]
            }),
            current_image_b: render_device.create_bind_group(&BindGroupDescriptor {
                label: None,
                layout: &bind_group_layouts.current_image,
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(render_device.create_buffer_with_data(&BufferInitDescriptor {
                        label: None,
                        contents: &(ImagesHolderState::ImageB as u32).to_be_bytes(),
                        usage: BufferUsages::UNIFORM
                    }).as_entire_buffer_binding())
                }]
            }),
            actions: actions_holder.as_bind_group(
                &bind_group_layouts.actions,
                render_device,
                gpu_images,
                fallback_image
            // error does not implement Debug nor Display, so can't use .expect()
            ).unwrap_or_else(|_| { panic!("failed to create bind group for the actions_holder") })
        }
    }
}

pub fn queue_bind_groups( mut commands: Commands ) {
    commands.init_resource::<BindGroups>()
}

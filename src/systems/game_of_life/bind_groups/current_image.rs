use bevy::{
    prelude::*,
    render::{
        render_resource::{
            BindGroupLayout,
            BindGroupLayoutEntry,
            BindGroupLayoutDescriptor,
            ShaderStages,
            BindingType,
            BufferBindingType,
            BindGroup,
            BindGroupEntry,
            BindGroupDescriptor,
            BindingResource,
            BufferInitDescriptor,
            BufferUsages
        },
        renderer::RenderDevice
    }
};
use crate::systems::game_of_life::images_holder::ImagesHolderState;
use super::{
    LayoutHolderCreator,
    LayoutHolder,
    GroupHolderCreator,
    GroupHolder,
    LayoutsHolder
};


pub struct CurrentImageLayout(BindGroupLayout);
impl LayoutHolderCreator for CurrentImageLayout {
    fn new(world: &mut World) -> Self {
        let render_device = world.get_resource::<RenderDevice>().unwrap();

        Self(
            render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
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
            })
        )
    }
}
impl LayoutHolder for CurrentImageLayout {
    fn get_bind_group_layout(&self) -> BindGroupLayout {
        self.0.to_owned()
    }
}


const BUFFER_INIT_DESCRIPTOR_TEMPLATE: BufferInitDescriptor = BufferInitDescriptor {
    label: None,
    contents: &(u32::MAX).to_be_bytes(),
    usage: BufferUsages::UNIFORM
};

#[derive(Clone)]
pub struct CurrentImageA(BindGroup);
impl GroupHolderCreator for CurrentImageA {
    fn new(world: &mut World) -> Self {
        let render_device = world.get_resource::<RenderDevice>().unwrap();
        let layouts = world.get_resource::<LayoutsHolder>().unwrap();

        Self(
            render_device.create_bind_group(&BindGroupDescriptor {
                label: None,
                layout: &layouts.current_image.get_bind_group_layout(),
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(render_device.create_buffer_with_data(&BufferInitDescriptor {
                        contents: &(ImagesHolderState::ImageA as u32).to_be_bytes(),
                        ..BUFFER_INIT_DESCRIPTOR_TEMPLATE
                    }).as_entire_buffer_binding())
                }]
            })
        )
    }
}
impl GroupHolder for CurrentImageA {
    fn get_bind_group(&self) -> BindGroup {
        self.0.to_owned()
    }
}

#[derive(Clone)]
pub struct CurrentImageB(BindGroup);
impl GroupHolderCreator for CurrentImageB {
    fn new(world: &mut World) -> Self {
        let render_device = world.get_resource::<RenderDevice>().unwrap();
        let layouts = world.get_resource::<LayoutsHolder>().unwrap();

        Self(
            render_device.create_bind_group(&BindGroupDescriptor {
                label: None,
                layout: &layouts.current_image.get_bind_group_layout(),
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(render_device.create_buffer_with_data(&BufferInitDescriptor {
                        contents: &(ImagesHolderState::ImageB as u32).to_be_bytes(),
                        ..BUFFER_INIT_DESCRIPTOR_TEMPLATE
                    }).as_entire_buffer_binding())
                }]
            })
        )
    }
}
impl GroupHolder for CurrentImageB {
    fn get_bind_group(&self) -> BindGroup {
        self.0.to_owned()
    }
}

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
            Buffer,
            BufferInitDescriptor,
            BufferUsages
        },
        renderer::{
            RenderDevice,
            RenderQueue
        }
    }
};
use crate::systems::game_of_life::actions_holder::{
    Action,
    ActionsHolder
};
use super::{
    LayoutHolderCreator,
    LayoutHolder,
    GroupHolderCreator,
    GroupHolder,
    LayoutsHolder,
    GroupsHolder
};


pub struct ActionsLayout(BindGroupLayout);
impl ActionsLayout {
    pub fn get_min_binding_size() -> usize {
        bytemuck::bytes_of::<Action>(
            &Action::default()
        ).len()
        +
        std::mem::size_of::<usize>()
    }
}
impl LayoutHolderCreator for ActionsLayout {
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
                        min_binding_size: std::num::NonZeroU64::new(Self::get_min_binding_size() as u64),
                        ty: BufferBindingType::Storage { read_only: true }
                    },
                    count: None
                }]
            })
        )
    }
}
impl LayoutHolder for ActionsLayout {
    fn get_bind_group_layout(&self) -> BindGroupLayout {
        self.0.to_owned()
    }
}


#[derive(Clone)]
pub struct Actions {
    buffer: Buffer,
    group: BindGroup
}
impl Actions {
    pub(self) fn create_buffer_with_data(render_device: &RenderDevice, data: &[u8]) -> Buffer {
        let empty;
        let contents = if data.is_empty() {
            // buffer must be > min binding size
            empty = vec![0; ActionsLayout::get_min_binding_size()];
            // dbg!(ActionsLayout::get_min_binding_size());
            // empty = vec![0; std::mem::size_of::<u32>() * 2];
            empty.as_slice()

        } else {
            data
        };

        render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST
        })
    }

    pub(self) fn create_bind_group(render_device: &RenderDevice, layouts: &LayoutsHolder, buffer: Buffer) -> BindGroup {
        render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &layouts.actions.get_bind_group_layout(),
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding()
            }]
        })
    }

    fn write_to_buffer(&self, render_queue: &RenderQueue, data: &[u8]) {
        render_queue.write_buffer(&self.buffer, 0, data)
    }

    fn replace_buffer(&mut self, render_device: &RenderDevice, layouts: &LayoutsHolder, data: &[u8]) {
        // TODO also update the bind group layout...
        // TODO also also update the pipeline...
        self.buffer = Actions::create_buffer_with_data(
            render_device,
            data
        );

        self.group = Actions::create_bind_group(
            render_device,
            layouts,
            self.buffer.to_owned()
        );
    }

    pub fn update_buffer(&mut self, render_device: &RenderDevice, render_queue: &RenderQueue, layouts: &LayoutsHolder, data: &[u8]) {
        if (self.buffer.size() as usize) < data.len() {
            self.replace_buffer(render_device, layouts, data);
        } else {
            self.write_to_buffer(render_queue, data);
        }
    }
}
impl GroupHolderCreator for Actions {
    fn new(world: &mut World) -> Self {
        let render_device = world.get_resource::<RenderDevice>().unwrap();
        let layouts = world.get_resource::<LayoutsHolder>().unwrap();

        let buffer = Self::create_buffer_with_data(render_device, &[]);

        Self {
            buffer: buffer.to_owned(),
            group: Self::create_bind_group(render_device, layouts, buffer)
        }
    }
}
impl GroupHolder for Actions {
    fn get_bind_group(&self) -> BindGroup {
        self.group.to_owned()
    }
}


pub fn update_actions(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    actions_holder: Res<ActionsHolder>,
    mut groups_holder: ResMut<GroupsHolder>,
    layouts_holder: Res<LayoutsHolder>
) {
    if !actions_holder.is_changed() {
        return;
    }

    let new_buffer_data: &[u8] = bytemuck::cast_slice(actions_holder.actions.as_slice());

    groups_holder.actions.update_buffer(
        render_device.as_ref(),
        render_queue.as_ref(),
        layouts_holder.as_ref(),
        new_buffer_data
    );
}

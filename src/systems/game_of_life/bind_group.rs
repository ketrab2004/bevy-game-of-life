use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssets,
        renderer::RenderDevice,
        render_resource::{
            BindGroupDescriptor,
            BindGroupEntry,
            BindingResource,
            BindGroup
        }
    }
};
use super::{
    types::ImagesHolder,
    pipeline::Pipeline
};


#[derive(Resource)]
pub struct ImageBindGroup(pub(super) BindGroup);


pub fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<Pipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    image_holder: Res<ImagesHolder>,
    render_device: Res<RenderDevice>,
) {
    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &pipeline.texture_bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::TextureView(&gpu_images[&image_holder.a].texture_view),
        }, BindGroupEntry {
            binding: 1,
            resource: BindingResource::TextureView(&gpu_images[&image_holder.b].texture_view)
        }],
    });

    commands.insert_resource(ImageBindGroup(bind_group));
}

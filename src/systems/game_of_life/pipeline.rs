use bevy::{
    prelude::*,
    render::{
        render_resource::{
            BindGroupLayout,
            CachedComputePipelineId,
            BindGroupLayoutDescriptor,
            BindGroupLayoutEntry,
            ShaderStages,
            BindingType,
            StorageTextureAccess,
            TextureFormat,
            TextureViewDimension,
            PipelineCache,
            ComputePipelineDescriptor
        },
        renderer::RenderDevice
    }
};
use std::borrow::Cow;


#[derive(Resource, Debug)]
pub struct Pipeline {
    pub texture_bind_group_layout: BindGroupLayout,
    pub init_pipeline: CachedComputePipelineId,
    pub update_pipeline: CachedComputePipelineId,
}

impl FromWorld for Pipeline {
    fn from_world(world: &mut World) -> Self {
        let bind_group_layout_entry = BindGroupLayoutEntry {
            binding: u32::MAX,
            visibility: ShaderStages::COMPUTE,
            ty: BindingType::StorageTexture {
                access: StorageTextureAccess::ReadWrite,
                format: TextureFormat::Rgba8Unorm,
                view_dimension: TextureViewDimension::D2,
            },
            count: None,
        };

        let texture_bind_group_layout =
            world
                .resource::<RenderDevice>()
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        ..bind_group_layout_entry
                    }, BindGroupLayoutEntry {
                        binding: 1,
                        ..bind_group_layout_entry
                    }]
                });

        let shader = world
            .resource::<AssetServer>()
            .load("game_of_life.wgsl");

        let pipeline_cache = world.resource::<PipelineCache>();

        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
        });

        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
        });

        Pipeline {
            texture_bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}

use bevy::{
    prelude::*,
    render::{
        render_resource::{
            CachedComputePipelineId,
            PipelineCache,
            ComputePipelineDescriptor
        }
    }
};
use std::borrow::Cow;
use super::bind_groups::BindGroupLayouts;


#[derive(Resource, Debug)]
pub struct Pipeline {
    pub init_pipeline: CachedComputePipelineId,
    pub update_pipeline: CachedComputePipelineId,
}

impl FromWorld for Pipeline {
    fn from_world(world: &mut World) -> Self {
        let bind_group_layouts = world.get_resource::<BindGroupLayouts>().unwrap();

        let shader = world
            .resource::<AssetServer>()
            .load("game_of_life.wgsl");

        let pipeline_cache = world.resource::<PipelineCache>();


        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![bind_group_layouts.images.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
        });

        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![bind_group_layouts.images.clone()],
            push_constant_ranges: Vec::new(),
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
        });


        Pipeline {
            init_pipeline,
            update_pipeline,
        }
    }
}

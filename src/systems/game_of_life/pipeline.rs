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
    pub input_pipeline: CachedComputePipelineId,
    pub update_pipeline: CachedComputePipelineId
}
impl Pipeline {
    pub fn get_pipelines_vec(&self) -> Vec<CachedComputePipelineId> {
        vec!(
            // self.input_pipeline,
            self.update_pipeline
        )
    }
}

impl FromWorld for Pipeline {
    fn from_world(world: &mut World) -> Self {
        let bind_group_layouts = world.get_resource::<BindGroupLayouts>().unwrap();

        let asset_server = world.resource::<AssetServer>();

        let input_shader = asset_server.load("add_inputs.wgsl");
        let main_shader = asset_server.load("game_of_life.wgsl");

        let pipeline_cache = world.resource::<PipelineCache>();


        let input_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some(Cow::from("input pipeline")),
            layout: vec![
                bind_group_layouts.images.clone(),
                bind_group_layouts.current_image.clone(),
                bind_group_layouts.actions.clone()
            ],
            push_constant_ranges: Vec::new(),
            shader: input_shader,
            shader_defs: vec![],
            entry_point: Cow::from("add_inputs"),
        });

        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some(Cow::from("update pipeline")),
            layout: vec![
                bind_group_layouts.images.clone(),
                bind_group_layouts.current_image.clone()
            ],
            push_constant_ranges: Vec::new(),
            shader: main_shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
        });


        Pipeline {
            input_pipeline,
            update_pipeline
        }
    }
}

use bevy::{
    prelude::*,
    render::render_resource::{
        CachedComputePipelineId,
        PipelineCache,
        ComputePipelineDescriptor
    }
};
use std::borrow::Cow;
use super::bind_groups::{LayoutsHolder, LayoutHolder};


#[derive(Resource, Debug)]
pub struct Pipeline {
    pub input_pipeline: CachedComputePipelineId,
    pub update_pipeline: CachedComputePipelineId
}
impl Pipeline {
    pub fn get_pipelines_vec(&self) -> Vec<CachedComputePipelineId> {
        vec!(
            self.input_pipeline,
            self.update_pipeline
        )
    }
}

impl FromWorld for Pipeline {
    fn from_world(world: &mut World) -> Self {
        let bind_group_layouts = world.get_resource::<LayoutsHolder>().unwrap();

        let asset_server = world.resource::<AssetServer>();

        let input_shader = asset_server.load("add_inputs.wgsl");
        let main_shader = asset_server.load("game_of_life.wgsl");

        let pipeline_cache = world.resource::<PipelineCache>();


        let input_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some(Cow::from("input pipeline")),
            layout: vec![
                bind_group_layouts.images.get_bind_group_layout(),
                bind_group_layouts.current_image.get_bind_group_layout(),
                bind_group_layouts.actions.get_bind_group_layout()
            ],
            push_constant_ranges: Vec::new(),
            shader: input_shader,
            shader_defs: vec![],
            entry_point: Cow::from("add_inputs"),
        });

        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some(Cow::from("update pipeline")),
            layout: vec![
                bind_group_layouts.images.get_bind_group_layout(),
                bind_group_layouts.current_image.get_bind_group_layout()
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

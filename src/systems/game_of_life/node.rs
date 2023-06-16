use bevy::{
    prelude::*,
    render::{
        render_graph::{
            Node as RenderGraphNode,
            RenderGraphContext,
            NodeRunError
        },
        renderer::RenderContext,
        render_resource::{
            PipelineCache,
            CachedPipelineState,
            ComputePassDescriptor
        }
    }
};
use super::{
    SIZE,
    WORKGROUP_SIZE,
    pipeline::Pipeline,
    bind_groups::BindGroups
};


#[derive(Debug, Default)]
pub enum NodeState {
    #[default]
    Loading,
    Resize(u32, u32),
    Update
}


#[derive(Debug, Default)]
pub struct Node {
    state: NodeState
}

impl RenderGraphNode for Node {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<Pipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            NodeState::Loading => {
                if let CachedPipelineState::Ok(_) = pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline)
                {
                    self.state = NodeState::Update;
                }
            }
            _ => ()
        }
    }

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<Pipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, &world.resource::<BindGroups>().images, &[]);

        // select the pipeline based on the current state
        match self.state {
            NodeState::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE.0, SIZE.1 / WORKGROUP_SIZE.1, 1);
            }
            _ => ()
        }

        Ok(())
    }
}

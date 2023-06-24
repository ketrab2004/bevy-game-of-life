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
use strum::{
    EnumCount,
    IntoEnumIterator
};
use super::{
    SIZE,
    WORKGROUP_SIZE,
    pipeline::Pipeline,
    bind_groups::BindGroups,
    images_holder::{
        ImagesHolder,
        ImagesHolderState
    },
    actions_holder::ActionsHolder
};


pub const NODE_ID: &str = "game_of_life";

#[derive(Debug, Default)]
pub enum NodeState {
    #[default]
    Loading,
    Resize(u32, u32),
    Update
}


#[derive(Debug)]
pub struct Node {
    state: NodeState,
    timer: Timer
}
impl Default for Node {
    fn default() -> Self {
        Self {
            state: NodeState::default(),
            timer: Timer::from_seconds(5., TimerMode::Repeating)
        }
    }
}

impl RenderGraphNode for Node {
    fn update(&mut self, world: &mut World) {
        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            NodeState::Loading => {
                let pipeline = world.resource::<Pipeline>();
                let pipeline_cache = world.resource::<PipelineCache>();

                if let CachedPipelineState::Ok(_) = pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = NodeState::Update;
                }
            }
            NodeState::Update => {
                world.resource_scope(|_, mut images_holder: Mut<ImagesHolder>| {
                    let desired_state_index = (self.timer.times_finished_this_tick() + images_holder.state as u32) as usize % ImagesHolderState::COUNT;

                    let Some(desired_state) = ImagesHolderState::iter().nth(desired_state_index) else {
                        panic!("Somehow couldn't find ImagesHolderState with index {}", desired_state_index);
                    };

                    images_holder.state = desired_state;
                });

                let time = world.resource::<Time>();
                self.timer.tick(time.delta());
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
        let bind_groups = world.resource::<BindGroups>();
        let images_holder = world.resource::<ImagesHolder>();
        let actions_holder = world.resource::<ActionsHolder>();

        // select the pipeline based on the current state
        match self.state {
            NodeState::Update => {

                if !actions_holder.0.is_empty() {
                    //TODO send actions to gpu to apply them
                    dbg!(&actions_holder.0);
                }


                if self.timer.just_finished() {
                    let mut pass = render_context
                        .command_encoder()
                        .begin_compute_pass(&ComputePassDescriptor::default());

                    let current_image_state = images_holder.state;

                    pass.set_bind_group(0, &bind_groups.images, &[]);
                    pass.set_bind_group(1, bind_groups.get_current_image_from_state(current_image_state), &[]);

                    let update_pipeline = pipeline_cache
                        .get_compute_pipeline(pipeline.update_pipeline)
                        .unwrap();
                    pass.set_pipeline(update_pipeline);

                    // for _ in 0..self.timer.times_finished_this_tick() {
                        //TODO update bind group 1 (current image) in loop
                    pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE.0, SIZE.1 / WORKGROUP_SIZE.1, 1);
                    // }
                }
            }
            NodeState::Resize(_, _) => todo!(),
            _ => ()
        }

        Ok(())
    }
}

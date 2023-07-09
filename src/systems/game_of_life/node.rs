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
        // if the corresponding pipelines has loaded, transition to the next stage
        match self.state {
            NodeState::Loading => {
                let pipeline = world.resource::<Pipeline>();
                let pipeline_cache = world.resource::<PipelineCache>();

                let mut all_loaded = true;
                for (i, id) in pipeline.get_pipelines_vec().iter().enumerate() {
                    let state = pipeline_cache.get_compute_pipeline_state(id.to_owned());

                    match state {
                        CachedPipelineState::Ok(_) => (),
                        CachedPipelineState::Err(err) => {
                            panic!("failed to load pipeline #{i}, {err}");
                        },
                        _ => {
                            all_loaded = false;
                            break
                        }
                    }
                }

                if all_loaded {
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
        // select the pipeline based on the current state
        match self.state {
            NodeState::Update => {
                let pipeline_cache = world.resource::<PipelineCache>();
                let pipeline = world.resource::<Pipeline>();
                let bind_groups = world.resource::<BindGroups>();
                let images_holder = world.resource::<ImagesHolder>();
                let actions_holder = world.resource::<ActionsHolder>();


                #[warn(clippy::overly_complex_bool_expr)] //TODO fix issue with input pipeline and remove && false
                if !actions_holder.actions.is_empty() && false {
                    info!("executing input pipeline!  🖱️");
                    let mut pass = render_context
                        .command_encoder()
                        .begin_compute_pass(&ComputePassDescriptor::default());

                    pass.set_bind_group(0, &bind_groups.images, &[]);
                    pass.set_bind_group(1, bind_groups.get_current_image_from_state(images_holder.state), &[]);
                    pass.set_bind_group(2, &bind_groups.actions.bind_group, &[]);

                    let input_pipeline = pipeline_cache
                        .get_compute_pipeline(pipeline.input_pipeline)
                        .unwrap();
                    pass.set_pipeline(input_pipeline);

                    pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE.0, SIZE.1 / WORKGROUP_SIZE.1, 1);
                }


                if self.timer.just_finished() {
                    info!("executing update pipeline! ⬆️");
                    let mut pass = render_context
                        .command_encoder()
                        .begin_compute_pass(&ComputePassDescriptor::default());

                    pass.set_bind_group(0, &bind_groups.images, &[]);
                    pass.set_bind_group(1, bind_groups.get_current_image_from_state(images_holder.state), &[]);

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

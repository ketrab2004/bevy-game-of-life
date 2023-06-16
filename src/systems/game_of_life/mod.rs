use bevy::{
    prelude::*,
    render::{
        RenderApp,
        RenderSet,
        render_graph::RenderGraph,
        extract_resource::ExtractResourcePlugin
    }
};
use self::types::ImagesHolder;

mod types;
mod startup;

mod bind_groups;
mod pipeline;

mod node;

pub(self) const SIZE: (u32, u32) = (1280, 720);
pub(self) const WORKGROUP_SIZE: (u32, u32) = (8, 8);


#[derive(Debug)]
pub struct GameOfLifePlugin {}
impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup::startup);
        app.add_plugin(ExtractResourcePlugin::<ImagesHolder>::default());


        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<bind_groups::BindGroupLayouts>()
            .init_resource::<pipeline::Pipeline>()
            .add_system(bind_groups::queue_bind_groups.in_set(RenderSet::Queue));


        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node("game_of_life", node::Node::default());

        // make node run before camera render
        render_graph.add_node_edge(
            "game_of_life",
            bevy::render::main_graph::node::CAMERA_DRIVER,
        );
    }
}

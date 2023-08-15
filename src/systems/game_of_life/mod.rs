use bevy::{
    prelude::*,
    render::{
        RenderApp,
        render_graph::RenderGraph,
        extract_resource::ExtractResourcePlugin
    }
};


pub mod actions_holder;
pub mod images_holder;
mod bind_groups;
mod pipeline;
mod node;

pub(self) const SIZE: (u32, u32) = (64, 64);
pub(self) const WORKGROUP_SIZE: (u32, u32) = (8, 8);


#[derive(Debug)]
pub struct GameOfLifePlugin {}
impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<images_holder::ImagesHolder>()
            .init_resource::<actions_holder::ActionsHolder>()
            .add_plugins((
                ExtractResourcePlugin::<images_holder::ImagesHolder>::default(),
                ExtractResourcePlugin::<actions_holder::ActionsHolder>::default()
            ))
            .add_systems(PreUpdate, actions_holder::actions_holder_cleaner);


        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_plugins(bind_groups::BindGroupsPlugin {});

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node(node::NODE_ID, node::Node::default());

        // make node run before camera render
        render_graph.add_node_edge(
            node::NODE_ID,
            bevy::render::main_graph::node::CAMERA_DRIVER
        );
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<bind_groups::LayoutsHolder>()
            .init_resource::<pipeline::Pipeline>();
    }
}

use bevy::prelude::*;

mod resources;
mod systems;


#[tokio::main]
async fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(resources::ResourcesPlugin{})
        .add_plugin(systems::SystemsPlugin{})
        .run();
}

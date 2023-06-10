use bevy::prelude::*;

mod setup;
mod resources;
mod systems;


#[tokio::main]
async fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(resources::ResourcesPlugin{})
        .add_startup_system(setup::setup)
        .add_plugin(systems::SystemsPlugin{})
        .run();
}

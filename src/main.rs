use bevy::prelude::*;

mod setup;
mod systems;


#[tokio::main]
async fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup::setup)
        .add_plugin(systems::SystemsPlugin{})
        .run();
}

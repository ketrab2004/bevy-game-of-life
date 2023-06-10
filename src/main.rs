use bevy::prelude::*;

mod setup;


#[tokio::main]
async fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup::setup)
        .run();
}

use bevy::prelude::*;

mod resources;
mod systems;


#[tokio::main]
async fn main() {
    // set BEVY_ASSET_ROOT to make asset loading work even when the .exe is run directly
    if cfg!(debug_assertions) {
        match std::env::current_exe() {
            Err(err) => warn!("failed to get path of executable, assets might fail to load; {err}"),
            Ok(path) => {
                let asset_folder = if path.components().any(|component| component.as_os_str() == "target") {
                    // extra ../ to also remove the .exe part of the path
                    path.join("../../../").canonicalize().expect("failed to get assets folder path")
                } else {
                    path.join("../").canonicalize().expect("failed to get default assets folder path")
                };

                std::env::set_var("BEVY_ASSET_ROOT", asset_folder.as_os_str());
            }
        }
    }


    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(resources::ResourcesPlugin{})
        .add_plugin(systems::SystemsPlugin{})
        .run();
}

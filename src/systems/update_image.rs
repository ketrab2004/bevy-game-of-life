use bevy::prelude::*;
use super::game_of_life::images_holder::{
    ImagesHolder,
    ImagesHolderState
};


pub fn update_image(
    mut commands: Commands,
    images_holder: Res<ImagesHolder>,
    sprites: Query<Entity, (With<Sprite>, With<Handle<Image>>)>
) {
    for entity in sprites.iter() {
        // replace existing image handle
        commands.entity(entity).insert(match images_holder.state {
            ImagesHolderState::ImageA => images_holder.a.clone(),
            ImagesHolderState::ImageB => images_holder.b.clone()
        });
    }
}

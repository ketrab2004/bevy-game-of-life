use bevy::{
    prelude::*,
    render::{
        render_resource::{
            BindGroupLayout,
            BindGroup
        },
        Render,
        RenderSet
    }
};
use super::images_holder::ImagesHolderState;


mod images;
mod current_image;
mod actions;


pub(self) trait LayoutHolderCreator {
    fn new(world: &mut World) -> Self;
}
pub trait LayoutHolder {
    fn get_bind_group_layout(&self) -> BindGroupLayout;
}

pub(self) trait GroupHolderCreator {
    fn new(world: &mut World) -> Self;
}
pub trait GroupHolder {
    fn get_bind_group(&self) -> BindGroup;
}


#[derive(Resource)]
pub struct LayoutsHolder {
    pub images: images::ImagesLayout,
    pub current_image: current_image::CurrentImageLayout,
    pub actions: actions::ActionsLayout
}

impl FromWorld for LayoutsHolder {
    fn from_world(world: &mut World) -> Self {
        Self {
            images: images::ImagesLayout::new(world),
            current_image: current_image::CurrentImageLayout::new(world),
            actions: actions::ActionsLayout::new(world)
        }
    }
}


#[derive(Resource)]
pub struct GroupsHolder {
    pub images: images::Images,
    pub current_image_a: current_image::CurrentImageA,
    pub current_image_b: current_image::CurrentImageB,
    pub actions: actions::Actions
}

impl GroupsHolder {
    pub fn get_current_image_from_state(&self, state: ImagesHolderState) -> BindGroup {
        match state {
            ImagesHolderState::ImageA => self.current_image_a.get_bind_group(),
            ImagesHolderState::ImageB => self.current_image_b.get_bind_group()
        }
    }
}
impl FromWorld for GroupsHolder {
    fn from_world(world: &mut World) -> Self {
        Self {
            images: images::Images::new(world),
            current_image_a: current_image::CurrentImageA::new(world),
            current_image_b: current_image::CurrentImageB::new(world),
            actions: actions::Actions::new(world)
        }
    }
}


pub struct BindGroupsPlugin {}
impl Plugin for BindGroupsPlugin {
    fn build(&self, render_app: &mut App) {
        render_app.add_systems(Render, (
            queue_bind_groups,
            actions::update_actions
                .run_if(resource_exists::<GroupsHolder>())
        ).chain()
        .in_set(RenderSet::Queue));
    }
}

pub fn queue_bind_groups( mut commands: Commands ) {
    commands.init_resource::<GroupsHolder>()
}

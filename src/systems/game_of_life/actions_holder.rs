use bevy::{
    prelude::*,
    render::extract_resource::ExtractResource
};


#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum ActionType {
    Add,
    Remove
}


#[derive(Clone, Debug)]
#[repr(C)]
pub struct Action {
    action: ActionType,
    pos: Vec2
}


#[derive(Resource, ExtractResource, Clone, Default, Debug)]
pub struct ActionsHolder(pub Vec<Action>);

impl ActionsHolder {
    pub fn push_raw(&mut self, action: Action) {
        self.0.push(action)
    }

    pub fn push(&mut self, typ: ActionType, pos: Vec2) {
        self.push_raw(Action {
            action: typ,
            pos
        })
    }
}


pub fn actions_holder_cleaner(mut actions_holder: ResMut<ActionsHolder>) {
    actions_holder.0.clear();
}

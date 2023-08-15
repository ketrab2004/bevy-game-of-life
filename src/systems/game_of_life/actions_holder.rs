use bevy::{
    prelude::*,
    render::extract_resource::ExtractResource
};


#[repr(u32)]
#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
pub enum ActionType {
    #[default]
    Add = 0,
    Remove = 1
}
unsafe impl bytemuck::Contiguous for ActionType {
    type Int = u32;
    const MIN_VALUE: Self::Int = ActionType::Add as u32;
    const MAX_VALUE: Self::Int = ActionType::Remove as u32;
}
unsafe impl bytemuck::Pod for ActionType {}
unsafe impl bytemuck::Zeroable for ActionType {}

// impl ShaderType for ActionType {
//     type ExtraMetadata = ();
//     const METADATA: Metadata<Self::ExtraMetadata> = Metadata {
//         alignment: AlignmentValue::new(u32::SHADER_SIZE.get()),
//         has_uniform_min_alignment: true,
//         min_size: SizeValue::from(u32::SHADER_SIZE),
//         extra: ()
//     };
// }
// impl ShaderSize for ActionType {}

// impl WriteInto for ActionType {
//     fn write_into<B>(&self, writer: &mut Writer<B>)
//         where B: BufferMut
//     {
//         writer.write(&(self.to_owned() as u32).to_be_bytes());
//     }
// }


#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Action {
    pub pos: Vec2,
    pub action: ActionType,
}


#[derive(Resource, ExtractResource, Clone, Default, Debug)]
pub struct ActionsHolder {
    pub actions: Vec<Action>
}

impl ActionsHolder {
    #[inline]
    pub fn push_raw(&mut self, action: Action) {
        self.actions.push(action)
    }

    #[inline]
    pub fn push(&mut self, typ: ActionType, pos: Vec2) {
        self.push_raw(Action {
            action: typ,
            pos,
            // _padding: 0
        })
    }
}


pub fn actions_holder_cleaner(mut actions_holder: ResMut<ActionsHolder>) {
    actions_holder.actions.clear();
}

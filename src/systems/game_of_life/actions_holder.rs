use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        render_resource::{
            AsBindGroup,
            ShaderType,
            ShaderSize,
            encase::{
                private::{
                    Metadata,
                    AlignmentValue,
                    SizeValue,
                    Writer,
                    BufferMut
                },
                internal::WriteInto
            }
        }
    }
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ActionType {
    Add = 0,
    Remove = 1
}

impl ShaderType for ActionType {
    type ExtraMetadata = ();
    const METADATA: Metadata<Self::ExtraMetadata> = Metadata {
        alignment: AlignmentValue::new(u32::SHADER_SIZE.get()),
        has_uniform_min_alignment: true,
        min_size: SizeValue::from(u32::SHADER_SIZE),
        extra: ()
    };
}
impl ShaderSize for ActionType {}

impl WriteInto for ActionType {
    fn write_into<B>(&self, writer: &mut Writer<B>)
        where B: BufferMut
    {
        writer.write(&(self.to_owned() as u32).to_be_bytes());
    }
}


#[derive(Clone, Debug, ShaderType)]
pub struct Action {
    pub action: ActionType,
    pub pos: Vec2
}


#[derive(Resource, ExtractResource, AsBindGroup, Clone, Default, Debug)]
pub struct ActionsHolder {
    #[storage(0, read_only, visibility(compute))]
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
            pos
        })
    }
}


pub fn actions_holder_cleaner(mut actions_holder: ResMut<ActionsHolder>) {
    actions_holder.actions.clear();
}

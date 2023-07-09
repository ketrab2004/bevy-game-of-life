use bevy::prelude::*;

#[cfg(debug_assertions)]
use bevy_mod_debugdump::{
    schedule_graph::Settings as ScheduleGraphSettings,
    schedule_graph_dot,
    render_graph::Settings as RenderGraphSettings,
    render_graph_dot
};


pub struct Debugger;
impl Plugin for Debugger {
    fn name(&self) -> &str {
        if cfg!(debug_assertions) {
            "bevy_mod_debugdump-er"
        } else {
            ""
        }
    }

    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            let schedule_graph_settings = ScheduleGraphSettings {
                ..default()
            };
            let render_graph_settings = RenderGraphSettings {
                ..default()
            };

            std::fs::write("./main_schedule.dot", schedule_graph_dot(app, CoreSchedule::Main, &schedule_graph_settings))
                .expect("failed to save main schedule graph");

            std::fs::write("./render.dot", render_graph_dot(app, &render_graph_settings))
                .expect("failed to save render graph");
        }
    }
}

use bevy::prelude::*;
// use bevy_debug_text_overlay::OverlayPlugin;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_debug_text_overlay::{screen_print};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, _app: &mut App) {
        // app
        // .add_plugin(WorldInspectorPlugin::new())
            // .add_plugin(OverlayPlugin {
            //     font_size: 32.0,
            //     ..default()
            // })
            // .add_system(draw_current_state)
    }
}

// fn draw_current_state(state: Res<State<crate::states::GameState>>,) {
//     let current_state = &state.0;
//     let str = format!("{:#?}", current_state);
//     screen_print!("State: {str}");
// }
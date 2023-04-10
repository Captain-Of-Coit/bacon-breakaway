use bevy::prelude::*;

use crate::states;
use crate::player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(setup_camera.in_schedule(OnEnter(states::GameState::InGame)))
        .add_system(move_camera.run_if(in_state(states::GameState::InGame)));
    }
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

// use bevy_math::Lerp;
use bevy_easings::Lerp;

fn move_camera(
    windows: Query<&Window>,
    time: Res<Time>,
    piggie_stats: Res<player::PiggieStats>,
    piggie_query: Query<(&player::Piggie, &Transform), Without<Camera2d>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    let (_, piggie_transform) = piggie_query.get_single().unwrap();
    let window = windows.get_single().expect("Only one window");
    let screen_height = window.height();
    let top_20_percent = screen_height * 0.8;

    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        // let window_height_half = screen_height * 0.5;
        let piggie_y_in_viewport = piggie_transform.translation.y - camera_transform.translation.y;// + window_height_half;

        let progress = (piggie_y_in_viewport / top_20_percent).clamp(0.0, 1.0);
        let multiplier = f32::lerp(&1.0, &5.0, &progress);

        camera_transform.translation.y += multiplier * piggie_stats.camera_speed * time.delta_seconds();
    }
}

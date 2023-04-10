use bevy::prelude::*;
// use bevy_debug_text_overlay::{screen_print};

use crate::player::PiggieStats;

#[derive(Resource)]
pub struct RunStats {
    pub distance: u32,
    pub record_distance: u32,
}

impl Default for RunStats {
    fn default() -> Self {
        RunStats {
            distance: 0,
            record_distance: 0,
        }
    }
}

pub struct RunStatsPlugin;

impl Plugin for RunStatsPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<RunStats>()
        .add_system(update_runstats.run_if(in_state(crate::states::GameState::InGame)))
        // .add_system(print_runstats.run_if(in_state(crate::states::GameState::InGame)))
        
        // .add_system(on_enter.in_schedule(OnEnter(crate::states::GameState::InGame)))
        // .add_system(on_exit.in_schedule(OnExit(crate::states::GameState::InGame)))

        .add_system(calculate_corn_earnings.in_schedule(OnExit(crate::states::GameState::InGame)))
        .add_system(ensure_top_record_distance.in_schedule(OnExit(crate::states::GameState::InGame)))
        
        ;
    }
}

// fn on_enter(
//     mut commands: Commands,
// ) {
//     commands.remove_resource::<RunStats>();
//     commands.insert_resource(RunStats::default());
// }

fn update_runstats(
    mut runstats: ResMut<RunStats>,
    player: Query<&Transform, With<crate::player::Piggie>>,
) {
    let position = player.get_single().unwrap();
    let distance = position.translation.y;
    runstats.distance = distance as u32;
}

fn calculate_corn_earnings(
    runstats: Res<RunStats>,
    mut piggiestats: ResMut<PiggieStats>,
) {
    let earned_corn = (runstats.distance as u32) / crate::constants::CORN_DISTANCE_EARN_DIVIDER;
    piggiestats.corn = piggiestats.corn + earned_corn;
    // Always get at least 10 corn
    if piggiestats.corn < crate::constants::MINIMUM_CORN_EARN {
        piggiestats.corn = crate::constants::MINIMUM_CORN_EARN;
    }
    // runstats.distance = 0;
}

fn ensure_top_record_distance(
    mut runstats: ResMut<RunStats>
) {
    if runstats.distance > runstats.record_distance {
        runstats.record_distance = runstats.distance
    }
}

// fn print_runstats(runstats: Res<RunStats>) {
//     let distance = runstats.distance;
//     // distance = distance.round();
//     screen_print!("Distance: {distance}");
//     // println!("Distance: {}", distance);
// }
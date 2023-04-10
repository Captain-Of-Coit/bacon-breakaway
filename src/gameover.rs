use bevy::prelude::*;
use belly::prelude::*;

pub struct GameoverPlugin;
use std::time::Duration;

use crate::states;

#[derive(Component)]
struct NextStateTimer {
    timer: Timer,
}

impl Plugin for GameoverPlugin {
    fn build(&self, app: &mut App) {
        app
        // Startup
        .add_system(setup_gameover.in_schedule(OnEnter(states::GameState::GameOver)))
        // Gameplay
        .add_system(check_if_gameover.run_if(in_state(states::GameState::InGame)))

        .add_system(next_state.run_if(in_state(states::GameState::GameOver)));
    }
}

fn setup_gameover(
    mut commands: Commands,
    runstats: Res<crate::runstats::RunStats>,
) {
    // println!("Gaaaame ooooveeeer!");
    let distance = runstats.distance.to_string();
    let earned_corn = ((runstats.distance as u32) / crate::constants::CORN_DISTANCE_EARN_DIVIDER).to_string();

    commands.add(eml! {
        <div>
            <div s:flex-direction="column" s:align-items="center" s:justify-content="center" s:width="100%" s:height="100%" s:color="black">
                <div s:font-size="46" s:color="black" >
                    "YOU GOT CAUGHT!"
                </div>
                <div s:font-size="34" s:color="black" >
                    "Distance: " {distance}
                </div>
                <div s:font-size="34" s:color="black" >
                    "Earned Corn: " {earned_corn}
            </div>
            </div>
        </div>
    });
    // commands.add(StyleSheet::parse(
    //     r#"
    //     * {
    //         font: "fonts/Yomogi-Regular.ttf"
    //     }
    // "#,
    // ));

    commands.spawn((
        NextStateTimer {
            timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
        },
    ));
}

fn next_state(
    mut commands: Commands,
    mut q: Query<(Entity, &mut NextStateTimer)>,
    time: Res<Time>,
    all_transform_query: Query<Entity, With<Transform>>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
) {
    for (timer_entity, mut fuse_timer) in q.iter_mut() {
        // timers gotta be ticked, to work
        fuse_timer.timer.tick(time.delta());

        // if it finished, despawn the bomb
        if fuse_timer.timer.finished() {
            // println!("Transition to corn store bro!");
            for e in all_transform_query.iter() {
                commands.entity(e).despawn_recursive();
            }
            commands.entity(timer_entity).despawn_recursive();
            next_state.set(crate::states::GameState::CornStore);
        }
    }
}


fn check_if_gameover(
    q_camera: Query<&Transform, With<Camera>>,
    q_player: Query<&Transform, With<crate::player::Piggie>>,
    windows: Query<&Window>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
) {
    let window = windows.get_single().unwrap();

    let window_height = window.height();
    // Give player a few more pixels (25) to survive on
    let camera_limit = (window_height / 2.0) + 25.0;

    let camera_transform = q_camera.get_single().unwrap();
    let player_transform = q_player.get_single().unwrap();
    
    let camera_y = camera_transform.translation.y;
    let player_y = player_transform.translation.y;

    // println!("Camera: {}, Player: {}", camera_y, player_y);

    let difference = camera_y - player_y;

    if difference > camera_limit {
        next_state.set(crate::states::GameState::GameOver)
    }
}
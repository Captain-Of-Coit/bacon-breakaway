use belly::prelude::*;
use bevy::prelude::*;

use crate::runstats::RunStats;
use crate::states;
use crate::player::PiggieStats;

pub struct PlayerUIPlugin;

impl Plugin for PlayerUIPlugin {
    fn build(&self, app: &mut App) {
        app
        //.init_resource::<PiggieStats>()
          //  .add_event::<DrainEnergyEvent>()
            // Startup
            .add_system(setup_player_ui.in_schedule(OnEnter(states::GameState::InGame)))
            // .add_system(stop_player.in_schedule(OnExit(states::GameState::InGame)))
            // Gameplay
            // .add_system(print_energy.run_if(in_state(states::GameState::InGame)))
            // .add_system(move_player.run_if(in_state(states::GameState::InGame)))
            // .add_system(update_energy.run_if(in_state(states::GameState::InGame)))
            // .add_system(update_animation_timer.run_if(in_state(states::GameState::InGame)))
            // .add_system(animate_sprite.run_if(in_state(states::GameState::InGame)));
            ;
    }
}

fn setup_player_ui(mut commands: Commands) {
    commands.add(eml! {
    // <body>
        <div s:flex-direction="column" s:color="black">
            <div s:width="100%" s:height="90%" s:flex-direction="row" s:background-color="xblue">
                // Left sidebar
                <div s:width="50%" s:height="100%" s:flex-direction="column" s:background-color="xred" s:color="black">
                    // "Distance: "{from!(RunStats:distance | fmt.s("{s:0.2}"))}

                    // "Record Distance: "{from!(RunStats:record_distance | fmt.s("{s:0.2}"))}

                    "Energy"
                    <progressbar s:width="400px" maximum=100. bind:value=from!(PiggieStats:energy)/>
                    "Fatness"
                    <progressbar s:width="400px" maximum=100. bind:value=from!(PiggieStats:fatness)/>
                    "Stamina"
                    <progressbar s:width="400px" maximum=1. bind:value=from!(PiggieStats:stamina)/>
                </div>
            </div>
          </div>
    // </body>
  })
}

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
// use iyes_progress::{Progress, ProgressCounter, ProgressPlugin, ProgressSystem};
// use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

use crate::states;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/player.png")]
    pub player: Handle<Image>,
    #[asset(path = "images/player-anim.png")]
    pub player_anim: Handle<Image>,
    
    #[asset(path = "images/obstacle.png")]
    pub obstacle: Handle<Image>,
    #[asset(path = "images/cow.png")]
    pub cow: Handle<Image>,


    #[asset(path = "images/piggie_face.png")]
    pub piggie_face: Handle<Image>,
    
    #[asset(path = "images/main_menu_bg.jpg")]
    pub main_menu_bg: Handle<Image>,
    #[asset(path = "images/credits_bg.jpg")]
    pub credits_bg: Handle<Image>,
    #[asset(path = "images/barn.jpg")]
    pub cornstore_bg: Handle<Image>,


    #[asset(path = "images/title.png")]
    pub title: Handle<Image>,

    #[asset(path = "fonts/Yomogi-Regular.ttf")]
    pub font: Handle<Font>,

    #[asset(path = "images/apple.png")]
    pub apple: Handle<Image>,

    #[asset(path = "images/donut.png")]
    pub donut: Handle<Image>,

}

// #[derive(AssetCollection, Resource)]
// pub struct MusicAssets {
//     #[asset(path = "music/intro.mp3")]
//     pub main_menu: Handle<AudioSource>,
// }

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_loading_state(
            LoadingState::new(states::GameState::Loading)
                .continue_to_state(crate::constants::STARTING_STATE),
        )
        .add_collection_to_loading_state::<_, ImageAssets>(states::GameState::Loading)

        // .add_plugin(ProgressPlugin::new(states::GameState::Loading))

        // .add_system(print_progress.run_if(in_state(states::GameState::Loading)))
        // .add_collection_to_loading_state::<_, MusicAssets>(states::GameState::Loading)
        ;
    }
}

// fn print_progress(
//     progress: Option<Res<ProgressCounter>>,
//     diagnostics: Res<Diagnostics>,
//     mut last_done: Local<u32>,
// ) {
//     if let Some(progress) = progress.map(|counter| counter.progress()) {
//         if progress.done > *last_done {
//             *last_done = progress.done;
//             info!(
//                 "[Frame {}] Changed progress: {:?}",
//                 diagnostics
//                     .get(FrameTimeDiagnosticsPlugin::FRAME_COUNT)
//                     .map(|diagnostic| diagnostic.value().unwrap_or(0.))
//                     .unwrap_or(0.),
//                 progress
//             );
//         }
//     }
// }
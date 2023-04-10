use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::{prelude::Audio, AudioControl, AudioPlugin, AudioInstance, AudioTween};

// use crate::assets;
// use crate::music;
use crate::{states, main};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(AudioPlugin)
        // .add_system(setup.in_schedule(OnExit(states::GameState::Loading)))
        .add_startup_system(setup)

        .add_system(enter_main_menu.in_schedule(OnEnter(states::GameState::MainMenu)))
        // .add_system(exit_main_menu.in_schedule(OnExit(states::GameState::MainMenu)))

        .add_system(enter_game.in_schedule(OnEnter(states::GameState::InGame)))
        // .add_system(exit_game.in_schedule(OnExit(states::GameState::InGame)))

        .add_system(enter_cornstore.in_schedule(OnEnter(states::GameState::GameOver)))
        // .add_system(exit_cornstore.in_schedule(OnExit(states::GameState::CornStore)))

        
        .add_system(enter_credits.in_schedule(OnEnter(states::GameState::Finished)))
        // .add_system(exit_credits.in_schedule(OnExit(states::GameState::Credits)))
        
        ;
    }
}


#[derive(Resource)]
struct MusicController {
  intro: Handle<AudioInstance>,
  main: Handle<AudioInstance>,
  cornstore: Handle<AudioInstance>,
  credits: Handle<AudioInstance>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
  let intro_handle = audio.play(asset_server.load("music/intro.ogg")).looped().with_volume(0.0).handle();

  commands.insert_resource(MusicController{
    intro: intro_handle,
    main: audio.play(asset_server.load("music/main.ogg")).looped().with_volume(0.0).handle(),
    cornstore: audio.play(asset_server.load("music/cornstore.ogg")).looped().with_volume(0.0).handle(),
    credits: audio.play(asset_server.load("music/credits.ogg")).looped().with_volume(0.0).handle(),
  });
}

const DEFAULT_FADE: u64 = 50;

fn enter_main_menu(
  handle: Res<MusicController>,
  mut audio_instances: ResMut<Assets<AudioInstance>>
) {
  // println!("Enter main menu");
  if let Some(instance) = audio_instances.get_mut(&handle.credits) {
    // instance.seek_to(playback_position);
    instance.set_volume(0.0, AudioTween::linear(Duration::from_millis(DEFAULT_FADE)));
  }

  if let Some(instance) = audio_instances.get_mut(&handle.intro) {
    // println!("Setting valume");
    instance.set_volume(1.0, AudioTween::linear(Duration::from_millis(DEFAULT_FADE)));
  }
}

// fn exit_main_menu(
//   handle: Res<MusicController>,
//   mut audio_instances: ResMut<Assets<AudioInstance>>
// ) {
//   if let Some(instance) = audio_instances.get_mut(&handle.intro) {
//     instance.set_volume(0.0, AudioTween::linear(Duration::from_millis(DEFAULT_FADE)));
//   }
// }

fn enter_game(
  handle: Res<MusicController>,
  mut audio_instances: ResMut<Assets<AudioInstance>>
) {
  let mut playback_position: f64 = 0.0;
  if let Some(intro_instance) = audio_instances.get_mut(&handle.intro) {
    intro_instance.set_volume(0.0, AudioTween::linear(Duration::from_millis(DEFAULT_FADE)));
    playback_position = intro_instance.state().position().unwrap();
  }
  if let Some(intro_instance) = audio_instances.get_mut(&handle.cornstore) {
    intro_instance.set_volume(0.0, AudioTween::linear(Duration::from_millis(DEFAULT_FADE)));
  }
  if let Some(main_instance) = audio_instances.get_mut(&handle.main) {
    main_instance.seek_to(playback_position);
    main_instance.set_volume(1.0, AudioTween::linear(Duration::from_millis(DEFAULT_FADE)));
  }
}

// fn exit_game(
//   handle: Res<MusicController>,
//   mut audio_instances: ResMut<Assets<AudioInstance>>
// ) {
//   if let Some(instance) = audio_instances.get_mut(&handle.main) {
//     instance.set_volume(0.0, AudioTween::linear(Duration::from_millis(DEFAULT_FADE)));
//   }
// }

fn enter_cornstore(
  handle: Res<MusicController>,
  mut audio_instances: ResMut<Assets<AudioInstance>>
) {
  let mut playback_position: f64 = 0.0;
  if let Some(instance) = audio_instances.get_mut(&handle.main) {
    instance.set_volume(0.0, AudioTween::linear(Duration::from_millis(DEFAULT_FADE)));
    playback_position = instance.state().position().unwrap();
  }
  if let Some(instance) = audio_instances.get_mut(&handle.cornstore) {
    instance.seek_to(playback_position);
    instance.set_volume(1.0, AudioTween::linear(Duration::from_millis(DEFAULT_FADE)));
  }
}

// fn exit_cornstore(
//   handle: Res<MusicController>,
//   mut audio_instances: ResMut<Assets<AudioInstance>>
// ) {
//   if let Some(instance) = audio_instances.get_mut(&handle.cornstore) {
//     instance.set_volume(0.0, AudioTween::linear(Duration::from_millis(DEFAULT_FADE)));
//   }
// }

fn enter_credits(
  handle: Res<MusicController>,
  mut audio_instances: ResMut<Assets<AudioInstance>>
) {
  let mut playback_position: f64 = 0.0;
  if let Some(instance) = audio_instances.get_mut(&handle.main) {
    instance.set_volume(0.0, AudioTween::linear(Duration::from_millis(DEFAULT_FADE)));
    playback_position = instance.state().position().unwrap();
  }
  if let Some(instance) = audio_instances.get_mut(&handle.credits) {
    instance.seek_to(playback_position);
    instance.set_volume(1.0, AudioTween::linear(Duration::from_millis(DEFAULT_FADE)));
  }
}

// fn exit_credits(
//   handle: Res<MusicController>,
//   mut audio_instances: ResMut<Assets<AudioInstance>>
// ) {
//   if let Some(instance) = audio_instances.get_mut(&handle.credits) {
//     instance.set_volume(0.0, AudioTween::linear(Duration::from_millis(DEFAULT_FADE)));
//   }
// }

// fn setup(
//   mut commands: Commands,
//   audio: Res<Audio>,
//   audio_assets: Res<assets::MusicAssets>,
// ) {
//   let music = audio_assets.main_menu.clone();
//   let handle = audio.play_with_settings(
//     music,
//     PlaybackSettings::LOOP.with_volume(0.0),
//   );
//   commands.insert_resource(MusicController{
//     main_menu: handle
//   });
// }


// fn enter_main_menu(
//   music_controller: Res<MusicController>,
//   audio_sinks: Res<Assets<AudioSink>>,
// ) {
//   println!("Main menu entered");
//   println!("{:#?}", music_controller.main_menu);

//   // let weak: Audio = music_controller.main_menu.make_strong();

//   // let music: Audio = music_controller.main_menu();

//   if let Some(sink) = audio_sinks.get(&music_controller.main_menu) {
//     println!("Setting colume to 1.0");
//     sink.set_volume(1.0);
//     sink.play();
//   } else {
//     println!("Didn't find audio sink...");
//   }
// }
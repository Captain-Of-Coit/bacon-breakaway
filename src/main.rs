use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod constants;
mod states;
mod assets;
mod player;
mod player_ui;
mod camera;
mod obstacles;
mod map;
mod runstats;
mod debug;
mod gameover;
mod mainmenu;
mod cornstore;
mod finish;
mod utils;
mod music;

fn main() {
    App::new()
        // Clear background
        // .insert_resource(ClearColor(Color::rgba(0.0, 0.0, 0.0, 0.0)))

        // 3rd party plugins
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Tell the asset server to watch for asset changes on disk:
            watch_for_changes: true,
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())

        // My Game Plugins
        .add_plugin(debug::DebugPlugin)
        .add_plugin(states::StatesPlugin)
        .add_plugin(map::MapPlugin)
        .add_plugin(assets::AssetsPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(player_ui::PlayerUIPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(obstacles::ObstaclesPlugin)
        .add_plugin(runstats::RunStatsPlugin)
        .add_plugin(gameover::GameoverPlugin)
        .add_plugin(mainmenu::MainmenuPlugin)
        .add_plugin(cornstore::CornstorePlugin)
        .add_plugin(finish::FinishPlugin)
        .add_plugin(music::MusicPlugin)

        
        .run();
}

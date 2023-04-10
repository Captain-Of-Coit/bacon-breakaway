// use bevy::ecs::system::assert_is_system;
use bevy::prelude::*;
use bevy_inspector_egui::egui::epaint::image;
// use crate::player::Piggie;
use crate::states;
use crate::assets;
use belly::prelude::*;
use bevy::app::AppExit;

pub struct MainmenuPlugin;

impl Plugin for MainmenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(BellyPlugin)
        .add_event::<StartGameEvent>()
        .add_event::<QuitGameEvent>()
        .add_system(setup_mainmenu.in_schedule(OnEnter(states::GameState::MainMenu)))

        .add_system(start_game_event_handler.run_if(in_state(states::GameState::MainMenu)))
        .add_system(quit_game_event_handler.run_if(in_state(states::GameState::MainMenu)))
        
        .add_system(animate_piggie_face.run_if(in_state(states::GameState::MainMenu)))
        .add_system(animate_title.run_if(in_state(states::GameState::MainMenu)))
        // .add_system(update_mainmenu.run_if(in_state(states::GameState::MainMenu)))
        ;
    }
}

struct StartGameEvent {}
struct QuitGameEvent {}


fn start_game_event_handler(
    mut events: EventReader<StartGameEvent>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
    query: Query<Entity, With<Transform>>,
    mut commands: Commands
) {
    for _event in events.iter() {
        // info!("StartGameEvent");
        for e in query.iter() {
            commands.entity(e).despawn_recursive();
        }
        next_state.set(crate::states::GameState::InGame);
    }
}

fn quit_game_event_handler(
    mut events: EventReader<QuitGameEvent>,
    mut exit: EventWriter<AppExit>,
) {
    for _event in events.iter() {
        // info!("QuitGameEvent");
        exit.send(AppExit);
    }
}

#[derive(Component)]
struct PiggieFace {}

#[derive(Component)]
struct TitleText {}

fn animate_piggie_face(
    time: Res<Time>,
    mut query: Query<(&PiggieFace, &mut Transform)>,
) {
    let wave_speed = 1.0;
    let wave_amplitude_x = 5.0;
    let wave_amplitude_y = 10.0;
    let rotation_speed = 2.;
    let rotation_amplitude = std::f32::consts::PI / 9.0; // 30 degrees

    let offset_y = -170.;
    let offset_x = -220.;

    for (_, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;

        // Update the x position using a sine wave motion.
        translation.x = offset_x + wave_amplitude_x * (time.elapsed_seconds() * wave_speed).sin();

        // Update the y position using a sine wave motion with a phase shift.
        translation.y = offset_y + wave_amplitude_y * ((time.elapsed_seconds() * wave_speed) + std::f32::consts::PI / 2.0).sin();

        let rotation = &mut transform.rotation;

        // Rotate the sprite slightly.
        *rotation = Quat::from_rotation_z(
            rotation_amplitude * (time.elapsed_seconds() * rotation_speed).sin(),
        );
    }
}

fn animate_title(
    time: Res<Time>,
    mut query: Query<(&TitleText, &mut Transform)>,
) {
    let wave_speed = 1.0;
    let wave_amplitude_x = 300.0;
    let wave_amplitude_y = 50.0;
    let rotation_speed = 1.;
    let rotation_amplitude = std::f32::consts::PI / 13.0; // 30 degrees

    let offset_y = 200.;
    let offset_x = 0.;

    for (_, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;

        // Update the x position using a sine wave motion.
        translation.x = offset_x + wave_amplitude_x * (time.elapsed_seconds() * wave_speed).sin();

        // Update the y position using a sine wave motion with a phase shift.
        translation.y = offset_y + wave_amplitude_y * ((time.elapsed_seconds() * wave_speed) + std::f32::consts::PI / 2.0).sin();

        let rotation = &mut transform.rotation;

        // Rotate the sprite slightly.
        *rotation = Quat::from_rotation_z(
            rotation_amplitude * (time.elapsed_seconds() * rotation_speed).sin(),
        );
    }
}


fn setup_mainmenu(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    image_assets: Res<assets::ImageAssets>,
    windows: Query<&Window>,
    assets: Res<Assets<Image>>,
) {
    // println!("In main menu");
    commands.spawn(Camera2dBundle::default());
    // commands.spawn(EmlScene::new(asset_server.load("mainmenu.eml")));

    commands.spawn(
        crate::utils::create_bg(
            &image_assets.main_menu_bg.clone(),
            assets,
            windows
        )
    );

    let mut piggie_transform = Transform::from_scale(Vec3::splat(1.0));
    
    piggie_transform.translation.x = 200.0;
    piggie_transform.translation.y = 200.0;
    
    commands.spawn(SpriteBundle {
        texture: image_assets.piggie_face.clone(),
        transform: piggie_transform,
        ..default()
    }).insert(PiggieFace{});

    let mut title_transform = Transform::from_scale(Vec3::splat(1.0));
    
    title_transform.translation.x = 0.0;
    title_transform.translation.y = 200.0;
    title_transform.translation.z = 10.0;

    commands.spawn(SpriteBundle {
        texture: image_assets.title.clone(),
        transform: title_transform,
        ..default()
    }).insert(TitleText{});

    let font: Handle<Font> = image_assets.font.clone();

    commands.add(eml! {
        <body>
            <div s:align-items="center" s:justify-content="center" s:width="100%" s:height="100%">
                <button c:green on:press=|ctx| ctx.send_event(StartGameEvent {}) s:font=font>
                    "Run for your life"
                </button>
                <button c:red on:press=|ctx| ctx.send_event(QuitGameEvent {})>
                    "Give up on life"
                </button>
            </div>
        </body>
    });
    commands.add(StyleSheet::parse(
        r#"
        * {
            font: "fonts/Yomogi-Regular.ttf"
        }
        .green .button-foreground {
            background-color: green;
            color: white;
        }
        .red .button-foreground {
            background-color: red;
            color: white;
        }
        button:hover > span > .button-foreground {
            background-color: white;
            color: black;
        }
    "#,
    ));
}
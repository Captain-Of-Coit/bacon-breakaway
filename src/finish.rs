use belly::build::PropertyValue;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use belly::prelude::*;

use crate::states;
use crate::player;

pub struct FinishPlugin;

impl Plugin for FinishPlugin {
    fn build(&self, app: &mut App) {
        app
        // .insert_resource(TilemapRenderSettings {
        //     render_chunk_size: RENDER_CHUNK_SIZE,
        //     ..Default::default()
        // })
        // .add_plugin(TilemapPlugin)
        // // .insert_resource(ChunkManager::default())

        .add_system(check_if_finished.run_if(in_state(states::GameState::InGame)))
        // .add_system(despawn_outofrange_chunks.run_if(in_state(states::GameState::InGame)))

        .add_system(setup_finish_animation.in_schedule(OnEnter(states::GameState::Finished)))


        .add_system(check_if_piggie_post_finish.run_if(in_state(states::GameState::Finished)))

        .add_system(move_player_up.run_if(in_state(states::GameState::Finished)))

        
        .add_system(crate::utils::remove_all.in_schedule(OnExit(states::GameState::Finished)))

        .add_system(create_credits_resource.in_schedule(OnEnter(states::GameState::Credits)))

        
        .add_event::<BackToMainMenuEvent>()

        .add_system(setup_credits.in_schedule(OnEnter(states::GameState::Credits)))
        .add_system(animate_credits.run_if(in_state(states::GameState::Credits)))

        .add_system(start_game_event_handler.run_if(in_state(states::GameState::Credits)))
        
        // .add_system(desetup_map.in_schedule(OnExit(states::GameState::InGame)))
        ;
    }
}

fn check_if_finished(
    piggie: Query<&Transform, With<crate::player::Piggie>>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
) {
    let piggie_pos_y = piggie.get_single().unwrap().translation.y;

    let is_finished = piggie_pos_y > crate::constants::FINISH_DISTANCE;

    if is_finished {
        next_state.set(crate::states::GameState::Finished);
    }
}

fn setup_finish_animation(
    mut commands: Commands,
    time: Res<Time>,
    mut ext_forces: Query<&mut ExternalForce, With<player::Piggie>>,
    mut query: Query<&mut Transform, With<player::Piggie>>,
) {
    commands.add(eml! {
        <div>
            <div s:flex-direction="column" s:align-items="center" s:justify-content="center" s:width="100%" s:height="100%" s:color="black">
                <div s:font-size="36" s:color="black">
                    "You made it! Piggie escaped successfully!"
                </div>
            </div>
        </div>
    });

    // for mut ext_force in ext_forces.iter_mut() {
    //     let mut velocity = Vec2::ZERO;
    //     velocity.y = 3000.0;
    //     ext_force.force = velocity * time.delta_seconds();
    //     for mut transform in &mut query {
    //         transform.rotation = Quat::from_rotation_z(0.0);
    //     }
    // }
}

fn move_player_up(
    mut query: Query<&mut Transform, With<player::Piggie>>,
    time: Res<Time>,
) {
    let mut transform = query.get_single_mut().unwrap();

    transform.translation.y += 100.0 * time.delta_seconds();
}

fn check_if_piggie_post_finish(
    piggie: Query<&Transform, With<crate::player::Piggie>>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
) {
    let piggie_pos_y = piggie.get_single().unwrap().translation.y;

    let is_finished = piggie_pos_y > (crate::constants::FINISH_DISTANCE + 400.0);

    if is_finished {
        next_state.set(crate::states::GameState::Credits);
        // println!("Moving to the credits screen");
    }
}

struct BackToMainMenuEvent {}

fn start_game_event_handler(
    mut events: EventReader<BackToMainMenuEvent>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
    query: Query<Entity, With<Transform>>,
    mut commands: Commands
) {
    for _event in events.iter() {
        for e in query.iter() {
            commands.entity(e).despawn_recursive();
        }
        next_state.set(crate::states::GameState::MainMenu);
    }
}

#[derive(Resource)]
pub struct CreditsAnim {
    pub margin_top: f32,
    pub margin_top_str: PropertyValue,
}

impl Default for CreditsAnim {
    fn default() -> Self {
        CreditsAnim {
            margin_top: 1100.0,
            margin_top_str: PropertyValue::new("1200px")
        }
    }
}

fn create_credits_resource(
    mut commands: Commands
) {
    commands.remove_resource::<CreditsAnim>();
    commands.insert_resource(CreditsAnim::default());
}

fn animate_credits(
    // mut query: Query<&mut Style, (With<Transform>, With<Node>, With<Children>)>,
    mut elements: Elements,
    time: Res<Time>,
    mut credits_anim: ResMut<CreditsAnim>,
    mut q_transform: Query<&mut Style>
) {
    // let divs: Vec<Mut<Style>> = query.iter_mut().collect();

    // let mut div = divs.first().unwrap();
    
    if credits_anim.margin_top <= 50.0 {
        return
    }

    credits_anim.margin_top -= (90.0 * time.delta_seconds()) as f32;
    // credits_anim.margin_top -= 10.0;

    // println!("credits: {}", credits_anim.margin_top);

    for entity in elements.select(".text").entities() {

        let mut transform = q_transform.get_mut(entity).unwrap();

        transform.margin.top = Val::Px(credits_anim.margin_top);

        // transform.translation.y += credits_anim.margin_top;

        // println!("new translation: {}", transform.translation.y);

        // do whatever you want
        // let style: &mut Style = &mut elements.get_component_mut(entity).unwrap();

        // elements.

        // style.margin.top = Val::Px(credits_anim.margin_top * time.delta_seconds());
      }

    // div.margin.top = Val::Px(credits_anim.margin_top * time.delta_seconds());
}

#[derive(Component)]
struct CreditsText {}

fn setup_credits(
    mut commands: Commands,
    image_assets: Res<crate::assets::ImageAssets>,
    windows: Query<&Window>,
    assets: Res<Assets<Image>>,
) {
    // println!("Writing credits scene");

    let camera = Camera2dBundle::default();

    // camera.transform.translation.y -= 1000.0;

    commands.spawn(
        crate::utils::create_bg(
            &image_assets.credits_bg.clone(),
            assets,
            windows
        )
    );

    commands.spawn(camera);

    let gui = eml! {
        <div c:text>
            <div s:flex-direction="column" s:align-items="center" s:justify-content="center" s:width="100%" s:height="100%" s:color="white">
                <div s:font-size="36" s:color="white" s:margin-bottom="10%" s:margin-left="30%">
                    "You successfully helped Piggie to escape, nicely done!"
                </div>
                <div s:font-size="22" s:color="white" s:margin-bottom="5%">
                    "Game made for Bevy Jam #3"
                </div>
                <div s:font-size="22" s:color="white" s:margin-bottom="5%">
                    "Game Design: Captain_of_Coit & Captain_of_Coit's Wife"
                </div>
                <div s:font-size="22" s:color="white" s:margin-bottom="5%">
                    "Programming: Captain_of_Coit"
                </div>
                <div s:font-size="22" s:color="white" s:margin-bottom="5%">
                    "Graphics: Captain_of_Coit's Wife"
                </div>
                <div s:font-size="22" s:color="white" s:margin-bottom="5%">
                    "Music: Captain_of_Coit"
                </div>
                <div s:font-size="22" s:color="white" s:margin-bottom="5%">
                    "Made with Bevy 0.10.1"
                </div>
                <button c:green on:press=|ctx| ctx.send_event(BackToMainMenuEvent {}) >
                    "Back to Main Menu"
                </button>
            </div>
        </div>
    };

    // let text = commands.spawn_empty().id();

    // gui.add_to(text);

    // commands.add(gui);    

    // commands.add(gui);

    // commands.spawn_empty().with_children(|parent| {
    //     // parent.spawn(MyChildBundle::default());
    //     gui.add_to(parent);
    // });

    
    // gui.add_to(text.id());

    // text.insert(CreditsText{});

    // gui.add_to(text);

    commands.add(gui);

}
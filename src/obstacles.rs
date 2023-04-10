use std::time::Duration;

use crate::assets::ImageAssets;
use crate::states;
use bevy::prelude::*;
// use bevy::render::camera;
use bevy_rapier2d::prelude::*;
use rand::Rng;

pub struct ObstaclesPlugin;

impl Plugin for ObstaclesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnObstacleTimer(Timer::from_seconds(
            0.2,
            TimerMode::Repeating,
        )))
        .add_system(spawn_obstacles.run_if(in_state(states::GameState::InGame)))
        .add_system(update_spawn_timer.run_if(in_state(states::GameState::InGame)))
        ;
    }
}

#[derive(Resource)]
struct SpawnObstacleTimer(Timer);

#[derive(Component)]
struct Obstacle {}

fn update_spawn_timer(
    mut timer: ResMut<SpawnObstacleTimer>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    let camera_y = camera_query.get_single().unwrap().translation.y;

    let min_spawn_rate = 50.0;
    let max_spawn_rate = 200.0;
    let target_camera_y = crate::constants::FINISH_DISTANCE;

    let spawn_rate = max_spawn_rate - ((camera_y / target_camera_y) * (max_spawn_rate - min_spawn_rate)).clamp(0.0, max_spawn_rate - min_spawn_rate);
    
    let new_duration = Duration::from_millis(spawn_rate as u64);

    timer.0.set_duration(new_duration);
}


fn spawn_obstacles(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    camera_query: Query<&Transform, With<Camera>>,
    mut timer: ResMut<SpawnObstacleTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen_range(-700.0..700.0);

        let camera_y = camera_query.get_single().unwrap().translation.y;

        let y = camera_y + 500.0;

        let probability_of_cow = (camera_y / crate::constants::FINISH_DISTANCE).clamp(0.0, 0.7) as f64;

        let is_cow = rng.gen_bool(probability_of_cow);

        let mut mass = AdditionalMassProperties::Mass(20.0);
        let mut texture = image_assets.obstacle.clone();
        let mut collider = Collider::cuboid(25.0, 25.0);

        if is_cow {
            mass = AdditionalMassProperties::Mass(200.0);
            texture = image_assets.cow.clone();
            collider = Collider::cuboid(35.0, 35.0);
        }

        commands
            .spawn(RigidBody::Dynamic)
            .insert(GravityScale(0.0))
            .insert(mass)
            .insert(Damping {
                linear_damping: 10.0,
                angular_damping: 100.0,
            })
            .insert(collider)
            .insert(SpriteBundle {
                texture: texture,
                transform: Transform::default().with_translation(Vec3::new(x, y, 10.)),
                ..default()
            })
            .insert(Obstacle {});
    }
}

// fn spawn_obstacles(
//     mut commands: Commands,
//     image_assets: Res<ImageAssets>,
// ) {
//     // TODO should continiously spawn objects instead
//     let num_obstacles = 1000;
//

//     for _ in 0..num_obstacles {
//         let x: f32 = rng.gen_range(-5000.0..5000.0);
//         let y: f32 = rng.gen_range(-5000.0..5000.0);

//         commands
//             // .spawn(RigidBody::Fixed)
//             .spawn(RigidBody::Dynamic)
//             .insert(GravityScale(0.0))
//             .insert(Damping {
//                 linear_damping: 10.0,
//                 angular_damping: 100.0,
//             })
//             .insert(Collider::cuboid(25.0, 25.0))
//             .insert(
//                 SpriteBundle {
//                     texture: image_assets.obstacle.clone(),
//                     transform: Transform::default()
//                         .with_translation(Vec3::new(x, y, 10.)),
//                     ..default()
//                 },
//             )
//             .insert(Obstacle {});
//     }
// }

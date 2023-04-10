use std::time::Duration;

use bevy::prelude::*;
// use bevy_debug_text_overlay::screen_print;
use bevy_rapier2d::prelude::*;

use crate::assets::ImageAssets;
use crate::states;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PiggieStats>()
            .add_event::<DrainEnergyEvent>()
            // Startup
            .add_system(setup_player.in_schedule(OnEnter(states::GameState::InGame)))
            .add_system(stop_player.in_schedule(OnExit(states::GameState::InGame)))
            // Gameplay
            // .add_system(print_energy.run_if(in_state(states::GameState::InGame)))
            .add_system(move_player.run_if(in_state(states::GameState::InGame)))
            .add_system(update_energy.run_if(in_state(states::GameState::InGame)))
            .add_system(update_animation_timer.run_if(in_state(states::GameState::InGame)))

            .add_system(animate_sprite.run_if(in_state(states::GameState::InGame)))
            .add_system(animate_sprite.run_if(in_state(states::GameState::Finished)))
            ;
    }
}

#[derive(Resource)]
pub struct PiggieStats {
    pub corn: u32,
    pub speed: f32,
    pub turn_speed: f32,
    pub camera_speed: f32,
    pub energy: f32,
    pub fatness: f32,
    pub stamina: f32,
}

impl Default for PiggieStats {
    fn default() -> Self {
        PiggieStats {
            corn: crate::constants::STARTING_CORN,
            speed: 1.2,
            turn_speed: 1.0,
            camera_speed: crate::constants::INITIAL_CAMERA_SPEED,
            energy: crate::constants::MAX_ENERGY,
            fatness: 0.0,
            stamina: 0.0,
        }
    }
}

// impl PiggieStats {
//     pub fn energy(&self) -> f32 {
//         return self.energy
//     }
// }

#[derive(Component)]
pub struct Piggie {
    pub is_moving: bool
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
    query_piggie: Query<&Piggie>
) {
    let piggie = query_piggie.get_single().unwrap();
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if piggie.is_moving {
                let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            }
        }
    }
}

fn update_animation_timer(
    mut query: Query<&mut AnimationTimer, With<Piggie>>,
    piggiestats: Res<PiggieStats>,
) {
    let mut timer = query.get_single_mut().unwrap();

    // println!("{:#?}", timer.0.duration());

    let new_speed = 100.0 + (400.0 - (piggiestats.energy * 4.0));

    let new_speed = new_speed.max(1.0);

    let new_duration = Duration::from_millis(new_speed as u64);

    timer.0.set_duration(new_duration);
}

fn setup_player(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    piggie_stats: Res<PiggieStats>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = image_assets.player_anim.clone();

    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(128.0, 128.0), 2, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // let player_size = 0.5;

    let player_fatness_percent = ((piggie_stats.fatness * 2.0) / 100.0).clamp(0.5, 1.0);

    // let player_size = 0.5 * player_fatness_percent;
    // println!("Fatty: {:#?}", player_fatness_percent);
    // let player_size = 0.5;
    let player_size = player_fatness_percent;

    let mut sprite_transform = Transform::from_scale(Vec3::splat(player_size));
    sprite_transform.translation.z = 10.0;

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(50.0 * player_fatness_percent, 50.0 * player_fatness_percent))
        .insert(GravityScale(0.0))
        .insert(AdditionalMassProperties::Mass(10.0 * player_fatness_percent)) // TODO should depend on fatness
        .insert(Damping {
            linear_damping: 10.0,
            angular_damping: 100.0,
        })
        .insert(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: sprite_transform,
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)))
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(Piggie {
            is_moving: false
        });
}

struct DrainEnergyEvent(f32);

fn stop_player(
    // velocities: Query<&Velocity, With<Piggie>>,
    mut ext_forces: Query<&mut ExternalForce, With<Piggie>>,
) {
    for mut ext_force in ext_forces.iter_mut() {
        ext_force.force = Vec2::ZERO;
    }
}

fn move_player(
    time: Res<Time>,
    piggie_stats: Res<PiggieStats>,
    keys: Res<Input<KeyCode>>,
    mut query_piggie: Query<&mut Piggie>,
    mut query: Query<&mut Transform, With<Piggie>>,
    mut ext_forces: Query<&mut ExternalForce, With<Piggie>>,
    velocities: Query<&Velocity, With<Piggie>>,
    mut ev_drain: EventWriter<DrainEnergyEvent>,
) {
    let mut piggie = query_piggie.get_single_mut().expect("no piggie");
    if piggie_stats.energy <= 0.0 {
        piggie.is_moving = false;
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = Vec2::ZERO;
        }
        return;
    }

    for mut transform in &mut query {
        let mut velocity = Vec2::ZERO;




        let mut new_speed = 90.0 * piggie_stats.speed;

        // if piggie_stats.energy < 40.0 {
        //     new_speed = (piggie_stats.energy * 2.5) * piggie_stats.speed;
        // }

        let fatness_percent = piggie_stats.fatness / 100.0;
        let fatness_factor = 1.0; // Adjust this value to control the impact of fatness on new_speed

        new_speed = new_speed * (1.0 - fatness_percent * fatness_factor).clamp(0.0, 3.0);

        // println!("new speed: {}", new_speed);




        // let new_speed = piggie_stats.energy * crate::constants::ENERGY_MOVE_SPEED_MULTIPLIER;
        // let new_speed = piggie_stats.speed + (piggie_stats.energy * 10.0 / (piggie_stats.energy + 10.0));
        // let new_speed = piggie_stats.speed + (piggie_stats.energy * 10.0 / (piggie_stats.energy + 10.0));
        // let new_speed = if piggie_stats.energy > 0.0 {
        //     piggie_stats.speed + (10.0 * piggie_stats.energy / piggie_stats.energy.max(10.0))
        //     } else {
        //     piggie_stats.speed
        //     };

        if keys.pressed(KeyCode::Up) || keys.pressed(KeyCode::W) {
            velocity.y += new_speed
            // velocity.y += piggie_stats.speed;
        }

        if keys.pressed(KeyCode::Down) || keys.pressed(KeyCode::S) {
            // velocity.y -= piggie_stats.speed;
            velocity.y -= new_speed;
        }

        if keys.pressed(KeyCode::Right) || keys.pressed(KeyCode::D) {
            // velocity.x += piggie_stats.turn_speed;
            velocity.x += new_speed / 2.0;
        }

        if keys.pressed(KeyCode::Left) || keys.pressed(KeyCode::A) {
            // velocity.x -= piggie_stats.turn_speed;
            velocity.x -= new_speed / 2.0;
        }

        if velocity.length() > 0.0 {
            // Normalize the velocity vector
            // velocity = velocity.normalize();
            // Calculate the angle between the current direction and the velocity vector

            // Update the translation based on the velocity
            // transform.translation += velocity * time.delta_seconds();
            for mut ext_force in ext_forces.iter_mut() {
                piggie.is_moving = true;
                
                let fatness_mod = 10000.0 * (piggie_stats.fatness / 20.0).max(1.0);

                ext_force.force = (velocity * fatness_mod) * time.delta_seconds();
                let mut to_drain = ext_force.force.x.abs() + ext_force.force.y.abs();
                to_drain = to_drain / 200000.0;
                // println!("Drained: {}", to_drain);
                ev_drain.send(DrainEnergyEvent(to_drain));
            }

            // let angle = velocity.y.atan2(velocity.x) - std::f32::consts::FRAC_PI_2;
            // transform.rotation = Quat::from_rotation_z(angle);
        } else {
            piggie.is_moving = false;
            for mut ext_force in ext_forces.iter_mut() {
                ext_force.force = Vec2::ZERO;
            }
        }

        for vel in velocities.iter() {
            let angle = vel.linvel.y.atan2(vel.linvel.x) - std::f32::consts::FRAC_PI_2;
            transform.rotation = Quat::from_rotation_z(angle);
        }

        // println!("{:#?}", transform.rotation);
    }
}

use bevy_easings::Lerp;

fn update_energy(
    mut ev_drain: EventReader<DrainEnergyEvent>,
    mut piggie_stats: ResMut<PiggieStats>,
) {
    for ev in ev_drain.iter() {
        if piggie_stats.energy > 0.0 {
            let min_energy_drain = ev.0 * 2.0; // Maximum energy drain when stamina is 0.0
            let max_energy_drain = ev.0 * 0.5; // Minimum energy drain when stamina is 1.0

            // Interpolate the energy drain based on the stamina value
            let energy_drain = f32::lerp(&min_energy_drain, &max_energy_drain, &piggie_stats.stamina);
            // println!("Drain: {}", energy_drain);
            piggie_stats.energy -= energy_drain.max(0.02);

            if piggie_stats.fatness > 0.0 {
                piggie_stats.fatness -= 0.01;
            }

            if piggie_stats.energy < 0.0 {
                piggie_stats.energy = 0.0;
            }

            piggie_stats.stamina = piggie_stats.stamina + 0.00005;
        }
    }
}
// fn print_energy(stats: Res<PiggieStats>) {
//     let energy = stats.energy.round();
//     screen_print!("Energy: {energy}");
//     // println!("Distance: {}", distance);
// }

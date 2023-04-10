use std::collections::HashSet;

use bevy::math::Vec3Swizzles;
// Everything related to the map
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::tiles::TileTextureIndex;

use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 64.0, y: 64.0 };
// For this example, don't choose too large a chunk size.
const CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };
// Render chunk sizes are set to 4 render chunks per user specified chunk.
const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 8,
    y: CHUNK_SIZE.y * 8,
};
// How far away a tile has to be before we despawn it
const DESPAWN_DISTANCE: f32 = 2000.0;

use crate::states;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(TilemapRenderSettings {
            render_chunk_size: RENDER_CHUNK_SIZE,
            ..Default::default()
        })
        .add_plugin(TilemapPlugin)
        // .insert_resource(ChunkManager::default())

        .add_system(spawn_chunks_around_camera.run_if(in_state(states::GameState::InGame)))
        .add_system(despawn_outofrange_chunks.run_if(in_state(states::GameState::InGame)))

        .add_system(setup_map.in_schedule(OnEnter(states::GameState::InGame)))
        .add_system(desetup_map.in_schedule(OnExit(states::GameState::InGame)))
        ;
    }
}

fn setup_map(
    mut commands: Commands
) {
    commands.insert_resource(TilemapRenderSettings {
        render_chunk_size: RENDER_CHUNK_SIZE,
        ..Default::default()
    });
    commands.insert_resource(ChunkManager::default());
}

fn desetup_map(
    mut commands: Commands
) {
    commands.remove_resource::<TilemapRenderSettings>();
    commands.remove_resource::<ChunkManager>();
}

#[derive(Component)]
struct Tile {}

fn get_random_tile() -> u32 {
    let weights = vec![
        1 + (10 * 80), // 0 - Grass
        1 + (10 * 1), // 1 - Dirt 10% chance
        1 + (10 * 20), // 2 - Grass
        1 + (10 * 20), // 3 - Grass
        0,            // 4 - Not used
        0,            // 5 - Not Used
    ];
    let dist = WeightedIndex::new(&weights).unwrap();
    let random_number = dist.sample(&mut rand::thread_rng());
    return random_number as u32;
}

fn spawn_chunk(
    commands: &mut Commands,
    asset_server: &AssetServer,
    chunk_pos: IVec2,
    is_finished: bool,
) {
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());
    // Spawn the elements of the tilemap.
    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let tile_pos = TilePos { x, y };
            let mut tile_index = get_random_tile();
            // println!("TileY: {}", chunk_pos.y);
            if is_finished {
                tile_index = 4
            }
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(tile_index),
                    ..Default::default()
                })
                .id();
            commands.entity(tilemap_entity).insert(Tile{}).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x,
        chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y,
        0.0,
    ));
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TILE_SIZE.into(),
        size: CHUNK_SIZE.into(),
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: TILE_SIZE,
        transform,
        ..Default::default()
    });
}

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
    camera_pos / (chunk_size * tile_size)
}

fn spawn_chunks_around_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkManager>,
    piggie: Query<&Transform, With<crate::player::Piggie>>
) {
    let piggie_pos_y = piggie.get_single().unwrap().translation.y;

    let is_finished = piggie_pos_y > (crate::constants::FINISH_DISTANCE - 1000.0);

    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        for y in (camera_chunk_pos.y - 2)..(camera_chunk_pos.y + 2) {
            for x in (camera_chunk_pos.x - 2)..(camera_chunk_pos.x + 2) {
                if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                    chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_chunk(&mut commands, &asset_server, IVec2::new(x, y), is_finished);
                }
            }
        }
    }
}

fn despawn_outofrange_chunks(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera>>,
    chunks_query: Query<(Entity, &Transform), With<Tile>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    for camera_transform in camera_query.iter() {
        for (entity, chunk_transform) in chunks_query.iter() {
            let chunk_pos = chunk_transform.translation.xy();
            let distance = camera_transform.translation.xy().distance(chunk_pos);
            if distance > DESPAWN_DISTANCE {
                let x = (chunk_pos.x / (CHUNK_SIZE.x as f32 * TILE_SIZE.x)).floor() as i32;
                let y = (chunk_pos.y / (CHUNK_SIZE.y as f32 * TILE_SIZE.y)).floor() as i32;
                chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

#[derive(Default, Debug, Resource)]
struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

use bevy::prelude::*;

pub fn remove_all(
  mut commands: Commands,
  all_transform_query: Query<Entity, With<Transform>>,
) {
  // println!("Removed all entities...");
  for e in all_transform_query.iter() {
    commands.entity(e).despawn_recursive();
  }
}

#[derive(Component)]
struct ImageBackground {}

pub fn create_bg(
  image: &Handle<Image>,
  assets: Res<Assets<Image>>,
  windows: Query<&Window>,
  // assets: Res<AssetServer>,
) -> SpriteBundle {

  let window = windows.get_single().unwrap();

  let image_as = assets.get(image).unwrap();

  let sprite_width = image_as.size().x as f32;
  let sprite_height = image_as.size().y as f32;

  let window_width = window.width();
  let window_height = window.height();
  
  let scale_x = window_width / sprite_width;
  let scale_y = window_height / sprite_height;

  let max_scale = scale_x.max(scale_y);
  let scale = Vec3::splat(max_scale);

  // let min_scale = scale_x.min(scale_y);
  // let scale = Vec3::splat(min_scale);

  SpriteBundle {
    texture: image.clone(),
    transform: Transform::from_scale(scale),
    ..default()
  }
}
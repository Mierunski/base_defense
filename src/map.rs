use bevy::prelude::*;

use crate::TILE_SIZE;

#[derive(Component)]
pub struct Map;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_simple_map);
    }
}

fn create_simple_map(mut commands: Commands) {
    let mut tiles = Vec::new();
    tiles.reserve_exact(10000);

    for y in -50..50 {
        for x in -50..50 {
            let tile = commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.8, 0.8, 0.8),
                        custom_size: Some(Vec2::splat(TILE_SIZE * 0.9)),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(y as f32 * TILE_SIZE, x as f32 * TILE_SIZE, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .id();
            tiles.push(tile);
        }
    }

    commands
        .spawn()
        .insert(Map)
        .insert(Name::new("Map"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);
}

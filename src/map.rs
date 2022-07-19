use std::iter::Map;

use bevy::prelude::*;

use crate::TILE_SIZE;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_simple_map);
    }
}

fn create_simple_map(mut commands: Commands) {
    for y in -50..50 {
        for x in -50..50 {
            commands.spawn_bundle(SpriteBundle {
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
            });
        }
    }
}

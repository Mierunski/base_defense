use crate::constants::*;
use bevy::prelude::*;
use noise::{utils::PlaneMapBuilder, OpenSimplex};
extern crate noise;
use bevy::prelude::Color;
use noise::utils::*;
#[derive(Component)]
pub struct Map;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_simple_map);
    }
}

fn create_simple_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut tiles = Vec::new();
    tiles.reserve_exact(10000);

    let simplex = OpenSimplex::default();

    let noise_map = PlaneMapBuilder::new(&simplex)
        .set_size(MAP_SIZE as usize, MAP_SIZE as usize)
        .set_x_bounds(-BOUND_SIZE, BOUND_SIZE)
        .set_y_bounds(-BOUND_SIZE, BOUND_SIZE)
        .build();
    for y in -MAP_SIZE / 2..MAP_SIZE / 2 {
        for x in -MAP_SIZE / 2..MAP_SIZE / 2 {
            let tile = commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.8, 0.8, 0.8),
                        custom_size: Some(Vec2::splat(TILE_SIZE * 0.9)),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(y as f32 * TILE_SIZE, x as f32 * TILE_SIZE, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .id();
            tiles.push(tile);

            let gray = (noise_map
                .get_value((x + MAP_SIZE / 2) as usize, (y + MAP_SIZE / 2) as usize)
                + 0.5)
                .clamp(0.0, 1.0) as f32;
            if gray > 0.8 {
                let gold = commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(1.0, 0.8, 0.0),
                            custom_size: Some(Vec2::splat(TILE_SIZE * 0.6)),
                            ..Default::default()
                        },
                        texture: asset_server.load("sprites/projectile.png"),
                        transform: Transform {
                            translation: Vec3::new(y as f32 * TILE_SIZE, x as f32 * TILE_SIZE, 1.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .id();
                tiles.push(gold)
            };
        }
    }

    commands
        .spawn_bundle(SpatialBundle::default())
        .insert(Map)
        .insert(Name::new("Map"))
        .push_children(&tiles);
}

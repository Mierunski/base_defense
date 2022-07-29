use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::TILE_SIZE;

pub struct EnemyPlugin;

#[derive(Component, Inspectable)]
pub struct Enemy {
    speed: f32,
    attack: f32,
    health: f32,
    max_hp: f32,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_enemies);
    }
}

fn update_enemies(mut commands: Commands) {}

impl Enemy {
    pub fn new(mut commands: Commands, mut translation: Vec3, asset_server: Res<AssetServer>) {
        translation.z = 10.0;
        let enemy = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.2, 0.8, 0.2),
                    custom_size: Some(Vec2::splat(TILE_SIZE * 0.7)),
                    ..Default::default()
                },
                texture: asset_server.load("sprites/tower.png"),
                transform: Transform {
                    translation,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy {
                attack: 10.0,
                max_hp: 100.0,
                speed: 0.2 * TILE_SIZE,
                health: 100.0,
            })
            .insert(Name::new("Enemy"))
            .id();
    }
}

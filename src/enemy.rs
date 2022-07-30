use bevy::{math::Vec3Swizzles, prelude::*, transform};
use bevy_inspector_egui::Inspectable;

use crate::{tower::Tower, TILE_SIZE};

pub struct EnemyPlugin;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy {
    speed: f32,
    attack: f32,
    pub health: f32,
    max_hp: f32,
    timer: Timer,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_enemies);
    }
}

fn update_enemies(
    mut commands: Commands,
    mut q_towers: Query<(Entity, &mut Tower, &mut Transform), Without<Enemy>>,
    mut q_enemies: Query<(Entity, &mut Enemy, &mut Transform), Without<Tower>>,
    time: Res<Time>,
) {
    for (entity, mut enemy, mut transform) in q_enemies.iter_mut() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        let mut longest = f32::INFINITY;
        let mut target = Vec3::ZERO;
        let mut target_tower = None;
        for (e_tower, mut tower, mut t_tower) in q_towers.iter_mut() {
            let diff = t_tower.translation.xy() - transform.translation.xy();
            let cur_len = diff.length();
            if cur_len < longest {
                longest = cur_len;
                target = diff.extend(transform.translation.z);
                target_tower = Some(tower);
                debug!(?diff, ?longest);
            }
        }
        if longest.is_infinite() {
            continue;
        }
        if longest < TILE_SIZE * 0.3 {
            enemy.timer.tick(time.delta());
            if enemy.timer.just_finished() {
                target_tower.unwrap().health -= enemy.attack;
            }
            continue;
        }
        let norm_target = target.xy().normalize().extend(1.0);
        let movement = norm_target * enemy.speed * time.delta_seconds();
        transform.translation += movement;
    }
}

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
                texture: asset_server.load("sprites/enemy.png"),
                transform: Transform {
                    translation,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy {
                attack: 10.0,
                max_hp: 100.0,
                speed: 3.0 * TILE_SIZE,
                health: 100.0,
                timer: Timer::from_seconds(0.5, true),
            })
            .insert(Name::new("Enemy"))
            .id();
    }
}

use std::cmp::Ordering;

use bevy::{math::Vec3Swizzles, prelude::*, transform};

use crate::{
    hp_bar::{create_hp_bar, Health},
    tower::Tower,
    TILE_SIZE,
};

pub struct EnemyPlugin;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy {
    speed: f32,
    attack: f32,
    timer: Timer,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_enemies);
    }
}

fn update_enemies(
    mut commands: Commands,
    mut q_towers: Query<(Entity, &mut Health, &mut Tower, &mut Transform), Without<Enemy>>,
    mut q_enemies: Query<(Entity, &mut Health, &mut Enemy, &mut Transform), Without<Tower>>,
    time: Res<Time>,
) {
    for (entity, mut health, mut enemy, mut transform) in q_enemies.iter_mut() {
        if health.current <= 0.0 {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        let pos: Vec2 = transform.translation.xy();
        let closest_tower = q_towers.iter_mut().min_by(|a, b| {
            if (a.3.translation.xy() - pos).length_squared()
                < (b.3.translation.xy() - pos).length_squared()
            {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        if closest_tower.is_none() {
            continue;
        }
        let (mut entity, mut health, tower, target) = closest_tower.unwrap();
        let diff = target.translation.xy() - pos;
        if diff.length() < TILE_SIZE * 0.3 {
            enemy.timer.tick(time.delta());
            if enemy.timer.just_finished() {
                health.current -= enemy.attack;
            }
            continue;
        }
        let norm_target = diff.normalize().extend(1.0);
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
                speed: 3.0 * TILE_SIZE,
                timer: Timer::from_seconds(0.5, true),
            })
            .insert(Name::new("Enemy"))
            .insert(Health {
                current: 100.0,
                max: 100.0,
            })
            .id();
        let hp_bar = create_hp_bar(
            &mut commands,
            Vec2::new(0.0, TILE_SIZE * 0.5),
            Vec2::new(TILE_SIZE * 0.85, TILE_SIZE * 0.1),
            enemy,
        );
        commands.entity(enemy).add_child(hp_bar);
    }
}

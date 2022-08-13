use std::{cmp::Ordering, f32::consts::PI};

use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_inspector_egui::Inspectable;

use crate::{
    enemy::Enemy,
    hp_bar::{create_hp_bar, Health},
    projectile::{Projectile, PROJECTILE_LAYER},
    TILE_SIZE,
};

pub struct TowerPlugin;

#[derive(Component, Inspectable)]
pub struct Tower {
    pub health: f32,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct AttackTimer {
    timer: Timer,
}

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_towers);
    }
}

fn update_towers(
    mut commands: Commands,
    mut q_towers: Query<
        (Entity, &mut Health, &mut Transform, &mut AttackTimer),
        (With<Tower>, Without<Enemy>),
    >,
    q_enemies: Query<(Entity, &mut Enemy, &mut Transform), (Without<Tower>, With<Enemy>)>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    for (entity, health, mut transform, mut attack_timer) in q_towers.iter_mut() {
        if health.current < 0.0 {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        let pos: Vec2 = transform.translation.truncate();
        let mut target: Vec2 = Vec2::new(0.0, 0.0);

        let closest_enemy = q_enemies.iter().min_by(|a, b| {
            if (a.2.translation.xy() - pos).length_squared()
                < (b.2.translation.xy() - pos).length_squared()
            {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        if let Some(x) = closest_enemy {
            target = x.2.translation.xy();
        }

        let angle = (pos - target).angle_between(Vec2::new(1.0, 0.0)) - PI / 2.0;
        if !angle.is_nan() {
            transform.rotation = Quat::from_rotation_z(-angle);
        }

        if let Some(_) = closest_enemy {
            attack_timer.timer.tick(time.delta());
            if attack_timer.timer.just_finished() {
                Projectile::spawn(
                    &mut commands,
                    transform.translation.xy().extend(PROJECTILE_LAYER),
                    transform.rotation.mul_vec3(Vec3::Y).xy(),
                    &asset_server,
                );
            }
        }
    }
}

impl Tower {
    pub fn create_tower(mut commands: Commands, translation: Vec3, asset_server: Res<AssetServer>) {
        let trans = Transform {
            translation,
            ..Default::default()
        };
        let tower = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.2, 0.2),
                    custom_size: Some(Vec2::splat(TILE_SIZE * 0.7)),
                    ..Default::default()
                },
                texture: asset_server.load("sprites/tower.png"),
                transform: trans,
                ..Default::default()
            })
            .insert(Tower { health: 100.0 })
            .insert(Name::new("Tower"))
            .insert(AttackTimer {
                timer: Timer::from_seconds(1.0, true),
            })
            .insert(Health {
                current: 100.0,
                max: 100.0,
            })
            .id();
        let hp_bar = create_hp_bar(
            &mut commands,
            Vec2::new(0.0, TILE_SIZE * 0.5),
            Vec2::new(TILE_SIZE * 0.85, TILE_SIZE * 0.1),
            tower,
        );
        commands.entity(tower).add_child(hp_bar);
    }
}

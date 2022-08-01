use bevy::{prelude::*, sprite::collide_aabb::collide};
// use bevy_inspector_egui::Inspectable;

use crate::{enemy::Enemy, hp_bar::Health, TILE_SIZE};

pub const PROJECTILE_LAYER: f32 = 20.0;
pub struct ProjectilePlugin;

#[derive(Component)]
pub struct Projectile {
    damage: f32,
    speed: f32,
    direction: Vec2,
}

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_projectiles);
    }
}

fn update_projectiles(
    mut commands: Commands,
    mut q_projectiles: Query<
        (Entity, &mut Transform, &Projectile),
        (With<Projectile>, Without<Enemy>),
    >,
    mut q_enemies: Query<(&mut Health, &mut Transform), With<Enemy>>,
    time: Res<Time>,
) {
    for (entity, mut transform, projectile) in q_projectiles.iter_mut() {
        let delta = (projectile.direction * projectile.speed * time.delta_seconds()).extend(0.0);
        transform.translation += delta;

        let e = q_enemies.iter_mut().find(|x| {
            collide(
                x.1.translation,
                Vec2::splat(TILE_SIZE * 0.7),
                transform.translation,
                Vec2::splat(TILE_SIZE * 0.2),
            )
            .is_some()
        });

        if let Some((mut enemy_hp, _)) = e {
            enemy_hp.current -= projectile.damage;
            commands.entity(entity).despawn_recursive();
        }
    }
}

impl Projectile {
    pub fn spawn(
        mut commands: &mut Commands,
        translation: Vec3,
        direction: Vec2,
        asset_server: &Res<AssetServer>,
    ) {
        let proj = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.2, 0.2, 0.8),
                    custom_size: Some(Vec2::splat(TILE_SIZE * 0.2)),
                    ..Default::default()
                },
                texture: asset_server.load("sprites/projectile.png"),
                transform: Transform {
                    translation,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Projectile {
                damage: 20.0,
                speed: 1.0,
                direction,
            })
            .insert(Name::new("Projectile"))
            .id();
    }
}

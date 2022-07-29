use std::f32::consts::PI;

use bevy::{prelude::*, sprite::Anchor};
use bevy_inspector_egui::Inspectable;

use crate::TILE_SIZE;

pub struct TowerPlugin;

#[derive(Component, Inspectable)]
pub struct Tower {
    health: f32,
    hp_bar: Entity,
}

#[derive(Component)]
pub struct HPBar;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_towers);
    }
}

fn update_towers(
    mut commands: Commands,
    mut q_towers: Query<(Entity, &Children, &mut Tower, &mut Transform)>,
    mut q_bars: Query<&mut Sprite, With<HPBar>>,
    time: Res<Time>,
) {
    let mut positions = Vec::new();
    for (entity, children, mut tower, mut transform) in q_towers.iter_mut() {
        for child in children.iter() {
            if let Ok(mut sprite) = q_bars.get_mut(*child) {
                if let Some(size) = sprite.custom_size.as_mut() {
                    size.x = (TILE_SIZE * 0.85) * (tower.health / 100.0);
                }
            }
        }
        let pos: Vec2 = transform.translation.truncate();
        let target: Vec2 = Vec2::new(0.0, 0.0);

        let angle = (pos - target).angle_between(Vec2::new(1.0, 0.0)) - PI / 2.0;
        if !angle.is_nan() {
            positions.push((pos, angle.to_degrees()));
            transform.rotation = Quat::from_rotation_z(-angle);
        }
        if !tower.update(time.delta_seconds()) {
            commands.entity(entity).despawn_recursive();
        }
    }
    for (p, d) in positions {
        print!("{} {},", p, d);
    }
    print!("\n");
}

impl Tower {
    fn update(&mut self, time: f32) -> bool {
        self.health -= 5.0 * time;
        if self.health < 0.0 {
            return false;
        }
        true
    }

    pub fn create_tower(mut commands: Commands, translation: Vec3, asset_server: Res<AssetServer>) {
        let hp_frame = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.1, 0.1, 0.1),
                    custom_size: Some(Vec2::new(TILE_SIZE * 0.9, TILE_SIZE * 0.15)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, TILE_SIZE * 0.5, 10.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();

        let hp_bar = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.1, 0.9, 0.1),
                    custom_size: Some(Vec2::new(TILE_SIZE * 0.85, TILE_SIZE * 0.1)),
                    anchor: Anchor::CenterLeft,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(-TILE_SIZE * 0.425, TILE_SIZE * 0.5, 11.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(HPBar)
            .id();

        let tower = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.2, 0.2),
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
            .insert(Tower {
                health: 100.0,
                hp_bar,
            })
            .insert(Name::new("Tower"))
            .id();
        commands.entity(tower).add_child(hp_frame).add_child(hp_bar);
    }
}
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::TILE_SIZE;

pub struct TowerPlugin;

#[derive(Component, Inspectable)]
pub struct Tower {
    health: f32,
}

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_towers);
    }
}

fn update_towers(
    mut commands: Commands,
    mut q_towers: Query<(Entity, &mut Tower)>,
    time: Res<Time>,
) {
    for (entity, mut tower) in q_towers.iter_mut() {
        if !tower.update(time.delta_seconds()) {
            commands.entity(entity).despawn();
        }
    }
}

impl Tower {
    fn update(&mut self, time: f32) -> bool {
        self.health -= 5.0 * time;
        if self.health < 0.0 {
            return false;
        }
        true
    }

    pub fn create_tower(mut commands: Commands, translation: Vec3) {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.2, 0.2),
                    custom_size: Some(Vec2::splat(TILE_SIZE * 0.7)),
                    ..Default::default()
                },
                transform: Transform {
                    translation,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Tower { health: 100.0 })
            .insert(Name::new("Tower"));
    }
}

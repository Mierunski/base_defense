use bevy::{prelude::*, sprite::Anchor};

use crate::constants::*;

#[derive(Component)]
pub struct HPBar {
    parent: Entity,
    offset: Vec3,
    size: Vec2,
}

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}
pub struct HPBarsPlugin;

impl Plugin for HPBarsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_hp_bars);
    }
}

pub fn update_hp_bars(
    mut q_bars: Query<(&HPBar, &mut Sprite, &mut Transform), With<HPBar>>,
    q_units: Query<(&Transform, &Health), Without<HPBar>>,
) {
    for (bar, mut sprite, mut local) in q_bars.iter_mut() {
        // local.rotation = local.rotation - global.rotation;
        // let a = global.rotation;

        if let Ok((transform, hp)) = q_units.get(bar.parent) {
            let inverse = transform.rotation.inverse();
            local.translation = inverse.mul_vec3(bar.offset);
            local.rotation = inverse;
            if let Some(size) = sprite.custom_size.as_mut() {
                size.x = bar.size.x * (hp.current / hp.max);
            }
        }
    }
}

pub fn create_hp_bar(commands: &mut Commands, offset: Vec2, size: Vec2, parent: Entity) -> Entity {
    let hp_frame = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.1, 0.1),
                custom_size: Some(Vec2::new(
                    size.x + TILE_SIZE * 0.05,
                    size.y + TILE_SIZE * 0.05,
                )),
                anchor: Anchor::CenterLeft,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(-TILE_SIZE * 0.025, 0.0, -0.1),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.9, 0.1),
                custom_size: Some(size),
                anchor: Anchor::CenterLeft,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(-size.x / 2.0, offset.y, 11.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(HPBar {
            parent,
            offset: Vec3::new(-size.x / 2.0, offset.y, 11.0),
            size,
        })
        .add_child(hp_frame)
        .id()
}

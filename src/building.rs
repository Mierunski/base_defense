use crate::{constants::*, PlayerResources};
use bevy::prelude::*;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_miners);
    }
}

fn update_miners(
    mut res: ResMut<PlayerResources>,
    mut q_miners: Query<&mut Miner>,
    time: Res<Time>,
) {
    for mut miner in q_miners.iter_mut() {
        miner.mine_timer.tick(time.delta());
        if miner.mine_timer.just_finished() {
            res.gold += miner.gold;
        }
    }
}

#[derive(Component)]
pub struct Miner {
    mine_timer: Timer,
    gold: f32,
}

impl Miner {
    pub fn new(mut commands: Commands, translation: Vec3, asset_server: Res<AssetServer>) {
        let trans = Transform {
            translation,
            ..Default::default()
        };
        let miner = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_MINER,
                    custom_size: Some(Vec2::splat(TILE_SIZE * 0.7)),
                    ..Default::default()
                },
                texture: asset_server.load("sprites/tower.png"),
                transform: trans,
                ..Default::default()
            })
            .insert(Miner {
                mine_timer: Timer::from_seconds(1.0, true),
                gold: 10.0,
            })
            .insert(Name::new("Miner"));
    }
}

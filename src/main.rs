use crate::{building::BuildingPlugin, networking::NetworkingPlugin};
use bevy::{
    log::{Level, LogSettings},
    prelude::*,
};
use debug::DebugPlugin;
use enemy::EnemyPlugin;
use hp_bar::HPBarsPlugin;
use map::MapPlugin;
use projectile::ProjectilePlugin;
use tower::TowerPlugin;
use user_interface::UserInterfacePlugin;
extern crate noise;
use constants::*;

mod building;
mod constants;
mod debug;
mod enemy;
mod hp_bar;
mod map;
mod networking;
mod projectile;
mod tower;
mod user_interface;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Main,
    Building,
}

pub struct PlayerResources {
    pub gold: f32,
}

fn main() {
    println!("Usage: server [SERVER_PORT] or client [SERVER_PORT] [USER_NAME]");
    let args: Vec<String> = std::env::args().collect();

    let height = 900.0;
    App::new()
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "Base defense".to_string(),
            present_mode: bevy::window::PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(LogSettings {
            level: Level::TRACE,
            filter: "info,wgpu_core=warn,wgpu_hal=warn,base_defense::projectile=debug".to_string(),
        })
        .insert_resource(PlayerResources {
            gold: STARTING_GOLD,
        })
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Main)
        .add_plugin(NetworkingPlugin::new(&args))
        .add_plugin(MapPlugin)
        .add_plugin(TowerPlugin)
        .add_plugin(UserInterfacePlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(HPBarsPlugin)
        .add_plugin(BuildingPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}

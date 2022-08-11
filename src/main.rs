use crate::user_interface::Icons;
use crate::{networking::NetworkingPlugin, tower::Tower};
use bevy::{
    log::{Level, LogSettings},
    prelude::*,
    render::camera::{RenderTarget, ScalingMode},
};
use debug::DebugPlugin;
use enemy::{Enemy, EnemyPlugin};
use hp_bar::HPBarsPlugin;
use map::MapPlugin;
use projectile::ProjectilePlugin;
use tower::TowerPlugin;
use user_interface::{RightPanelWidth, UserInterfacePlugin};

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.15;

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

fn main() {
    println!("Usage: server [SERVER_PORT] or client [SERVER_PORT] [USER_NAME]");
    let args: Vec<String> = std::env::args().collect();

    let height = 900.0;
    App::new()
        .insert_resource(WindowDescriptor {
            width: height * RESOLUTION,
            height: height,
            title: "My game".to_string(),
            present_mode: bevy::window::PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(LogSettings {
            level: Level::TRACE,
            filter: "info,wgpu_core=warn,wgpu_hal=warn,base_defense::projectile=debug".to_string(),
        })
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Main)
        .add_plugin(NetworkingPlugin::new(&args))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_cursor_marker)
        .add_plugin(MapPlugin)
        .add_plugin(TowerPlugin)
        .add_plugin(UserInterfacePlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ProjectilePlugin)
        .add_plugin(HPBarsPlugin)
        .add_system_set(SystemSet::on_update(AppState::Building).with_system(cursor_position))
        .add_system(bevy::window::close_on_esc)
        .run();
}
#[derive(Component)]
struct MainCamera;
fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.top = 1.0;
    camera.projection.bottom = -1.0;
    camera.projection.right = 1.0 * RESOLUTION;
    camera.projection.left = -1.0 * RESOLUTION;
    camera.projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera).insert(MainCamera);
}

#[derive(Component)]
pub struct CursorMarker;

fn spawn_cursor_marker(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.2, 0.8),
                custom_size: Some(Vec2::splat(TILE_SIZE * 0.6)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 2.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CursorMarker);
}

fn cursor_position(
    mut commands: Commands,
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_marker: Query<&mut Transform, With<CursorMarker>>,
    buttons: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    mut selection: ResMut<Option<Icons>>,
    mut app_state: ResMut<State<AppState>>,
    panel_width: Res<RightPanelWidth>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        if screen_pos.x > wnd.width() - panel_width.0 {
            return;
        }

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        let tile_x = ((world_pos.x + TILE_SIZE / 2.0) / TILE_SIZE).floor();
        let tile_y = ((world_pos.y + TILE_SIZE / 2.0) / TILE_SIZE).floor();

        let mut marker = q_marker.single_mut();
        marker.translation.x = tile_x * TILE_SIZE;
        marker.translation.y = tile_y * TILE_SIZE;
        if buttons.just_pressed(MouseButton::Left) {
            if let Some(x) = *selection {
                match x {
                    Icons::Enemy => Enemy::new(commands, world_pos, asset_server),
                    Icons::Tower => Tower::create_tower(commands, marker.translation, asset_server),
                }
            }
        } else if buttons.just_pressed(MouseButton::Right) {
            *selection = None;
            app_state.set(AppState::Main);
        }
    }
}

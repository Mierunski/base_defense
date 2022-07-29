use bevy::{
    input::mouse::MouseMotion,
    log::{Level, LogSettings},
    prelude::*,
    render::camera::{RenderTarget, ScalingMode},
};
use debug::DebugPlugin;
use map::MapPlugin;
use tower::TowerPlugin;
use user_interface::UserInterfacePlugin;

use crate::tower::Tower;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.15;

mod debug;
mod map;
mod tower;
mod user_interface;

fn main() {
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
            filter: "info,wgpu_core=warn,wgpu_hal=warn,base_defense::tower=info".to_string(),
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_cursor_marker)
        .add_plugin(MapPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(TowerPlugin)
        .add_plugin(UserInterfacePlugin)
        .add_system(cursor_position)
        .run();
}
#[derive(Component)]
struct MainCamera;
fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;
    camera.orthographic_projection.scaling_mode = ScalingMode::None;

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
                translation: Vec3::new(0.0, 0.0, 1.0),
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

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        let tile_x = ((world_pos.x + TILE_SIZE / 2.0) / TILE_SIZE).floor();
        let tile_y = ((world_pos.y + TILE_SIZE / 2.0) / TILE_SIZE).floor();

        let mut marker = q_marker.single_mut();
        marker.translation.x = tile_x * TILE_SIZE;
        marker.translation.y = tile_y * TILE_SIZE;
        if buttons.just_pressed(MouseButton::Left) {
            Tower::create_tower(commands, marker.translation, asset_server);
        }
    }
}

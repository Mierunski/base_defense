use std::collections::HashMap;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    input::mouse::MouseWheel,
    prelude::*,
    render::camera::{RenderTarget, ScalingMode},
};
use bevy_egui::{
    egui::{self, ImageButton},
    EguiContext, EguiPlugin,
};

use crate::{constants::*, enemy::Enemy, tower::Tower, AppState};
#[derive(Component)]
struct FpsText;
pub struct UserInterfacePlugin;

#[derive(Default)]
struct UiState {
    _label: String,
    _value: f32,
    _inverted: bool,
    _enemy: egui::TextureId,
    icons: HashMap<Icons, Icon>,
}
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Icons {
    Enemy,
    Tower,
}
struct Icon {
    _handle: Handle<Image>,
    texture_id: egui::TextureId,
    clicked: bool,
}

impl Icon {
    fn new(
        path: &str,
        asset_server: &Res<AssetServer>,
        egui_context: &mut ResMut<EguiContext>,
    ) -> Icon {
        let _handle = asset_server.load(path);
        let texture_id = egui_context.add_image(_handle.clone_weak());

        Icon {
            _handle,
            texture_id,
            clicked: false,
        }
    }
}
#[derive(Default)]
pub struct RightPanelWidth(pub f32);

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .init_resource::<UiState>()
            .init_resource::<Option<Icons>>()
            .init_resource::<RightPanelWidth>()
            .add_startup_system(ui_setup)
            .add_startup_system(spawn_camera)
            .add_startup_system(spawn_cursor_marker)
            // .add_system(ui_update)
            .add_system(camera_follow)
            .add_plugin(EguiPlugin)
            // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
            // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
            .add_system(ui_example)
            .add_system_set(SystemSet::on_update(AppState::Building).with_system(cursor_position));
    }
}
fn ui_example(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    diagnostics: Res<Diagnostics>,
    mut app_state: ResMut<State<AppState>>,
    mut selection: ResMut<Option<Icons>>,
    mut panel_width: ResMut<RightPanelWidth>,
) {
    panel_width.0 = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            // Shorter version:
            for (key, icon) in ui_state.icons.iter_mut() {
                if let Some(x) = *selection {
                    if x == *key {
                        icon.clicked = true;
                    } else {
                        icon.clicked = false;
                    }
                } else {
                    icon.clicked = false;
                }
                if ui
                    .add(ImageButton::new(icon.texture_id, [50.0, 50.0]).selected(icon.clicked))
                    .clicked()
                {
                    icon.clicked = !icon.clicked;
                    if icon.clicked {
                        *selection = Some(*key);

                        _ = app_state.set(AppState::Building);
                    } else {
                        *selection = None;
                        _ = app_state.set(AppState::Main);
                    }
                }
            }

            ui.label("Hello world!");
            if ui.button("Click me").clicked() {
                // take some action here
            };

            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(average) = fps.average() {
                    // Update the value of the second section
                    ui.label(format!("FPS: {:.2}", average));
                }
            }

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}
pub const CAMERA_SPEED: f32 = 2.0;

fn camera_follow(
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
    keyboard: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
    time: Res<Time>,
) {
    let (mut camera_transform, mut projection) = camera_query.single_mut();

    let speed = CAMERA_SPEED * time.delta_seconds() * projection.scale;
    if keyboard.pressed(KeyCode::A) {
        camera_transform.translation.x -= speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera_transform.translation.x += speed;
    }
    if keyboard.pressed(KeyCode::W) {
        camera_transform.translation.y += speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera_transform.translation.y -= speed;
    }
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                projection.scale += ev.y * CAMERA_SPEED * 0.1;
                projection.scale = projection.scale.clamp(0.4, 5.0);
            }
            MouseScrollUnit::Pixel => {
                projection.scale += ev.y * CAMERA_SPEED * 0.1;
                projection.scale = projection.scale.clamp(0.4, 5.0);
            }
        }
    }
}

fn ui_setup(
    mut ui_state: ResMut<UiState>,
    asset_server: Res<AssetServer>,
    mut egui_context: ResMut<EguiContext>,
) {
    ui_state.icons.insert(
        Icons::Enemy,
        Icon::new("sprites/enemy.png", &asset_server, &mut egui_context),
    );
    ui_state.icons.insert(
        Icons::Tower,
        Icon::new("sprites/tower.png", &asset_server, &mut egui_context),
    );
}

fn cursor_position(
    commands: Commands,
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
            _ = app_state.set(AppState::Main);
        }
    }
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

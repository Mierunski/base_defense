use std::{collections::HashMap, fmt::format, iter::Map};

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    input::mouse::MouseWheel,
    prelude::*,
    text, ui,
};
use bevy_egui::{
    egui::{self, ImageButton},
    EguiContext, EguiPlugin,
};

use crate::{AppState, MainCamera, TILE_SIZE};
#[derive(Component)]
struct FpsText;
pub struct UserInterfacePlugin;

#[derive(Default)]
struct UiState {
    label: String,
    value: f32,
    inverted: bool,
    enemy: egui::TextureId,
    icons: HashMap<Icons, Icon>,
}
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Icons {
    Enemy,
    Tower,
}
struct Icon {
    handle: Handle<Image>,
    texture_id: egui::TextureId,
    clicked: bool,
}

impl Icon {
    fn new(
        path: &str,
        asset_server: &Res<AssetServer>,
        mut egui_context: &mut ResMut<EguiContext>,
    ) -> Icon {
        let handle = asset_server.load(path);
        let texture_id = egui_context.add_image(handle.clone_weak());

        Icon {
            handle,
            texture_id,
            clicked: false,
        }
    }
}

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .init_resource::<UiState>()
            .init_resource::<Option<Icons>>()
            .add_startup_system(ui_setup)
            // .add_system(ui_update)
            .add_system(camera_follow)
            .add_plugin(EguiPlugin)
            // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
            // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
            .add_system(ui_example);
    }
}
fn ui_example(
    mut egui_context: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    diagnostics: Res<Diagnostics>,
    mut app_state: ResMut<State<AppState>>,
    mut selection: ResMut<Option<Icons>>,
) {
    egui::SidePanel::right("right_panel")
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

                        app_state.set(AppState::Building);
                    } else {
                        *selection = None;
                        app_state.set(AppState::Main);
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
    mut commands: Commands,
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

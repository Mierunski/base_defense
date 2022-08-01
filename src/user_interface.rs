use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    input::mouse::MouseWheel,
    prelude::*,
};
use bevy_egui::{
    egui::{self, ImageButton},
    EguiContext, EguiPlugin,
};

use crate::{MainCamera, TILE_SIZE};
#[derive(Component)]
struct FpsText;
pub struct UserInterfacePlugin;

#[derive(Default)]
struct UiState {
    label: String,
    value: f32,
    inverted: bool,
    egui_texture_handle: Option<egui::TextureHandle>,

    bevy_icon: Handle<Image>,
    clicked: bool,
}

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .init_resource::<UiState>()
            .add_startup_system(ui_setup)
            .add_system(ui_update)
            .add_system(camera_follow)
            .add_plugin(EguiPlugin)
            // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
            // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
            .add_system(ui_example);
    }
}
fn ui_example(mut egui_context: ResMut<EguiContext>, mut ui_state: ResMut<UiState>) {
    // egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
    //     ui.label("world");
    // });

    let egui_texture_handle = egui_context.add_image(ui_state.bevy_icon.clone_weak());
    // ui_state
    //     .egui_texture_handle
    //     .get_or_insert_with(|| {
    //         egui_context
    //             .ctx_mut()
    //             .load_texture("Enemy", egui::ColorImage::example())
    //     })
    //     .clone();

    egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            // Shorter version:
            ui.image(egui_texture_handle, [50.0, 50.0]);
            if ui
                .add(ImageButton::new(egui_texture_handle, [50.0, 50.0]).selected(ui_state.clicked))
                .clicked()
            {
                ui_state.clicked = !ui_state.clicked;
            }
            ui.label("Hello world!");
            if ui.button("Click me").clicked() {
                // take some action here
            };
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

fn ui_setup(mut commands: Commands, mut ui_state: ResMut<UiState>, asset_server: Res<AssetServer>) {
    // commands.spawn_bundle(UiCameraBundle::default());
    // Rich text with multiple sections
    ui_state.bevy_icon = asset_server.load("sprites/enemy.png");

    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 60.0,
                    color: Color::GOLD,
                }),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(FpsText);
}

fn ui_update(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    }
}

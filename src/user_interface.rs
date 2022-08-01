use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    input::mouse::MouseWheel,
    prelude::*,
};

use crate::{MainCamera, TILE_SIZE};
#[derive(Component)]
struct FpsText;
pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(ui_setup)
            .add_system(ui_update)
            .add_system(camera_follow);
    }
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

fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn_bundle(UiCameraBundle::default());
    // Rich text with multiple sections
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

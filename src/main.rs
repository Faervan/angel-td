use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*, window,
};

mod game;
use game::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Angel TD".into(),
                        name: Some("angel-td".into()),
                        resolution: bevy::window::WindowResolution::with_scale_factor_override((1920.0, 1080.0).into(), 1.0),
                        mode: bevy::window::WindowMode::BorderlessFullscreen,
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        enabled_buttons: bevy::window::EnabledButtons { minimize: false, maximize: false, close: false },
                        ..default()
                    }),
                    ..default()
                }
            ).set(ImagePlugin::default_nearest()),
            // Adds frame time diagnostics
            FrameTimeDiagnosticsPlugin,
            // Adds a system that prints diagnostics to the console
            LogDiagnosticsPlugin::default(),
            GamePlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            window::close_on_esc,
        ))
        .run();
}

pub fn setup (
    mut commands: Commands
) {
    //Spawn Camera
    commands.spawn(Camera2dBundle::default());
}
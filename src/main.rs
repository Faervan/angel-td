use std::collections::HashMap;
use std::collections::VecDeque;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*, window,
};
use enemy_types::EnemyType;
use enemy_wave_map::{WaveMap, WaveRange, Waves};
use ui::UiPlugin;

mod components;
mod bullet;
mod tower;
mod enemy;
mod enemy_types;
mod tower_types;
mod bullet_types;
mod enemy_wave_map;
mod ui;
use {
    components::EnemyPath,
    enemy::*,
    tower::*,
    bullet::*,
};

#[derive(Resource)]
pub struct Gold(usize);

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
            UiPlugin,
        ))
        .add_systems(Startup, (
            setup,
            spawn_tower,
        ))
        .add_systems(Update, (
            window::close_on_esc,
            spawn_enemies,
            tower_get_target,
            tower_lost_target,
            tower_rotate_at_target,
            tower_charge,
            spawn_bullet,
            move_bullet,
            bullet_hits_enemy,
            tower_animate_charging,
            tower_animate_shooting,
        ))
        .add_systems(Update, (
            enemy_movement,
            enemy_at_destination,
        ).chain())
        .insert_resource(
            Waves {
                current: 0,
                wave_margin: Timer::from_seconds(5., TimerMode::Once),
                spawn_delay: Timer::from_seconds(0.4, TimerMode::Repeating),
                queue: VecDeque::new(),
            })
        .insert_resource(
            WaveMap {
                waves: 10,
                wave_range: HashMap::from([
                    (EnemyType::Militia, WaveRange{
                        lowest_level: 0,
                        lowest_probability: 3,
                        highest_level: 9,
                        highest_probability: 10,
                    })
                ]),
            })
        .insert_resource(Gold(0))
        .run();
}

pub fn setup (
    mut commands: Commands,
    asset_server: ResMut<AssetServer>
) {
    //Spawn Camera
    commands.spawn(Camera2dBundle::default());
    //Spawn map
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/maps/demo_map.png"),
            transform: Transform {
                translation: Vec3 { x: 0.0, y: 0.0, z: -1.0},
                ..default()
            },
            ..default()
        },
        EnemyPath {
            path_points: vec![
                Vec2::new(-960., -240.),
                Vec2::new(413., -210.),
                Vec2::new(407., -67.),
                Vec2::new(153., -74.),
                Vec2::new(153., 63.),
                Vec2::new(960., 65.)
            ]
        },
    ));
}
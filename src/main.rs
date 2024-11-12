use std::collections::HashMap;
use std::collections::VecDeque;
#[cfg(target_arch = "wasm32")]
use std::sync::Mutex;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use bevy::{
    diagnostic::LogDiagnosticsPlugin,
    prelude::*,
};
use enemy_types::EnemyType;
use enemy_wave_map::{WaveMap, WaveRange, Waves};
use ui::UiPlugin;
use ui::UiState;

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

pub const SCREENWIDTH: f32 = 1920.;
pub const SCREENHEIGTH: f32 = 1080.;

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
                        canvas: Some("#angel-td".to_string()),
                        ..default()
                    }),
                    ..default()
                }
            ).set(ImagePlugin::default_nearest()),
            // Adds a system that prints diagnostics to the console
            LogDiagnosticsPlugin::default(),
            UiPlugin,
        ))
        .init_state::<AppState>()
        .add_systems(Startup, init)
        .add_systems(OnEnter(AppState::InGame), setup)
        .add_systems(OnExit(AppState::InGame), cleanup)
        .add_systems(Update, (
            close_on_esc,
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
            spawn_tower.run_if(in_state(UiState::TowerPlacing(true))),
        ).run_if(in_state(AppState::InGame)))
        .add_systems(Update, (
            enemy_movement,
            enemy_at_destination,
        ).chain().run_if(in_state(AppState::InGame)))
        .add_systems(Update, toggle_app_state)
        .run();
}

fn init(mut next_state: ResMut<NextState<AppState>>) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        next_state.set(AppState::InGame);
    }
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
    commands.insert_resource(
        Waves {
            current: 0,
            wave_margin: Timer::from_seconds(5., TimerMode::Once),
            spawn_delay: Timer::from_seconds(0.4, TimerMode::Repeating),
            queue: VecDeque::new(),
        }
    );
    commands.insert_resource(
        WaveMap {
            waves: 9,
            wave_range: HashMap::from([
                (EnemyType::Militia, WaveRange{
                    lowest_level: 0,
                    lowest_probability: 3,
                    highest_level: 9,
                    highest_probability: 15,
                }),
                (EnemyType::HolyKnight, WaveRange{
                    lowest_level: 2,
                    lowest_probability: 1,
                    highest_level: 9,
                    highest_probability: 5,
                })
            ]),
    });
    commands.insert_resource(Gold(250));
}

fn cleanup(
    mut commands: Commands,
    entities: Query<Entity>,
) {
    println!("cleanup");
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(States, Hash, PartialEq, Eq, Clone, Debug, Default)]
enum AppState {
    InGame,
    #[default]
    Idle
}

fn close_on_esc(
    input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    #[cfg(not(target_arch = "wasm32"))]
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

#[allow(unused_variables, unused_mut)]
fn toggle_app_state(
    mut next_state: ResMut<NextState<AppState>>,
    app_state: Res<State<AppState>>,
) {
    #[cfg(target_arch = "wasm32")]
    {
        let state = unsafe {
            TOGGLE_STATE.get_mut().unwrap()
        };
        if *state {
            log("toggle_app_state is being executed now!");
            next_state.set(match app_state.get() {
                AppState::InGame => AppState::Idle,
                AppState::Idle => AppState::InGame
            });
            *state = false;
        }
    }
}


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(target_arch = "wasm32")]
static mut TOGGLE_STATE: Mutex<bool> = Mutex::new(false);

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub unsafe fn toggle_state() {
    *TOGGLE_STATE.get_mut().unwrap() = true;
}

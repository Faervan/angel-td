use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*, sprite::MaterialMesh2dBundle,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
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
            }),
            // Adds frame time diagnostics
            FrameTimeDiagnosticsPlugin,
            // Adds a system that prints diagnostics to the console
            LogDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, enemy_movement)
        .run();
}

#[derive(Component, Debug)]
pub struct Enemy {
    pub velocity: f32,
    pub path_state: usize
}

#[derive(Component)]
pub struct Tower {
    pub range: u32
}


//For testing purpose
#[derive(Component)]
pub struct TowerRadiusIndicator {
    pub range: u32
}

#[derive(Component)]
pub struct TowerBullet {}

#[derive(Component)]
pub struct EnemyPath {
    pub path_points: Vec<Vec2>
}

pub fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());
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
        }
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/enemies/ball_red_large.png"),
            transform: Transform {
                translation: Vec3 { x: -960.0, y: -240.0, z: 0.0 },
                ..default()
            },
            ..default()
        },
        Enemy {
            velocity: 300.0,
            path_state: 0
        }
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(Color::rgb(0., 0., 0.)),
            transform: Transform::from_translation(Vec3::new(42., -113., 0.)).with_scale(Vec3::new(100., 100., 0.)),
            ..default()
        },
        Tower {
            range: 300
        }
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(Color::rgba(0., 0., 0., 0.5)),
            transform: Transform::from_translation(Vec3::new(42., -113., -0.1)).with_scale(Vec3::new(300., 300., 0.)),
            ..default()
        },
        TowerRadiusIndicator {
            range: 300
        }
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(Color::rgb(1., 0., 0.)),
            transform: Transform::from_translation(Vec3::new(42., -113., 1.)).with_scale(Vec3::new(10., 10., 0.)),
            ..default()
        },
        TowerBullet {}
    ));
}

pub fn enemy_movement (
    mut enemy_query: Query<(Entity, &mut Transform, &mut Enemy), With<Enemy>>,
    path_query: Query<&EnemyPath, With<EnemyPath>>,
    time: Res<Time>,
    mut commands: Commands
) {
    if let Ok(path) = path_query.get_single() {
        for (entity, mut transform, mut enemy) in enemy_query.iter_mut() {
            let path_state_directions: Vec<Vec3> = (0..path.path_points.len()-1).map(|i| {
                let x = path.path_points[i+1].x - path.path_points[i].x;
                let y = path.path_points[i+1].y - path.path_points[i].y;
                let distance = path.path_points[i].distance(path.path_points[i+1]);
                Vec3::new(x / distance, y / distance, 0.)
            }).collect();
            transform.translation += path_state_directions[enemy.path_state] * enemy.velocity * time.delta_seconds();
            if
                //Distance of current enemy location from last path_point
                path.path_points[enemy.path_state].distance(Vec2::new(transform.translation.x, transform.translation.y))
                >=
                //Distance of next path_point from last path_point
                path.path_points[enemy.path_state].distance(path.path_points[enemy.path_state+1])
                {
                //Enemy reached next path_point, so it will change directions now
                enemy.path_state += 1;
                transform.translation = path.path_points[enemy.path_state].extend(0.);
                if enemy.path_state == path.path_points.len()-1 {
                    enemy_reaches_destination(entity, &mut commands);
                }
            }
        }
    }
}

fn enemy_reaches_destination (
    entity: Entity,
    commands: &mut Commands
) {
    //Add tower damage logic here
    commands.entity(entity).despawn();
    println!("Enemy despawned.");
}
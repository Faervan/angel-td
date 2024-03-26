use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*, sprite::MaterialMesh2dBundle, window,
};

const ENEMY_SIZE: f32 = 64.;

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
        .add_systems(Update, window::close_on_esc)
        .add_systems(Update, (
            enemy_movement,
            tower_check_for_enemies_in_range,
            move_bullet,
            bullet_hits_enemy
        ).chain())
        .run();
}

#[derive(Component, Debug)]
pub struct Enemy {
    pub velocity: f32,
    pub path_state: usize,
    pub direction: Vec3
}

#[derive(Component)]
pub struct Tower {
    pub range: f32
}


//For testing purpose
#[derive(Component)]
pub struct TowerRadiusIndicator {
    pub range: f32
}

#[derive(Component)]
pub struct TowerBullet {
    pub velocity: f32,
    pub direction: Vec3
}

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
        }
    ));
    //Spawn "enemy"
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
            velocity: 300.,
            path_state: 0,
            direction: Vec3::new(0., 0., 0.)
        }
    ));
    //Spawn "tower"
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(Color::rgb(0., 0., 0.)),
            transform: Transform::from_translation(Vec3::new(42., -113., 0.)).with_scale(Vec3::new(100., 100., 0.)),
            ..default()
        },
        Tower {
            range: 150.
        }
    ))
    .with_children(|parent| {
        parent.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::default()).into(),
                material: materials.add(Color::rgba(0., 0., 0., 0.5)),
                transform: Transform::from_translation(Vec3::new(0., 0., -0.1)).with_scale(Vec3::new(3., 3., 0.)),
                ..default()
            },
            TowerRadiusIndicator {
                range: 300.
            }
        ));
    });
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(Color::rgb(1., 0., 0.)),
            transform: Transform::from_translation(Vec3::new(42., -113., 1.)).with_scale(Vec3::new(10., 10., 0.)),
            ..default()
        },
        TowerBullet {
            velocity: 150.,
            direction: Vec3::new(0., 0., 0.)
        }
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
            enemy.direction = path_state_directions[enemy.path_state];
            transform.translation += enemy.direction * enemy.velocity * time.delta_seconds();
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

pub fn tower_check_for_enemies_in_range (
    tower_query: Query<(&Transform, &Tower), With<Tower>>,
    enemy_query: Query<(&Transform, &Enemy), With<Enemy>>,
    mut bullet_query: Query<(&Transform, &mut TowerBullet), With<TowerBullet>>
) {
    for (tower_pos, tower) in tower_query.iter() {
        for (enemy_pos, enemy) in enemy_query.iter() {
            if enemy_pos.translation.distance(tower_pos.translation) - ENEMY_SIZE / 2. <= tower.range {
                if let Ok((bullet_pos, mut bullet)) = bullet_query.get_single_mut() { if bullet.direction == Vec3::new(0., 0., 0.) {
                    bullet.direction = {
                        //let x = enemy_pos.translation.x - bullet_pos.translation.x;
                        //let y = enemy_pos.translation.y - bullet_pos.translation.y;
                        //let distance = bullet_pos.translation.distance(enemy_pos.translation);
                        let r1: f32 = 2. * bullet_pos.translation.x.powi(2) - 2. * enemy_pos.translation.x * bullet_pos.translation.x + (enemy_pos.translation.y - bullet_pos.translation.y).powi(2);
                        let r2: f32 = 2. * (enemy_pos.translation.x * enemy.direction.x - enemy.direction.x * bullet_pos.translation.x + enemy.direction.y * (enemy_pos.translation.y - bullet_pos.translation.y));
                        let r = - r1 / r2;
                        let gr = Vec3::new(
                            enemy_pos.translation.x + enemy.direction.x * r,
                            enemy_pos.translation.y + enemy.direction.y * r,
                            0.
                        );
                        let x = gr.x - bullet_pos.translation.x;
                        let y = gr.y - bullet_pos.translation.y;
                        let distance = bullet_pos.translation.distance(gr);
                        Vec3::new(x / distance, y / distance, 0.)
                    };
                    println!("bullet fires in direction: {}",bullet.direction);
                }}
            }
        }
    }
}

pub fn move_bullet (
    mut bullet_query: Query<(&TowerBullet, &mut Transform), With<TowerBullet>>,
    time: Res<Time>
) {
    if let Ok((bullet, mut bullet_pos)) = bullet_query.get_single_mut() {
        bullet_pos.translation += bullet.direction * bullet.velocity * time.delta_seconds();
    }
}

pub fn bullet_hits_enemy (
    bullet_query: Query<(Entity, &Transform), With<TowerBullet>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut commands: Commands
) {
    for (bullet, bullet_pos) in bullet_query.iter() {
        for enemy_pos in enemy_query.iter() {
            if bullet_pos.translation.distance(enemy_pos.translation) - ENEMY_SIZE / 2. <= 0. {
                commands.entity(bullet).despawn();
            }
        }
    }
}
use bevy::prelude::*;

use crate::enemy_types::EnemyType;

use super::components::{
    EnemyPath,
    Enemy
};

pub fn spawn_enemies (
    mut commands: Commands,
    asset_server: ResMut<AssetServer>
) {
    let enemy_type: &EnemyType = &EnemyType::Militia;
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(enemy_type.sprite()),
            transform: Transform {
                translation: Vec3::new(-960., -240., 0.),
                ..default()
            },
            ..default()
        },
        Enemy {
            enemy_type: *enemy_type,
            path_state: 0
        }
    ));
}

pub fn enemy_movement (
    mut enemy_query: Query<(&mut Transform, &mut Enemy), With<Enemy>>,
    path_query: Query<&EnemyPath, With<EnemyPath>>,
    time: Res<Time>,
) {
    if let Ok(path) = path_query.get_single() {
        for (mut transform, mut enemy) in enemy_query.iter_mut() {
            let path_state_directions: Vec<Vec3> = (0..path.path_points.len()-1).map(|i| {
                let x = path.path_points[i+1].x - path.path_points[i].x;
                let y = path.path_points[i+1].y - path.path_points[i].y;
                let distance = path.path_points[i].distance(path.path_points[i+1]);
                Vec3::new(x / distance, y / distance, 0.)
            }).collect();
            transform.translation += path_state_directions[enemy.path_state] * time.delta_seconds() * enemy.enemy_type.velocity();
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
            }
        }
    }
}

pub fn enemy_at_destination (
    enemy_query: Query<(Entity, &Enemy), With<Enemy>>,
    path_query: Query<&EnemyPath, With<EnemyPath>>,
    mut commands: Commands
) {
    for (entity, enemy) in enemy_query.iter() {
        if let Ok(path) = path_query.get_single() {
            if enemy.path_state == path.path_points.len()-1 {
                commands.entity(entity).despawn();
                println!("Enemy despawned");
            }
        }
    }
}
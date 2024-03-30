use bevy::prelude::*;
use super::{
    components::*,
    super::maps::components::EnemyPath
};

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>
) {
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
            path_state: 0
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
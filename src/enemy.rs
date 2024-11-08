use bevy::prelude::*;


use crate::{enemy_types::EnemyType, enemy_wave_map::{WaveMap, Waves}};

use super::components::{
    EnemyPath,
    Enemy
};

pub fn spawn_enemies(
    mut waves: ResMut<Waves>,
    wavemap: Res<WaveMap>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    time: Res<Time>
) {
    if waves.queue.len() > 0 {
        waves.spawn_delay.tick(time.delta());
        if waves.spawn_delay.just_finished() {
            let enemy_type: EnemyType = waves.queue.pop_front().unwrap();
            let health = enemy_type.health();
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
                    enemy_type,
                    path_state: 0,
                    real_health: health,
                    calc_health: health,
                }
            ));
        }
    } else if waves.queue.len() == 0 {
        if waves.wave_margin.paused() {
            waves.wave_margin.unpause();
        }
        waves.wave_margin.tick(time.delta());
        if waves.wave_margin.finished() {
            for (enemy_type, range) in wavemap.wave_range.iter() {
                if range.lowest_level <= waves.current && range.highest_level >= waves.current {
                    let diff_per_wave = (range.highest_probability as f32 - range.lowest_probability as f32)/(range.highest_level as f32 - range.lowest_level as f32);
                    let this_wave = range.lowest_probability as f32 + diff_per_wave * waves.current as f32 - range.lowest_level as f32;
                    for _ in 0..this_wave.round() as usize {
                        waves.queue.push_back(*enemy_type);
                    }
                    println!("{:?}", waves.queue);
                }
            }
            waves.current += 1;
            println!("new wave started! wave: {}", waves.current);
            waves.wave_margin.reset();
            waves.wave_margin.pause();
        }
    }
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
            }
        }
    }
}

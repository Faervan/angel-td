use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use crate::Gold;

use super::components::*;

pub fn spawn_bullet(
    tower_query: Query<(Entity, &Transform, &Tower, &Target), (With<IsCharged>, Without<Enemy>)>,
    mut enemy_query: Query<&mut Enemy, With<Enemy>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (tower_entity, tower_pos, tower, target) in tower_query.iter() {
        if let Ok(mut enemy) = enemy_query.get_mut(target.0) {
            let damage = tower.tower_type.damage();
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::default()).into(),
                    material: materials.add(Color::srgb(1., 0., 0.)),
                    transform: Transform::from_translation(tower_pos.translation).with_scale(Vec3::new(10., 10., 1.)),
                    ..default()
                },
                Bullet {
                    origin: tower_entity,
                    target: target.0,
                    bullet_type: tower.tower_type.bullet_type(),
                    damage,
                },
            ));
            //Reducing the targets calc_health, so that the Tower does not shoot at a target that will die by bullets already shot
            if enemy.calc_health >= damage {
                enemy.calc_health -= damage;
            } else {
                enemy.calc_health = 0;
            }
            commands.entity(tower_entity).remove::<IsCharged>().insert(IsShooting);
        }
    }
}

pub fn move_bullet(
    mut bullet_query: Query<(Entity, &mut Transform, &Bullet), Without<Enemy>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (bullet_entity, mut bullet_pos, bullet) in bullet_query.iter_mut() {
        if let Ok(target) = enemy_query.get(bullet.target) {
            let direction = {
                let x = target.translation.x - bullet_pos.translation.x;
                let y = target.translation.y - bullet_pos.translation.y;
                let distance = bullet_pos.translation.distance(target.translation);
                Vec3::new(x / distance, y / distance, 0.)
            };
            bullet_pos.translation += direction * bullet.bullet_type.velocity() * time.delta_seconds();
        } else {
            commands.entity(bullet_entity).despawn();
        }
    }
}

pub fn bullet_hits_enemy(
    bullet_query: Query<(Entity, &Transform, &Bullet), Without<Enemy>>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy), With<Enemy>>,
    mut commands: Commands,
    mut gold: ResMut<Gold>,
) {
    for (bullet_entity, bullet_pos, bullet) in bullet_query.iter() {
        if let Ok((enemy_entity, enemy_pos, mut enemy)) = enemy_query.get_mut(bullet.target) {
            if enemy_pos.translation.distance(bullet_pos.translation) <= enemy.enemy_type.hit_circle() {
                commands.entity(bullet_entity).despawn();
                if enemy.real_health >= bullet.damage {
                    enemy.real_health -= bullet.damage;
                } else {
                    enemy.real_health = 0;
                }
                if enemy.real_health <= 0 {
                    commands.entity(enemy_entity).despawn();
                    gold.0 += enemy.enemy_type.bounty();
                    println!("Gold: {}", gold.0);
                }
            }
        }
    }
}

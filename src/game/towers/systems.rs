use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use super::{
    components::*,
    super::enemies::components::Enemy
};

const ENEMY_SIZE: f32 = 64.;

pub fn spawn_tower(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    //Spawn "tower"
    let texture = asset_server.load("sprites/turrets/ballista_bow_sheet.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(100.,100.), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 3, forward: true };
    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_translation(Vec3::new(42., -113., 0.)).with_scale(Vec3::new(1.3, 1.3, 0.)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Tower {
            range: 150.
        },
        Rotatable {
            speed: 0.3
        }
    ))
    .with_children(|parent| {
        parent.spawn(
            SpriteBundle {
                texture: asset_server.load("sprites/turrets/ballista_body.png"),
                transform: Transform::from_translation(Vec3::new(0., 0., -0.1)),
                ..default()
            }
        );
        parent.spawn(
            SpriteBundle {
                texture: asset_server.load("sprites/turrets/ballista_stand.png"),
                transform: Transform::from_translation(Vec3::new(0., 0., -0.2)),
                ..default()
            }
        );
        parent.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::default()).into(),
                material: materials.add(Color::rgba(0., 0., 0., 0.5)),
                transform: Transform::from_translation(Vec3::new(0., 0., -0.5)).with_scale(Vec3::new(300./1.3, 300./1.3, 0.)),
                ..default()
            },
            TowerRadiusIndicator {
                range: 300.
            }
        ));
    });
}

pub fn tower_check_for_enemies_in_range (
    tower_query: Query<(&Transform, &Tower), With<Tower>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut bullet_query: Query<(&Transform, &mut TowerBullet), With<TowerBullet>>
) {
    for (tower_pos, tower) in tower_query.iter() {
        for enemy_pos in enemy_query.iter() {
            if enemy_pos.translation.distance(tower_pos.translation) - ENEMY_SIZE / 2. <= tower.range {
                if let Ok((bullet_pos, mut bullet)) = bullet_query.get_single_mut() {
                    bullet.direction = {
                        let x = enemy_pos.translation.x - bullet_pos.translation.x;
                        let y = enemy_pos.translation.y - bullet_pos.translation.y;
                        let distance = bullet_pos.translation.distance(enemy_pos.translation);
                        Vec3::new(x / distance, y / distance, 0.)
                    };
                }
            }
        }
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (mut indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.forward = false;
                atlas.index - 1
            } else if atlas.index == indices.first {
                indices.forward = true;
                atlas.index + 1
            } else if indices.forward {
                atlas.index + 1
            } else {
                atlas.index -1
            };
        }
    }
}

pub fn rotate_ballista (
    //time: Res<Time>,
    mut ballistas: Query<(&mut Transform, &Rotatable), Without<Enemy>>,
    enemy: Query<&Transform, With<Enemy>>
) {
    if let Ok(enemy) = enemy.get_single() {
        for (mut ballista, _rotate) in &mut ballistas {
            //Getting the angle between default tower rotation and enemy
            let angle_to_enemy = (enemy.translation.truncate() - ballista.translation.truncate()).angle_between(Vec2::new(0.,1.));
            //Calculating the rotation of the tower, so that it "looks" into the direction of the enemy
            //TAU is The full circle constant (τ) and equal to 2π.
            ballista.rotation = Quat::from_rotation_z(-angle_to_enemy-std::f32::consts::TAU);
            //ballista.rotate_z(rotate.speed * std::f32::consts::TAU * time.delta_seconds());
        }
    }
}

pub fn spawn_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(Color::rgb(1., 0., 0.)),
            transform: Transform::from_translation(Vec3::new(42., -113., 1.)).with_scale(Vec3::new(10., 10., 0.)),
            ..default()
        },
        TowerBullet {
            velocity: 600.,
            direction: Vec3::new(0., 0., 0.)
        }
    ));
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
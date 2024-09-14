use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use crate::{ui::UiState, Gold, SCREENHEIGTH, SCREENWIDTH};

use super::{
    tower_types::*,
    components::*,
};

pub fn spawn_tower(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut next_state: ResMut<NextState<UiState>>,
    mut gold: ResMut<Gold>,
    tower_placing_state: Res<State<UiState>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) && *tower_placing_state.get() == UiState::TowerPlacing(true) {

        //Spawn "tower"
        let tower_type = &TowerType::XBow;
        let cursor_pos = window.get_single().unwrap().cursor_position().expect("getting cursor position failed");
        let tower_position = Vec3::new(cursor_pos.x - SCREENWIDTH / 2., (cursor_pos.y - SCREENHEIGTH / 2.) * -1., 0.);
        let texture = asset_server.load(tower_type.sprite());
        let tower_scale = Vec3::new(tower_type.scale(), tower_type.scale(), 0.);
        let tower_radius = Vec3::new(tower_type.range() * 2. / tower_type.scale(), tower_type.range() * 2. / tower_type.scale(), 0.);
        if let Some((width, height, grid_columns)) = tower_type.has_animation() {
            let layout = TextureAtlasLayout::from_grid(Vec2::new(width, height), usize::from(grid_columns), 1, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);
            // Use only the subset of sprites in the sheet that make up the run animation
            let animation_indices = AnimationIndices { first: 0, last: usize::from(grid_columns-1)};
            let tower = commands.spawn((
                SpriteSheetBundle {
                    texture,
                    atlas: TextureAtlas {
                        layout: texture_atlas_layout,
                        index: usize::from(animation_indices.first),
                    },
                    transform: Transform::from_translation(tower_position).with_scale(tower_scale),
                    ..default()
                },
                animation_indices,
                AnimationTimer(Timer::from_seconds(tower_type.cooldown().duration().as_secs_f32()/f32::from(grid_columns*2), TimerMode::Repeating)),
                Tower {
                    tower_type: *tower_type,
                    cooldown: tower_type.cooldown(),
                },
                IsCharged,
            ))
            .with_children(|parent| {
                parent.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Circle::default()).into(),
                        material: materials.add(Color::rgba(0., 0., 0., 0.5)),
                        transform: Transform::from_translation(Vec3::new(0., 0., -0.5)).with_scale(tower_radius),
                        ..default()
                    },
                    TowerRadiusIndicator,
                ));
            }).id();
            if tower_type.has_rotation() {
                commands.entity(tower).insert(ShouldRotate);
            }
        } else {
            commands.spawn((
                SpriteBundle {
                    texture,
                    transform: Transform::from_translation(tower_position).with_scale(tower_scale),
                    ..default()
                },
                Tower {
                    tower_type: *tower_type,
                    cooldown: tower_type.cooldown(),
                }
            ))
            .with_children(|parent| {
                parent.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Circle::default()).into(),
                        material: materials.add(Color::rgba(0., 0., 0., 0.5)),
                        transform: Transform::from_translation(Vec3::new(0., 0., -0.5)).with_scale(tower_radius),
                        ..default()
                    },
                    TowerRadiusIndicator,
                ));
            });
        }
        next_state.set(UiState::Normal);
        gold.0 -= tower_type.price();
    }
}

pub fn tower_get_target(
    mut tower_query: Query<(Entity, &Tower, &mut Transform), (Without<Target>, Without<Enemy>)>,
    enemy_query: Query<(Entity, &Transform, &Enemy), With<Enemy>>,
    mut commands: Commands,
) {
    for (tower_entity, tower, mut tower_pos) in tower_query.iter_mut() {
        for (enemy_entity, enemy_pos, enemy) in enemy_query.iter() {
            if tower_pos.translation.distance(enemy_pos.translation) <= tower.tower_type.range() && enemy.calc_health > 0 {
                commands.entity(tower_entity).insert(Target(enemy_entity));
                //Snap to enemy
                //Getting the angle between default tower rotation and enemy
                let angle_to_enemy = (enemy_pos.translation.truncate() - tower_pos.translation.truncate()).angle_between(Vec2::new(0.,1.));
                //Calculating the rotation of the tower, so that it "looks" into the direction of the enemy
                //TAU is The full circle constant (τ) and equal to 2π.
                tower_pos.rotation = Quat::from_rotation_z(- angle_to_enemy - std::f32::consts::TAU);
            }
        }
    }
}

pub fn tower_lost_target(
    tower_query: Query<(Entity, &Tower, &Target, &Transform)>,
    enemy_query: Query<(&Transform, &Enemy), With<Enemy>>,
    mut commands: Commands,
) {
    for (tower_entity, tower, target, tower_pos) in tower_query.iter() {
        if let Ok((enemy_pos, enemy)) = enemy_query.get(target.0) {
            if tower_pos.translation.distance(enemy_pos.translation) > tower.tower_type.range() || enemy.calc_health == 0 {
                commands.entity(tower_entity).remove::<Target>();
            }
        } else {
            commands.entity(tower_entity).remove::<Target>();
        }
    }
}

pub fn tower_rotate_at_target(
    mut tower_query: Query<(&Target, &mut Transform), (With<Target>, With<ShouldRotate>, Without<Enemy>)>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for (target, mut tower_pos) in tower_query.iter_mut() {
        if let Ok(target) = enemy_query.get(target.0) {
            //Rotate with enemy
            //Getting the angle between default tower rotation and enemy
            let angle_to_enemy = (target.translation.truncate() - tower_pos.translation.truncate()).angle_between(Vec2::new(0.,1.));
            //Calculating the rotation of the tower, so that it "looks" into the direction of the enemy
            //TAU is The full circle constant (τ) and equal to 2π.
            tower_pos.rotation = Quat::from_rotation_z(- angle_to_enemy - std::f32::consts::TAU);
        }
    }
}

pub fn tower_charge(
    mut tower_query: Query<(Entity, &mut Tower), Without<IsCharged>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (tower_entity, mut tower) in tower_query.iter_mut() {
        tower.cooldown.tick(time.delta());
        if tower.cooldown.just_finished() {
            commands.entity(tower_entity).insert(IsCharged);
        }
    }
}

pub fn tower_animate_charging(
    mut tower_query: Query<(&mut AnimationTimer, &mut TextureAtlas), (Without<IsCharged>, With<Tower>, Without<IsShooting>)>,
    time: Res<Time>,
) {
    for (mut timer, mut atlas) in &mut tower_query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if atlas.index > 0 {
                atlas.index -= 1;
            }
        }
    }
}

pub fn tower_animate_shooting(
    mut tower_query: Query<(Entity, &AnimationIndices, &mut AnimationTimer, &mut TextureAtlas), (With<IsShooting>, With<Tower>)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, indices, mut timer, mut atlas) in &mut tower_query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if atlas.index < indices.last {
                atlas.index += 1;
            } else if atlas.index == indices.last {
                commands.entity(entity).remove::<IsShooting>();
            }
        }
    }
}

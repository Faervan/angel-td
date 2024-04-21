use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use super::{
    tower_types::*,
    components::*,
};

pub fn spawn_tower(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    //Spawn "tower"
    let tower_type = &TowerType::XBow;
    let tower_position = Vec3::new(42., -113., 0.);
    let texture = asset_server.load(tower_type.sprite());
    let tower_scale = Vec3::new(tower_type.scale(), tower_type.scale(), 0.);
    let tower_radius = Vec3::new(tower_type.range() * 2. / tower_type.scale(), tower_type.range() * 2. / tower_type.scale(), 0.);
    if let Some((width, height, grid_columns, animation_frame_duration)) = tower_type.has_animation() {
        let layout = TextureAtlasLayout::from_grid(Vec2::new(width, height), grid_columns, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        // Use only the subset of sprites in the sheet that make up the run animation
        let animation_indices = AnimationIndices { first: 0, last: grid_columns-1, forward: true };
        commands.spawn((
            SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                transform: Transform::from_translation(tower_position).with_scale(tower_scale),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(animation_frame_duration, TimerMode::Repeating)),
            *tower_type,
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
    } else {
        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform::from_translation(tower_position).with_scale(tower_scale),
                ..default()
            },
            *tower_type,
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
}

pub fn tower_get_target(
    tower_query: Query<(Entity, &TowerType, &Transform), Without<Target>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut commands: Commands,
) {
    for (tower_entity, tower_type, tower_pos) in tower_query.iter() {
        for (enemy_entity, enemy_pos) in enemy_query.iter() {
            if tower_pos.translation.distance(enemy_pos.translation) <= tower_type.range() {
                commands.entity(tower_entity).insert(Target(enemy_entity));
                println!("Tower got Target!");
            }
        }
    }
}

pub fn tower_lost_target(
    tower_query: Query<(Entity, &TowerType, &Target, &Transform), With<Target>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut commands: Commands,
) {
    for (tower_entity, tower_type, target, tower_pos) in tower_query.iter() {
        let target = enemy_query.get(target.0).unwrap().translation;
        if tower_pos.translation.distance(target) > tower_type.range() {
            commands.entity(tower_entity).remove::<Target>();
            println!("Tower lost Target!");
        }
    }
}
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

use crate::{components::GameCursor, tower_types::TowerType};

use super::UiState;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(UiState::TowerPlacing), set_custom_cursor)
            .add_systems(Update, move_cursor.run_if(in_state(UiState::TowerPlacing)))
            .add_systems(OnExit(UiState::TowerPlacing), remove_custom_cursor);
    }
}

pub fn set_custom_cursor (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tower_type = TowerType::XBow;
    let tower_radius = Vec3::new(tower_type.range() * 2. / tower_type.scale(), tower_type.range() * 2. / tower_type.scale(), 0.);
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor.visible = false;
        let cursor_spawn = window.cursor_position().unwrap().extend(15.);
        println!("Cursor_spawn: {}", cursor_spawn);
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/turrets/ballista_full.png").into(),
                transform: Transform::from_translation(cursor_spawn).with_scale(Vec3::new(tower_type.scale(), tower_type.scale(), 0.)),
                ..default()
            },
            GameCursor {}
        ))
        .with_children(|parent| {
            parent.spawn(
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::default()).into(),
                    material: materials.add(Color::rgba(0., 0., 0., 0.5)),
                    transform: Transform::from_translation(Vec3::new(0., 0., -0.5)).with_scale(tower_radius),
                    ..default()
                }
            );
        });
    }
}

pub fn move_cursor (
    window: Query<&Window, With<PrimaryWindow>>,
    mut cursor: Query<&mut Transform, With<GameCursor>>,
) {
    if let Ok(window) = window.get_single() {
        if let Some(cursor_pos) = window.cursor_position() {
            let mut img_pos = cursor.get_single_mut().unwrap();
            img_pos.translation.x = cursor_pos.x - window.resolution.width() / 2.;
            img_pos.translation.y = (cursor_pos.y - window.resolution.height() / 2.) * -1.;
        }
    }
}

pub fn remove_custom_cursor (
    mut commands: Commands,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    cursor: Query<Entity, With<GameCursor>>,
) {
    if let (Ok(cursor), Ok(mut window)) = (cursor.get_single(), windows.get_single_mut()) {
        commands.entity(cursor).despawn_recursive();
        window.cursor.visible = true;
    }
}
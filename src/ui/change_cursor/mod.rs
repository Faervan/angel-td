use bevy::{prelude::*, ui::widget::UiImageSize, window::PrimaryWindow};

use crate::{components::GameCursor, tower_types::TowerType};

use super::UiState;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<CursorState>()
            .add_systems(OnEnter(UiState::TowerPlacing), set_custom_cursor)
            .add_systems(Update, move_cursor.run_if(in_state(UiState::TowerPlacing)))
            .add_systems(OnExit(UiState::TowerPlacing), remove_custom_cursor);
    }
}

#[derive(States, Hash, Debug, Default, Clone, Eq, PartialEq)]
pub enum CursorState {
    #[default]
    Normal,
    Tower(TowerType),
}

pub fn set_custom_cursor (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor.visible = false;
        let cursor_spawn = window.cursor_position().unwrap().extend(0.);
        println!("Cursor_spawn: {}", cursor_spawn);
    
        commands.spawn((
            ImageBundle {
                image: asset_server.load("sprites/turrets/ballista_full.png").into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    ..default()
                },
                z_index: ZIndex::Global(15),
                transform: Transform::from_translation(cursor_spawn).with_scale(Vec3::new(TowerType::XBow.scale(), TowerType::XBow.scale(), 0.)),
                ..default()
            },
            GameCursor {}
        ));
    }
}

pub fn move_cursor (
    window: Query<&Window, With<PrimaryWindow>>,
    mut cursor: Query<(&mut Style, &UiImageSize), With<GameCursor>>,
) {
    if let Ok(window) = window.get_single() {
        if let Some(cursor_pos) = window.cursor_position() {
            let (mut img_pos, img_size) = cursor.get_single_mut().unwrap();
            img_pos.left = Val::Px(cursor_pos.x - img_size.size().x / 2.);
            img_pos.top = Val::Px(cursor_pos.y - img_size.size().y / 2.);
            println!("cursor_pos: {cursor_pos}");
        }
    }
}

pub fn remove_custom_cursor (
    mut commands: Commands,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    cursor: Query<Entity, With<GameCursor>>,
) {
    if let (Ok(cursor), Ok(mut window)) = (cursor.get_single(), windows.get_single_mut()) {
        commands.entity(cursor).despawn();
        window.cursor.visible = true;
    }
}
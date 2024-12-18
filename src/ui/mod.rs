use bevy::prelude::*;

mod build;
mod styles;
mod update_ui;
mod change_cursor;

use build::*;
use update_ui::*;
use change_cursor::*;

use crate::{setup, spawn_tower, AppState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<UiState>()
            .add_plugins(CursorPlugin)
            .add_systems(OnEnter(AppState::InGame),
                build_hud.after(setup)
            )
            .add_systems(Update, (
                update_gold_count,
                update_wave_count,
                interact_with_tower_place_btn,
                update_tower_placing_state
                    .run_if(in_state(UiState::TowerPlacing(true))
                        .or_else(in_state(UiState::TowerPlacing(false))))
                    .before(spawn_tower),
            ).run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct UiBar;

#[derive(Component)]
pub struct UiGoldCount;

#[derive(Component)]
pub struct UiWaveCount;

#[derive(Component)]
pub struct TowerPlaceBtn;

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum UiState {
    #[default]
    Normal,
    TowerPlacing(bool),
}

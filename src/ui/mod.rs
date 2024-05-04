use bevy::prelude::*;

mod build;
mod styles;
mod update_ui;

use build::*;
use update_ui::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<UiState>()
            .add_systems(Startup, (
                build_hud
            ))
            .add_systems(Update, (
                update_gold_count,
                update_wave_count,
                interact_with_tower_place_btn,
            ));
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
    TowerPlacing,
}
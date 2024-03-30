use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_enemy)
            .add_systems(Update, (
                enemy_movement,
            ));
    }
}
use bevy::prelude::*;

mod towers;
mod enemies;
pub mod maps;

mod systems;
mod components;

use {enemies::*, towers::*, maps::*, systems::*};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                EnemyPlugin,
                TowerPlugin,
                MapPlugin,
            ))
            .add_systems(Update, (
                permanent_animated_sprites,
            ));
    }
}
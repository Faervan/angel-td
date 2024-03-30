use bevy::prelude::*;

mod towers;
mod enemies;
pub mod maps;
use {enemies::*, towers::*, maps::*};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                EnemyPlugin,
                TowerPlugin,
                MapPlugin,
            ));
    }
}
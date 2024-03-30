use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_map);
    }
}
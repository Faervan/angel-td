use bevy::prelude::*;

mod components;
mod systems;

use systems::*;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (spawn_tower, spawn_bullet))
            .add_systems(Update, (
                rotate_ballista,
                tower_check_for_enemies_in_range,
                move_bullet,
                bullet_hits_enemy
            ));
    }
}
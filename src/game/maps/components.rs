use bevy::prelude::*;

#[derive(Component)]
pub struct EnemyPath {
    pub path_points: Vec<Vec2>
}
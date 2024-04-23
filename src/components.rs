use bevy::prelude::*;
use super::enemy_types::EnemyType;
use super::tower_types::TowerType;

#[derive(Component)]
pub struct EnemyPath {
    pub path_points: Vec<Vec2>
}

#[derive(Component, Debug)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub path_state: usize,
}

#[derive(Component)]
pub struct Tower {
    pub tower_type: TowerType,
    pub cooldown: Timer,
}

#[derive(Component)]
pub struct TowerRadiusIndicator;

#[derive(Component, Debug)]
pub struct Target(pub Entity);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
    pub forward: bool
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
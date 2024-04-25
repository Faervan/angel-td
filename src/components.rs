use bevy::prelude::*;

use super::enemy_types::EnemyType;
use super::tower_types::TowerType;
use super::bullet_types::BulletType;

#[derive(Component)]
pub struct EnemyPath {
    pub path_points: Vec<Vec2>
}

#[derive(Component, Debug)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub path_state: usize,
    pub real_health: usize, //Real Health (reduced if Bullet hits Enemy)
    pub calc_health: usize, //Health as detected by Towers (reduced if Tower fires bullet)
}

#[derive(Component)]
pub struct Tower {
    pub tower_type: TowerType,
    pub cooldown: Timer,
}
#[derive(Component)]
pub struct ShouldRotate;

#[derive(Component)]
pub struct IsCharged;

#[derive(Component)]
pub struct IsShooting;

#[derive(Component)]
pub struct Bullet {
    pub origin: Entity,
    pub target: Entity,
    pub bullet_type: BulletType,
    pub damage: usize,
}

#[derive(Component)]
pub struct TowerRadiusIndicator;

#[derive(Component, Debug)]
pub struct Target(pub Entity);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
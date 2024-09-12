use bevy::prelude::*;
use super::{
    bullet_types::*,
    components::*,
};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum TowerType {
    XBow,
}

impl TowerType {
    pub fn cooldown(&self) -> Timer {
        match self {
            TowerType::XBow => Timer::from_seconds(0.2, TimerMode::Repeating),
        }
    }
    pub fn bullet_type(&self) -> BulletType {
        match self {
            TowerType::XBow => BulletType::RedBlob,
        }
    }
    pub fn range(&self) -> f32 {
        match self {
            TowerType::XBow => 200.,
        }
    }
    pub fn damage(&self) -> usize {
        match self {
            TowerType::XBow => 12,
        }
    }
    pub fn price(&self) -> usize {
        match self {
            TowerType::XBow => 50,
        }
    }
    pub fn sprite(&self) -> &str {
        match self {
            TowerType::XBow => "sprites/turrets/ballista_bow_sheet.png",
        }
    }
    pub fn scale(&self) -> f32 {
        match self {
            TowerType::XBow => 1.3,
        }
    }
    // If tower has an animation, returns (width, height, grid_columns)
    pub fn has_animation(&self) -> Option<(f32, f32, u8)> {
        match self {
            TowerType::XBow => Some((100., 100., 4)),
        }
    }
    pub fn has_rotation(&self) -> bool {
        match self {
            TowerType::XBow => true,
        }
    }
    pub fn has_required_space(&self, position: &Vec3, tower_query: Query<&Transform, With<Tower>>) -> bool {
        for tower in tower_query.iter() {
            if tower.translation.distance(*position) <= 50. {
                return false;
            }
        }
        true
    }
}

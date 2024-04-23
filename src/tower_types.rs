use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use super::bullet_types::*;

#[derive(Debug, Clone, Copy)]
pub enum TowerType {
    XBow,
}

impl TowerType {
    pub fn cooldown(&self) -> Timer {
        match self {
            TowerType::XBow => Timer::from_seconds(0.3, TimerMode::Repeating),
        }
    }
    pub fn bullet_type(&self) -> BulletType {
        match self {
            TowerType::XBow => BulletType::RedBlob,
        }
    }
    pub fn range(&self) -> f32 {
        match self {
            TowerType::XBow => 150.,
        }
    }
    //Returns a sprite and (if it's a sheet) the number of grid columns
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
    // If tower has an animation, returns (width, height, grid_columns, animation_frame_duration)
    pub fn has_animation(&self) -> Option<(f32, f32, usize, f32)> {
        match self {
            TowerType::XBow => Some((100., 100., 4, 0.05)),
        }
    }
}
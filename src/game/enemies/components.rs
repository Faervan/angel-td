use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Enemy {
    pub velocity: f32,
    pub path_state: usize
}
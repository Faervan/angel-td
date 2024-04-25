use std::collections::HashMap;
use std::collections::VecDeque;

use bevy::ecs::system::Resource;
use bevy::time::Timer;

use super::enemy_types::EnemyType;

#[derive(Debug)]
pub struct WaveRange {
    pub lowest_level: u16,
    pub lowest_probability: u16,
    pub highest_level: u16,
    pub highest_probability: u16,
}

#[derive(Resource)]
pub struct Waves{
    pub current: u16,
    pub queue: VecDeque<EnemyType>,
    pub wave_margin: Timer,
    pub spawn_delay: Timer,
}

#[derive(Resource)]
pub struct WaveMap {
    pub waves: u16,
    pub wave_range: HashMap<EnemyType, WaveRange>
}
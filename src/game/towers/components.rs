use bevy::prelude::*;

#[derive(Component)]
pub struct Tower {
    pub range: f32
}

//For testing purpose
#[derive(Component)]
pub struct TowerRadiusIndicator {
    pub range: f32
}

#[derive(Component)]
pub struct TowerBullet {
    pub velocity: f32,
    pub direction: Vec3
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
    pub forward: bool
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Rotatable {
    pub speed: f32,  //Rotations per second
}
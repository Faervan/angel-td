use bevy::prelude::*;
use super::components::*;

//Walking animations and such (not yet used)
pub fn permanent_animated_sprites (
    time: Res<Time>,
    mut query: Query<(&mut AnimationIndices, &mut AnimationTimer, &mut TextureAtlas), With<PermanentAnimation>>,
) {
    for (mut indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.forward = false;
                atlas.index - 1
            } else if atlas.index == indices.first {
                indices.forward = true;
                atlas.index + 1
            } else if indices.forward {
                atlas.index + 1
            } else {
                atlas.index -1
            };
        }
    }
}

//Shouting animations and such
pub fn once_animated_sprites (
    time: Res<Time>,
    mut query: Query<(&mut AnimationIndices, &mut AnimationTimer, &mut TextureAtlas, Entity), With<ExecuteAnimationOnce>>,
    mut commands: Commands
) {
    for (mut indices, mut timer, mut atlas, entity) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.forward = false;
                atlas.index - 1
            } else if atlas.index == indices.first && !indices.forward {
                indices.forward = true;
                commands.entity(entity).remove::<ExecuteAnimationOnce>();
                atlas.index
            } else if indices.forward {
                atlas.index + 1
            } else {
                atlas.index -1
            };
        }
    }
}
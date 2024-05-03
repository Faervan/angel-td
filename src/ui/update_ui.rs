use bevy::prelude::*;

use crate::{enemy_wave_map::{WaveMap, Waves}, Gold};

use super::{styles::get_count_text_style, UiGoldCount, UiWaveCount};

pub fn update_gold_count (
    mut gold_count_query: Query<&mut Text, With<UiGoldCount>>,
    gold: Res<Gold>,
    asset_server: Res<AssetServer>
) {
    if gold.is_changed() {
        if let Ok(mut text) = gold_count_query.get_single_mut() {
            *text = Text {
                sections: vec![
                    TextSection::new(
                        format!("Gold: {}", gold.0),
                        get_count_text_style(&asset_server)
                    )
                ],
                justify: JustifyText::Center,
                ..default()
            };
        }
    }
}

pub fn update_wave_count (
    mut wave_count_query: Query<&mut Text, With<UiWaveCount>>,
    wave: (Res<Waves>, Res<WaveMap>),
    asset_server: Res<AssetServer>
) {
    if wave.0.is_changed() {
        if let Ok(mut text) = wave_count_query.get_single_mut() {
            *text = Text {
                sections: vec![
                    TextSection::new(
                        format!("Wave {}/{}", wave.0.current, wave.1.waves),
                        get_count_text_style(&asset_server)
                    )
                ],
                justify: JustifyText::Center,
                ..default()
            };
        }
    }
}
use bevy::prelude::*;

use crate::Gold;

use super::{styles::get_count_text_style, UiGoldCount};

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
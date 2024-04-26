use bevy::prelude::*;

use crate::{ui::styles::{COUNT_STYLE, NORMAL_COUNT_COLOR}, Gold};
use super::{styles::{get_count_text_style, UI_BAR_STYLE}, UiBar, UiGoldCount};

pub fn build_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gold: Res<Gold>,
) {
    commands.spawn((
        NodeBundle {
            style: UI_BAR_STYLE,
            ..default()
        },
        UiBar,
    ))
    .with_children(|parent| {
        // == Gold Count ==
        parent.spawn(
            NodeBundle {
                style: COUNT_STYLE,
                background_color: NORMAL_COUNT_COLOR.into(),
                ..default()
            }
        )
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                format!("Gold: {}", gold.0),
                                get_count_text_style(&asset_server),
                            )
                        ],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    ..default()
                },
                UiGoldCount,
            ));
        });
    });
    println!("spawned ui");
}
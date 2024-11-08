use bevy::prelude::*;

use crate::{enemy_wave_map::{WaveMap, Waves}, ui::{styles::{COUNT_STYLE, NORMAL_COUNT_COLOR}, TowerPlaceBtn, UiWaveCount}, Gold};
use super::{styles::{get_count_text_style, UI_BAR_STYLE}, UiBar, UiGoldCount};

pub fn build_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gold: Res<Gold>,
    wave: (Res<Waves>, Res<WaveMap>),
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
        // == Wave Count ==
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
                                format!("Wave {}/{}", wave.0.current, wave.1.waves),
                                get_count_text_style(&asset_server),
                            )
                        ],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    ..default()
                },
                UiWaveCount,
            ));
        });
    });
    commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexEnd,
                padding: UiRect {
                    left: Val::Px(10.),
                    bottom: Val::Px(10.),
                    right: Val::Px(10.),
                    top: Val::Px(0.),
                },
                ..default()
            },
            ..default()
        }
    )
    .with_children(|parent| {
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(100.),
                    height: Val::Px(100.),
                    ..default()
                },
                image: asset_server.load("sprites/ui/Gear-hammer_-_Lorc_-_white_-_game-icons.svg.png").into(),
                ..default()
            },
            TowerPlaceBtn,
        ));
    });
    println!("spawned ui");
}

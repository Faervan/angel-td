use bevy::{prelude::*, window::PrimaryWindow};

use crate::{components::Tower, enemy_wave_map::{WaveMap, Waves}, tower_types::TowerType, Gold, SCREENHEIGTH, SCREENWIDTH};

use super::{styles::get_count_text_style, TowerPlaceBtn, UiGoldCount, UiState, UiWaveCount};

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

pub fn interact_with_tower_place_btn (
    mut button_query: Query<(&Interaction, &mut UiImage), (Changed<Interaction>, With<TowerPlaceBtn>)>,
    mut next_state: ResMut<NextState<UiState>>,
) {
    if let Ok((interaction, mut ui_image)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(UiState::TowerPlacing(true));
            },
            Interaction::Hovered => {
                ui_image.color = Color::srgb(0.5, 0.5, 0.5);
            },
            Interaction::None => {
                ui_image.color = Color::WHITE;
            }
        }
    }
}

pub fn update_tower_placing_state(
    window: Query<&Window, With<PrimaryWindow>>,
    gold: ResMut<Gold>,
    tower_query: Query<&Transform, With<Tower>>,
    ui_state: Res<State<UiState>>,
    mut next_state: ResMut<NextState<UiState>>,
) {
    let tower_type = &TowerType::XBow;
    if let Some(cursor_pos) = window.get_single().unwrap().cursor_position() {
        let tower_position = Vec3::new(cursor_pos.x - SCREENWIDTH / 2., (cursor_pos.y - SCREENHEIGTH / 2.) * -1., 0.);
        if *ui_state.get() != UiState::Normal {
            if gold.0 >= tower_type.price() && tower_type.has_required_space(&tower_position, tower_query) {
                next_state.set(UiState::TowerPlacing(true));
            } else {
                next_state.set(UiState::TowerPlacing(false));
            }
        }
    }
}

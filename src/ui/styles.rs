use bevy::prelude::*;

pub const NORMAL_COUNT_COLOR: Color = Color::rgba(0.15, 0.15, 0.15, 0.5);

pub const UI_BAR_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(0.0);
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::SpaceBetween;
    style.position_type = PositionType::Absolute;
    style
};

pub const COUNT_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(200.0);
    style.height = Val::Px(80.0);
    style.align_items = AlignItems::Center;
    style.justify_content = JustifyContent::Center;
    style
};

pub fn get_count_text_style (asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}
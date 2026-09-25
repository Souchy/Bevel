use crate::{Sets, plugins::gold::gold::GoldBank};
use bevy::{prelude::*, text::FontSize::Px};

#[derive(Component, Default)]
struct GoldText {
    displayed_amount: Option<f64>,
}

pub struct GoldUiPlugin;
impl Plugin for GoldUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui.in_set(Sets::Ui))
            .add_systems(Update, update_gold_text.in_set(Sets::Ui).after(Sets::Logic));
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        GoldText {
            displayed_amount: None,
        },
        Text::new("Gold: 0"),
        TextFont {
            font_size: Px(32.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            left: px(16),
            top: px(16),
            ..default()
        },
    ));
}

fn update_gold_text(gold_bank: Res<GoldBank>, mut texts: Query<(&mut Text, &mut GoldText)>) {
    let rounded = gold_bank.amount.floor();

    for (mut text, mut display) in &mut texts {
        if display.displayed_amount == Some(rounded) {
            continue;
        }

        text.0 = format!("Gold: {rounded}");
        display.displayed_amount = Some(rounded);
    }
}

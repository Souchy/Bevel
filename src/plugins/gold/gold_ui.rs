use crate::lib::ui::components::collapsible_panel::collapsible_panel;
use crate::{Sets, plugins::gold::gold::GoldBank};
use bevy::scene::prelude::*;
use bevy::{color::palettes::css::GOLD, prelude::*};

#[derive(Component, Clone, Default)]
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
    commands.spawn_scene(bsn! {
        collapsible_panel("Gold".to_string(), bsn_list![
            (
                GoldText {
                    displayed_amount: None,
                }
                Text::new("Gold: 0")
                TextFont {
                    font_size: FontSize::Px(32.0),
                }
                TextColor(GOLD)
            )
        ])
        Node {
            width: px(200.0),
        }
        BorderColor::all(GOLD)
    });
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

use crate::{Sets, plugins::{gold::gold::GoldBank, ui::draggable_panel::DraggablePanel}};
use bevy::{color::palettes::css::GOLD, prelude::*, text::FontSize::Px};

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
    commands
        .spawn((
            DraggablePanel::default(),
            Node {
                position_type: PositionType::Absolute,
                left: px(16),
                top: px(16),
                // width: px(200),
                width: Val::Auto,
                // height: px(80),
                height: Val::Auto,
                padding: UiRect::all(px(16)), // Adds breathing room inside the panel
                // 1. Set a non-zero border width on the Node
                border: UiRect::all(Val::Px(1.0)),
                // 2. Set the corner radius for rounding
                border_radius: BorderRadius::all(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)), // Dark grey, semi-transparent background
            BorderColor::all(GOLD),
            BoxShadow(vec![ShadowStyle {
                // A transparent black/dark color for a soft look
                color: Color::srgba(0.0, 0.0, 0.0, 0.6),
                // Horizontal (X) and Vertical (Y) offsets
                x_offset: Val::Px(0.0),
                y_offset: Val::Px(8.0),
                // Blurriness of the shadow edge
                blur_radius: Val::Px(12.0),
                // How much the shadow expands outwards past the node bounds
                spread_radius: Val::Px(2.0),
            }]),
        ))
        .with_children(|panel| {
            panel.spawn((
                GoldText {
                    displayed_amount: None,
                },
                Text::new("Gold: 0"),
                TextFont {
                    font_size: Px(32.0),
                    ..default()
                },
                Node::default(),
            ));
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

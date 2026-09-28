use bevy::prelude::*;

use crate::lib::ui::{
    properties::collapsible::CollapseToggle, styling::interaction_style::InteractionStyle,
};

pub fn bsn_collapse_button(tag: String) -> impl Scene {
    bsn! {
        Button
        CollapseToggle::new(tag)
        Node {
            width: px(16.0),
            height: px(16.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(px(8)),
        }
        InteractionStyle::<BackgroundColor>::new(
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)), // Dark grey, semi-transparent background
            BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.8)),
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
        )
        Children [
            Text::new("v"),
            TextFont {
                font_size: FontSize::Px(16.0),
            },
            TextColor(Color::WHITE),
        ]
    }
}

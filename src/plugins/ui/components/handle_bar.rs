use bevy::prelude::*;

use crate::plugins::ui::{
    properties::draggable::Draggable, styling::interaction_style::InteractionStyle,
};

pub fn bsn_handle_bar(ancestor_level: usize) -> impl Scene {
    bsn! {
        Draggable::new(ancestor_level)
        Node {
            width: Val::Percent(100.0),
            height: px(20),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        InteractionStyle::<BackgroundColor>::new(
            BackgroundColor(Color::srgba(0.01, 0.01, 0.01, 0.8)), // Dark grey, semi-transparent background
            BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.8)),
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
        )
    }
}

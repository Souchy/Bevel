use bevy::prelude::*;

use crate::plugins::ui::{
    components::{collapse_button::bsn_collapse_button, handle_bar::bsn_handle_bar},
    properties::collapsible::CollapsibleContent,
};

pub fn collapsible_panel(
    panel_title: String,
    contents: impl SceneList,
) -> impl Scene {
    let header_title = panel_title.clone();
    let collapse_toggle_id = panel_title.clone();
    let collapse_content_id = panel_title;

    bsn! {
        // Root panel container
        Node {
            flex_direction: FlexDirection::Column,
            // width: px(200),
            width: Val::Auto,
            height: Val::Auto,
            padding: UiRect::all(px(16)),
            border: UiRect::all(Val::Px(1.0)),
            border_radius: BorderRadius::all(Val::Px(16.0)),
        }
        BackgroundColor(Color::srgb(0.1, 0.1, 0.1))
        BoxShadow(vec![ShadowStyle {
            color: Color::srgba(0.0, 0.0, 0.0, 0.6),
            x_offset: Val::Px(0.0),
            y_offset: Val::Px(8.0),
            blur_radius: Val::Px(12.0),
            spread_radius: Val::Px(2.0),
        }])
        Children [
            // Header
            (
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: px(8.0)
                }
                Children [
                    // Handle + Title bar
                    (
                        bsn_handle_bar(2)
                        Children [
                            Text::new(header_title)
                        ]
                    ),
                    // Collapse button
                    (
                        bsn_collapse_button(collapse_toggle_id)
                    ),
                ]
            ),
            // Contents
            (
                CollapsibleContent::new(collapse_content_id)
                Node {
                    flex_direction: FlexDirection::Column,
                }
                Children [
                    { contents }
                ]
            )
        ]
    }
}

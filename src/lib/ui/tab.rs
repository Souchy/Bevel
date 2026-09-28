use bevy::prelude::*;

use crate::lib::ui::styling::interaction_style::InteractionStyle;

#[derive(Component, Clone, Default)]
pub struct TabGroup {
    selected: usize,
}

impl TabGroup {
    pub fn new(selected: usize) -> Self {
        Self { selected }
    }
}

#[derive(Component, Clone)]
pub struct TabButton {
    pub group: Entity,
    pub index: usize,
}

impl TabButton {
    pub fn new(group: Entity, index: usize) -> Self {
        Self { group, index }
    }
}

impl Default for TabButton {
    fn default() -> Self {
        Self {
            group: Entity::PLACEHOLDER,
            index: 0,
        }
    }
}

#[derive(Component, Clone)]
pub struct TabPanel {
    pub group: Entity,
    pub index: usize,
}

impl TabPanel {
    pub fn new(group: Entity, index: usize) -> Self {
        Self { group, index }
    }
}

impl Default for TabPanel {
    fn default() -> Self {
        Self {
            group: Entity::PLACEHOLDER,
            index: 0,
        }
    }
}

pub struct TabPlugin;

impl Plugin for TabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (select_tab, update_tab_panels).chain());
    }
}

// fn select_tab(
//     mut groups: Query<&mut TabGroup>,
//     buttons: Query<(&Interaction, &TabButton), Changed<Interaction>>,
// ) {
//     for (interaction, button) in &buttons {
//         if *interaction == Interaction::Pressed {
//             if let Ok(mut group) = groups.get_mut(button.group) {
//                 group.selected = button.index;
//             }
//         }
//     }
// }

// fn update_tab_panels(
//     groups: Query<(Entity, &TabGroup), Changed<TabGroup>>,
//     mut panels: Query<(&TabPanel, &mut Node)>,
// ) {
//     for (group_entity, group) in &groups {
//         for (panel, mut node) in &mut panels {
//             if panel.group == group_entity {
//                 node.display = if panel.index == group.selected {
//                     Display::Flex
//                 } else {
//                     Display::None
//                 };
//             }
//         }
//     }
// }

fn select_tab(
    mut groups: Query<&mut TabGroup>,
    buttons: Query<(Entity, &Interaction, &TabButton), Changed<Interaction>>,
    parents: Query<&ChildOf>,
) {
    for (button_entity, interaction, button) in &buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let mut ancestor = button_entity;
        while let Ok(child_of) = parents.get(ancestor) {
            ancestor = child_of.parent();

            if let Ok(mut group) = groups.get_mut(ancestor) {
                group.selected = button.index;
                break;
            }
        }
    }
}

fn update_tab_panels(
    groups: Query<(Entity, &TabGroup), Changed<TabGroup>>,
    mut panels: Query<(Entity, &TabPanel, &mut Node)>,
    parents: Query<&ChildOf>,
) {
    for (group_entity, group) in &groups {
        for (panel_entity, panel, mut node) in &mut panels {
            let mut ancestor = panel_entity;
            let mut belongs_to_group = false;

            while let Ok(child_of) = parents.get(ancestor) {
                ancestor = child_of.parent();

                if ancestor == group_entity {
                    belongs_to_group = true;
                    break;
                }
            }

            if belongs_to_group {
                node.display = if panel.index == group.selected {
                    Display::Flex
                } else {
                    Display::None
                };
            }
        }
    }
}

// pub struct TabBundleBuilder {
//     pub button_style: ButtonBundle,
//     pub panel_style: PanelBundle,
//     // tabs: Vec<(String, Box<dyn FnOnce(&mut UiChildBuilder) + 'a>)>,
// }

// impl TabBundleBuilder {
// 	// pub fn add_tab(&mut self) {

// 	// }

// 	pub fn spawn_tab_bar(&self, mut commands: Commands, tabs: Vec<impl Bundle>) {
// 		for (index, name) in tab_names.into_iter().enumerate() {

// 		}
// 	}
// }

// pub struct Tab {
// 	pub name: String,
// 	pub panel: Bundle
// }

pub fn bsn_tabs(
    titles: Vec<String>,
    // tab_contents: impl SceneList
    tab_contents: Vec<Box<dyn Scene>>,
) -> impl Scene {
    let group_id = Entity::PLACEHOLDER;

    let buttons = titles
        .into_iter()
        .enumerate()
        .map(|(index, title)| {
            bsn! {
                Name(format!("Tab Button {}", index))
                Button
                TabButton::new(group_id, index)
                Text::new(title)
                Node {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(px(8)),
                }
                InteractionStyle::<BackgroundColor>::new(
                    BackgroundColor(Color::srgba(0.05, 0.05, 0.05, 0.8)),
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
                    BackgroundColor(Color::srgba(0.15, 0.15, 0.15, 0.8)),
                )
            }
        })
        .collect::<Vec<_>>();

    let tab_panels = tab_contents
        .into_iter()
        .enumerate()
        .map(|(index, content)| {
            bsn! {
                Name(format!("Tab Panel {}", index))
                TabPanel::new(group_id, index)
                content
            }
        })
        .collect::<Vec<_>>();

    let tab_group = bsn! {
        // group
        Name("TabUI")
        TabGroup::new(0)
        Node {
            flex_direction: FlexDirection::Column,
            row_gap: px(16.0),
            // width: px(200),
            width: Val::Auto,
            height: Val::Auto,
            // padding: UiRect::all(px(16)),
            // border: UiRect::all(Val::Px(1.0)),
            // border_radius: BorderRadius::all(Val::Px(16.0)),
        }
    };

    bsn! {
        tab_group
        Children[
            // Tabs Buttons bar
            (
                Name("Tab Bar")
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: px(8.0),
                }
                Children [
                    { buttons }
                ]
            ),
            // Tabs Contents
            (
                Name("Tab Container")
                Node {
                    flex_direction: FlexDirection::Column,
                }
                Children [
                    { tab_panels }
                ]
            )
        ]
    }
}

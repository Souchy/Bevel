use crate::{
    Sets, UiEnabled, plugins::ui::{
        main_ui::MainUiPlugin, properties::{collapsible::CollapsiblePlugin, draggable::DraggablePlugin}, styling::interaction_style::interactive_style_system, tab::TabPlugin,
    },
};
use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod components;
pub mod properties;
pub mod styling;
pub mod tab;
pub mod main_ui;

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BaseUiPlugin)
            .add(DraggablePlugin)
            .add(TabPlugin)
            .add(CollapsiblePlugin)
            .add(MainUiPlugin)
    }
}

struct BaseUiPlugin;

impl Plugin for BaseUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiEnabled>()
            .add_systems(Startup, setup_ui.in_set(Sets::Ui))
            .add_systems(
                Update,
                (
                    interactive_style_system::<BackgroundColor>,
                    interactive_style_system::<BorderColor>,
                    interactive_style_system::<BoxShadow>,
                    interactive_style_system::<TextColor>,
                    interactive_style_system::<Node>,
                ),
            );
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Bundle, Default)]
pub struct ButtonBundle {
    button: Button,
    pub node: Node,
    pub border_color: BorderColor,
    pub background_color: BackgroundColor,
}

#[derive(Bundle, Default)]
pub struct PanelBundle {
    pub node: Node,
    pub border_color: BorderColor,
    pub background_color: BackgroundColor,
    pub shadow: BoxShadow,
}

use crate::{
    Sets, UiEnabled, plugins::ui::{collapsible::CollapsiblePlugin, draggable_panel::DraggablePanelPlugin, styling::interaction_style::interactive_style_system, tab::TabPlugin},
};
use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod draggable_panel;
pub mod styling;
pub mod tab;
pub mod collapsible;

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BaseUiPlugin)
            .add(DraggablePanelPlugin)
            .add(TabPlugin)
            .add(CollapsiblePlugin)
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
                    // interactive_style_system::<BorderRadius>,
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


use bevy::prelude::*;

use crate::Sets;
use crate::plugins::ui::components::collapsible_panel::collapsible_panel;
use crate::plugins::ui::tab::bsn_tabs;

pub struct MainUiPlugin;
impl Plugin for MainUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui.in_set(Sets::Ui));
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        collapsible_panel("Main".to_string(), bsn_list![
            (
                bsn_tabs(
                    vec!["Buildings".to_string(), "Jobs".to_string()],
                    vec![
                        Box::new(bsn! {
                            Text::new("some building: 0")
                        }),
                        Box::new(bsn! {
                            Text::new("some job: 0")
                        }),
                    ]
                )
            )
        ])
        Node {
            width: px(500.0),
        }
        Name("main_ui")
    });
}

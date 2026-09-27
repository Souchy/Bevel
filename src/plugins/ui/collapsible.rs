use bevy::prelude::*;

#[derive(Component)]
pub struct CollapsibleContent {
    id: &'static str,
    base_display: Option<Display>,
}
impl CollapsibleContent {
    pub fn new(id: &'static str) -> Self {
        Self {
            id,
            base_display: None,
        }
    }
    pub fn with_base(id: &'static str, base_display: Display) -> Self {
        Self {
            id,
            base_display: Some(base_display),
        }
    }
}

#[derive(Component)]
pub struct CollapseToggle {
    id: &'static str,
}
impl CollapseToggle {
    pub fn new(id: &'static str) -> Self {
        Self { id }
    }
}

pub struct CollapsiblePlugin;

impl Plugin for CollapsiblePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_click);
    }
}

fn on_click(
    click: On<Pointer<Click>>,
    toggles: Query<&CollapseToggle>,
    mut contents: Query<(&mut CollapsibleContent, &mut Node)>,
) {
    let Ok(toggle) = toggles.get(click.event_target()) else {
        return;
    };

    for (mut content, mut node) in &mut contents {
        if content.id == toggle.id {
            if node.display == Display::None {
                if let Some(cached) = content.base_display {
                    node.display = cached;
                }
            } else {
                if content.base_display.is_none() {
                    content.base_display = Some(node.display);
                }
                node.display = Display::None;
            }
        }
    }
}

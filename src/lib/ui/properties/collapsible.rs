use bevy::prelude::*;

#[derive(Component, Clone, Default)]
pub struct CollapsibleContent {
    id: String,
    base_display: Option<Display>,
}
impl CollapsibleContent {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            base_display: None,
        }
    }
    pub fn with_base(id: impl Into<String>, base_display: Display) -> Self {
        Self {
            id: id.into(),
            base_display: Some(base_display),
        }
    }
}

#[derive(Component, Clone, Default)]
pub struct CollapseToggle {
    id: String,
}
impl CollapseToggle {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
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

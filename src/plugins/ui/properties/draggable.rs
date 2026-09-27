use bevy::prelude::*;

pub struct DraggablePlugin;

impl Plugin for DraggablePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_drag_start).add_observer(on_drag);
    }
}

#[derive(Component, Clone, Default)]
pub struct Draggable {
    ancestor_level: usize,
    start_pos: Vec2,
}
impl Draggable {
    pub fn new(ancestor_level: usize) -> Self {
        Self {
            ancestor_level,
            ..Default::default()
        }
    }
}

fn on_drag_start(
    trigger: On<Pointer<DragStart>>,
    mut handles: Query<&mut Draggable>,
    parents: Query<&ChildOf>,
    panels: Query<&UiTransform>,
) {
    let handle_entity = trigger.event_target();

    if let Ok(handle) = handles.get(handle_entity) {
        if let Some(target_entity) =
            get_target_entity(handle_entity, handle.ancestor_level, &parents)
        {
            if let Ok(transform) = panels.get(target_entity) {
                let x = match transform.translation.x {
                    Val::Px(val) => val,
                    _ => 0.0,
                };
                let y = match transform.translation.y {
                    Val::Px(val) => val,
                    _ => 0.0,
                };

                if let Ok(mut handle_mut) = handles.get_mut(handle_entity) {
                    handle_mut.start_pos = Vec2::new(x, y);
                }
            }
        }
    }
}

fn on_drag(
    trigger: On<Pointer<Drag>>,
    handles: Query<&Draggable>,
    parents: Query<&ChildOf>,
    mut panels: Query<&mut UiTransform>,
) {
    let handle_entity = trigger.event_target();

    if let Ok(handle) = handles.get(handle_entity) {
        if let Some(target_entity) =
            get_target_entity(handle_entity, handle.ancestor_level, &parents)
        {
            if let Ok(mut transform) = panels.get_mut(target_entity) {
                let new_x = handle.start_pos.x + trigger.distance.x;
                let new_y = handle.start_pos.y + trigger.distance.y;

                transform.translation = Val2::px(new_x, new_y);
            }
        }
    }
}

fn get_target_entity(
    start_entity: Entity,
    levels: usize,
    parent_relations: &Query<&ChildOf>,
) -> Option<Entity> {
    let mut current = start_entity;
    for _ in 0..levels {
        if let Ok(relation) = parent_relations.get(current) {
            current = relation.parent();
        } else {
            return None; // Hit the root of the tree before reaching target level
        }
    }
    Some(current)
}

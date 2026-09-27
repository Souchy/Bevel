use bevy::{
    ecs::{component::Mutable, lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};

#[derive(Component, Clone, Default)]
#[component(on_add = Self::on_add)]
pub struct InteractionStyle<T: Component + Clone> {
    pub normal: T,
    pub hovered: Option<T>,
    pub pressed: Option<T>,
}

impl<T: Component + Clone> InteractionStyle<T> {
    pub fn new(normal: T, hovered: T, pressed: T) -> Self {
        Self {
            normal,
            hovered: Some(hovered),
            pressed: Some(pressed),
        }
    }
    pub fn hovered(normal: T, hovered: T) -> Self {
        Self {
            normal,
            hovered: Some(hovered),
            pressed: None,
        }
    }
    pub fn pressed(normal: T, pressed: T) -> Self {
        Self {
            normal,
            hovered: None,
            pressed: Some(pressed),
        }
    }
}

impl<T: Component + Clone> InteractionStyle<T> {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let entity = context.entity;
        let default_val = world.get::<Self>(entity).unwrap().normal.clone();

        // Make sure to have a Interaction component
        if !world.entity(entity).contains::<Interaction>() {
            world.commands().entity(entity).insert(Interaction::None);
        }
        // Make sure to have the default value component
        if !world.entity(entity).contains::<T>() {
            world.commands().entity(entity).insert(default_val);
        }
    }
}

// A single unified system drives the state machine without fighting or ordering bugs!
pub fn interactive_style_system<T: Component<Mutability = Mutable> + Clone>(
    mut query: Query<(&Interaction, &InteractionStyle<T>, &mut T), Changed<Interaction>>,
) {
    for (interaction, style, mut component) in &mut query {
        *component = match *interaction {
            Interaction::Pressed => style
                .pressed
                .as_ref()
                .or(style.hovered.as_ref())
                .unwrap_or(&style.normal)
                .clone(),
            Interaction::Hovered => style.hovered.as_ref().unwrap_or(&style.normal).clone(),
            Interaction::None => style.normal.clone(),
        };
    }
}

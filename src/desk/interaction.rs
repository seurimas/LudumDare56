use crate::prelude::*;

use super::Contextable;

#[derive(Resource, Default)]
pub struct InteractState {
    pub mouse_location: Vec2,
    pub hovered: Option<Entity>,
    pub grabbed: Option<Entity>,
}

#[derive(Component, Clone, Debug)]
pub enum Interactable {
    Contextable(Contextable),
    ContextItem,
    Backdrop,
}

#[derive(Event, Debug)]
pub struct InteractEvent {
    pub entity: Entity,
    pub mouse_world_location: Vec2,
    pub interact_type: InteractType,
    pub interactable: Interactable,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InteractType {
    Hover,            // Nothing grabbed, mouse in.
    HoverWithGrabbed, // Hover, with a grabbed item.
    Unhover,          // Maybe grabbed, mouse out.
    Press,            // Mouse down, grab.
    DropOver,         // Hover, with a grabbed release.
}

impl Interactable {
    pub fn priority(&self) -> i32 {
        match self {
            Interactable::Contextable(_) => 50,
            Interactable::ContextItem => 0,
            Interactable::Backdrop => -100,
        }
    }

    pub fn in_range(&self, offset: Vec2) -> bool {
        match self {
            Interactable::Contextable(_) => offset.length() < 50.0,
            Interactable::ContextItem => offset.length() < 50.0,
            Interactable::Backdrop => true,
        }
    }
}

pub fn track_mouse(
    mut mouse_location: Local<Vec2>,
    mut interact_state: ResMut<InteractState>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    if camera.is_empty() {
        return;
    }
    let (camera, camera_transform) = camera.single();
    if let Some(cursor_moved_event) = cursor_moved_events.read().last() {
        *mouse_location = cursor_moved_event.position;
    }

    if let Some(mouse_world_location) = camera
        .viewport_to_world(camera_transform, *mouse_location)
        .map(|v| v.origin.truncate())
    {
        interact_state.mouse_location = mouse_world_location;
    }
}

pub fn interactable_system(
    mut interact_events: EventWriter<InteractEvent>,
    query: Query<(Entity, &GlobalTransform, &Interactable)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut interact_state: ResMut<InteractState>,
) {
    let mouse_world_location = interact_state.mouse_location;
    let mut best: Option<(Entity, &Interactable)> = None;
    for (entity, transform, interactable) in query.iter() {
        let offset = transform.translation().truncate() - mouse_world_location;
        if interactable.in_range(offset) {
            if let Some((_, best_interactable)) = best {
                if interactable.priority() > best_interactable.priority() {
                    best = Some((entity, interactable));
                }
            } else {
                best = Some((entity, interactable));
            }
        }
    }
    if let Some((entity, interactable)) = best {
        let old_hover = interact_state.hovered;
        let pressed = mouse_button_input.just_pressed(MouseButton::Left);
        let released = mouse_button_input.just_released(MouseButton::Left);
        if pressed {
            interact_events.send(InteractEvent {
                entity,
                mouse_world_location,
                interact_type: InteractType::Press,
                interactable: interactable.clone(),
            });
            interact_state.hovered = Some(entity);
            // interact_state.grabbed = Some(entity); // Set elsewhere, when appropriate.
        } else if released {
            interact_events.send(InteractEvent {
                entity,
                mouse_world_location,
                interact_type: InteractType::DropOver,
                interactable: interactable.clone(),
            });
            interact_state.hovered = None;
            interact_state.grabbed = None;
        } else {
            interact_state.hovered = Some(entity);
        }

        // Send hover events.
        if let Some(old_hover) = old_hover {
            if old_hover != entity {
                interact_events.send(InteractEvent {
                    entity: old_hover,
                    mouse_world_location,
                    interact_type: InteractType::Unhover,
                    interactable: interactable.clone(),
                });
                if interact_state.grabbed.is_some() {
                    interact_events.send(InteractEvent {
                        entity,
                        mouse_world_location,
                        interact_type: InteractType::HoverWithGrabbed,
                        interactable: interactable.clone(),
                    });
                } else {
                    interact_events.send(InteractEvent {
                        entity,
                        mouse_world_location,
                        interact_type: InteractType::Hover,
                        interactable: interactable.clone(),
                    });
                }
            }
        } else if interact_state.hovered.is_some() {
            interact_events.send(InteractEvent {
                entity,
                mouse_world_location,
                interact_type: InteractType::Hover,
                interactable: interactable.clone(),
            });
        }
    }
}

pub fn add_backdrop_interactable(mut commands: Commands) {
    commands.spawn((
        Transform::default(),
        GlobalTransform::default(),
        Interactable::Backdrop,
    ));
}

use crate::prelude::*;

use super::DeskItem;

pub const LEFT_TRACK: usize = 0;
pub const RIGHT_TRACK: usize = 1;
pub const MIDDLE_TRACK: usize = 2;

#[derive(Component)]
pub struct ContextMenu {
    pub referenced: Contextable,
    pub ready: bool,
    pub left_hovering: bool,
    pub right_hovering: bool,
    pub middle_hovering: bool,
}

#[derive(Clone, Debug)]
pub enum Contextable {
    Debug,
    Demon,
    DeskItem(DeskItem),
}

#[derive(Component)]
pub struct ContextItem {
    pub name: String,
}

pub fn spawn_debug_item(mut commands: Commands, game_assets: Res<GameAssets>) {
    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
    commands.spawn((
        SpriteBundle {
            texture: game_assets.debug_texture.clone(),
            transform,
            ..Default::default()
        },
        Interactable::Contextable(Contextable::Debug),
    ));
}

pub fn spawn_context_menu(
    mut interact_events: EventReader<InteractEvent>,
    mut commands: Commands,
    existing: Query<Entity, With<ContextMenu>>,
    game_assets: Res<GameAssets>,
    skeletons: Res<Skeletons>,
) {
    let clicked_backdrop = interact_events.read().find(|event| {
        event.interact_type == InteractType::Press
            && matches!(event.interactable, Interactable::Contextable(_))
    });
    if let Some(clicked_backdrop) = clicked_backdrop {
        for entity in existing.iter() {
            commands.entity(entity).despawn_recursive();
        }
        let mut transform =
            Transform::from_translation(clicked_backdrop.mouse_world_location.extend(0.0));
        transform.scale = Vec3::splat(0.5);
        commands.spawn((
            SpineBundle {
                transform,
                skeleton: skeletons.context.clone(),
                ..Default::default()
            },
            ContextMenu {
                referenced: if let Interactable::Contextable(contextable) =
                    clicked_backdrop.interactable.clone()
                {
                    contextable
                } else {
                    Contextable::Debug
                },
                ready: false,
                left_hovering: false,
                right_hovering: false,
                middle_hovering: false,
            },
        ));
    }
}

pub fn interact_menu(
    mut interact_events: EventReader<InteractEvent>,
    interact_state: Res<InteractState>,
    items: Query<(&ContextItem, &Parent)>,
    mut context: Query<(&mut ContextMenu, &mut Spine)>,
) {
    for event in interact_events.read() {
        if let Ok((item, parent)) = items.get(event.entity) {
            if let Ok((mut context, mut spine)) = context.get_mut(parent.get()) {
                match event.interact_type {
                    InteractType::Hover => {
                        if item.name == "left" {
                            context.left_hovering = true;
                            spine
                                .animation_state
                                .set_animation_by_name(LEFT_TRACK, "left_hover", true)
                                .expect("Failed to set animation");
                        } else if item.name == "right" {
                            context.right_hovering = true;
                            spine
                                .animation_state
                                .set_animation_by_name(RIGHT_TRACK, "right_hover", true)
                                .expect("Failed to set animation");
                        } else if item.name == "middle" {
                            context.middle_hovering = true;
                            spine
                                .animation_state
                                .set_animation_by_name(MIDDLE_TRACK, "middle_hover", true)
                                .expect("Failed to set animation");
                        } else if item.name == "back" {
                        }
                    }
                    InteractType::Unhover => {
                        if item.name == "left" {
                            context.left_hovering = false;
                            spine.animation_state.set_empty_animation(LEFT_TRACK, 0.);
                        } else if item.name == "right" {
                            context.right_hovering = false;
                            spine.animation_state.set_empty_animation(RIGHT_TRACK, 0.);
                        } else if item.name == "middle" {
                            context.middle_hovering = false;
                            spine.animation_state.set_empty_animation(MIDDLE_TRACK, 0.);
                        }
                    }
                    InteractType::Press => {
                        println!("Grabbed: {}", item.name);
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn initialize_menu(
    mut readies: EventReader<SpineReadyEvent>,
    mut query: Query<(&mut ContextMenu, &mut Spine)>,
    mut commands: Commands,
) {
    // Setup new menus.
    for event in readies.read() {
        if let Ok((mut context, mut spine)) = query.get_mut(event.entity) {
            println!("Initializing menu");
            context.ready = true;
            // Spin once, for flavor
            spine
                .animation_state
                .add_animation_by_name(LEFT_TRACK, "left_hover", false, 0.)
                .expect("Failed to add animation");
            spine
                .animation_state
                .add_animation_by_name(RIGHT_TRACK, "right_hover", false, 0.)
                .expect("Failed to add animation");
            spine
                .animation_state
                .add_animation_by_name(MIDDLE_TRACK, "middle_hover", false, 0.)
                .expect("Failed to add animation");

            // Spawn interaction entities
            let interactions = spine
                .skeleton
                .find_bone("interaction")
                .expect("No interactions bone");

            for (bone_name, bone) in event.bones.iter() {
                println!("Adding context item: {}", bone_name);
                let bone_in_skeleton = spine.skeleton.find_bone(bone_name);
                if bone_in_skeleton.is_none() {
                    continue;
                }
                let bone_in_skeleton = bone_in_skeleton.unwrap();
                if bone_in_skeleton.parent().is_none() {
                    continue;
                } else if bone_in_skeleton.parent().unwrap().data().name()
                    != interactions.data().name()
                {
                    continue;
                }
                commands
                    .entity(*bone)
                    .insert((
                        ContextItem {
                            name: bone_name.clone(),
                        },
                        Interactable::ContextItem,
                    ))
                    .set_parent(event.entity);
            }
        }
    }
}

pub fn debug_handle_events(
    mut spine_events: EventReader<SpineEvent>,
    mut context: Query<(&mut ContextMenu, &mut Spine)>,
) {
    for event in spine_events.read() {
        println!("{:?}", event);
    }
}

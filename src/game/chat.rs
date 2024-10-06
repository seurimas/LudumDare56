use bevy::text::Text2dBounds;

use crate::prelude::*;

#[derive(Component)]
pub struct ChatBox {
    pub style: &'static str,
    pub attachment: Entity,
    pub text_entity: Option<Entity>,
    pub text: String,
}

#[derive(Component, Clone)]
pub struct AttachedChatBox(pub Entity);

impl ChatBox {
    pub fn info(attachment: Entity, text: String) -> Self {
        Self {
            style: "info",
            attachment,
            text_entity: None,
            text,
        }
    }

    pub fn talk(attachment: Entity, text: String) -> Self {
        Self {
            style: "talk",
            attachment,
            text_entity: None,
            text,
        }
    }

    pub fn read(attachment: Entity, text: String) -> Self {
        Self {
            style: "read",
            attachment,
            text_entity: None,
            text,
        }
    }
}

fn despawn_old_chat_box(id: Entity, world: &mut World) {
    if let Some(AttachedChatBox(old)) = world.get::<AttachedChatBox>(id).cloned() {
        world.commands().add(DespawnRecursive { entity: old });
        world.get_mut::<AttachedChatBox>(id).unwrap().0 = id;
    }
}

pub fn spawn_chat_box(
    commands: &mut Commands,
    skeleton: Handle<SkeletonData>,
    chat_box: ChatBox,
) -> Entity {
    let parent = chat_box.attachment.clone();
    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 10.0));
    commands.entity(parent).add(despawn_old_chat_box);
    let new_chat = commands
        .spawn((
            SpineBundle {
                skeleton,
                transform,
                ..Default::default()
            },
            chat_box,
        ))
        .set_parent(parent)
        .id();
    commands.entity(parent).insert(AttachedChatBox(new_chat));
    new_chat
}

pub fn manage_chat_boxes(
    mut commands: Commands,
    globals: Query<&GlobalTransform>,
    mut chat_box: Query<(Entity, &mut ChatBox)>,
    mut ready_events: EventReader<SpineReadyEvent>,
    mut spine_events: EventReader<SpineEvent>,
    mut spine: Query<&mut Spine>,
) {
    for (entity, chat_box) in chat_box.iter() {
        if let Ok(_global) = globals.get(chat_box.attachment) {
        } else {
            if let Some(entity) = commands.get_entity(entity) {
                entity.despawn_recursive();
            }
        }
    }

    for event in spine_events.read() {
        if let SpineEvent::Event { entity, name, .. } = event {
            if let Ok((_entity, chat_box)) = chat_box.get(*entity) {
                if *name == "ShowText" {
                    if let Some(mut text_entity) =
                        chat_box.text_entity.and_then(|id| commands.get_entity(id))
                    {
                        text_entity.insert(Visibility::Visible);
                    }
                }
            }
        }
    }

    for event in ready_events.read() {
        if let Ok((entity, mut chat_box)) = chat_box.get_mut(event.entity) {
            if let Ok(mut spine) = spine.get_mut(event.entity) {
                spine
                    .skeleton
                    .set_skin_by_name(chat_box.style)
                    .expect("Failed to set skin");
                spine
                    .animation_state
                    .set_animation_by_name(0, "arrive", false)
                    .expect("Failed to set animation");
            }

            let text_attach = event.bones.get("text_attach").unwrap();

            if let Some(mut text_attach) = commands.get_entity(*text_attach) {
                text_attach.with_children(|parent| {
                    let text_entity = parent.spawn(Text2dBundle {
                        text: Text::from_section(
                            chat_box.text.clone(),
                            TextStyle {
                                color: Color::BLACK,
                                ..Default::default()
                            },
                        ),
                        text_2d_bounds: Text2dBounds {
                            size: Vec2::new(200.0, 100.0),
                        },
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                        visibility: Visibility::Hidden,
                        ..Default::default()
                    });
                    chat_box.text_entity = Some(text_entity.id());
                });
            } else {
                println!("Failed to find text_attach bone");
            }
        }
    }
}

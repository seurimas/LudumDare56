use serde::{Deserialize, Serialize};

use crate::{
    game::{spawn_demon, AttachedChatBox, DemonBrainDef, DemonDna},
    prelude::*,
};

use super::Desk;

const CANDLE_0_TRACK: usize = 0;
const CANDLE_1_TRACK: usize = 1;
const CANDLE_2_TRACK: usize = 2;
const CANDLE_3_TRACK: usize = 3;
const CANDLE_4_TRACK: usize = 4;
const JOURNAL_TRACK: usize = 5;
const ALEMBIC_TRACK: usize = 6;
const SUMMONING_TRACK: usize = 7;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeskItem {
    Alembic,
    Summoning,
    Journal,
    Potion,
    Candle(usize),
}

impl DeskItem {
    pub fn demon_can_use(&self) -> bool {
        match self {
            DeskItem::Alembic | DeskItem::Journal | DeskItem::Summoning => true,
            DeskItem::Potion | DeskItem::Candle(_) => false,
        }
    }
}

#[derive(Component, Default, Clone, Debug)]
pub struct DeskItemState {
    pub user: Option<Entity>,
    pub progress: f32,
    pub just_completed: Option<DemonDna>,
    pub completed: Vec<DemonDna>,
}

// Summoning
pub fn handle_summoning_context(
    mut commands: Commands,
    skeletons: Res<Skeletons>,
    mut context_events: EventReader<ContextAction>,
    mut desk: Query<&mut Spine, With<Desk>>,
    attached_chats: Query<&AttachedChatBox>,
) {
    let desk = desk.iter_mut().next();
    if desk.is_none() {
        return;
    }
    let mut desk = desk.unwrap();
    for event in context_events.read() {
        match event {
            ContextAction::PressLeft(Contextable::DeskItem(DeskItem::Summoning)) => {
                desk.skeleton
                    .find_bone_mut("candle0")
                    .unwrap()
                    .set_scale(Vec2::new(1., 1.));
                desk.skeleton
                    .find_bone_mut("candle1")
                    .unwrap()
                    .set_scale(Vec2::new(1., 1.));
            }
            ContextAction::PressRight(Contextable::DeskItem(DeskItem::Summoning)) => {
                desk.skeleton
                    .find_bone_mut("candle2")
                    .unwrap()
                    .set_scale(Vec2::new(1., 1.));
                desk.skeleton
                    .find_bone_mut("candle3")
                    .unwrap()
                    .set_scale(Vec2::new(1., 1.));
            }
            ContextAction::PressMiddle(Contextable::DeskItem(DeskItem::Summoning)) => {
                desk.skeleton
                    .find_bone_mut("candle4")
                    .unwrap()
                    .set_scale(Vec2::new(1., 1.));
            }
            ContextAction::HoverLeft(chat_attach, Contextable::DeskItem(DeskItem::Summoning))
            | ContextAction::HoverMiddle(chat_attach, Contextable::DeskItem(DeskItem::Summoning))
            | ContextAction::HoverRight(chat_attach, Contextable::DeskItem(DeskItem::Summoning)) => {
                spawn_chat_box(
                    &mut commands,
                    skeletons.chat.clone(),
                    ChatBox::info(
                        *chat_attach,
                        "Light the candles to summon a demon.".to_string(),
                    ),
                );
            }
            ContextAction::Unhover(chat_attach) => {
                if let Ok(attached_chat) = attached_chats.get(*chat_attach) {
                    commands.entity(attached_chat.0).despawn_recursive();
                }
            }
            _ => continue,
        }
    }
}

pub fn trigger_summoning(
    mut commands: Commands,
    mut animation_events: EventReader<SpineEvent>,
    mut desk: Query<&mut Spine, With<Desk>>,
    items: Query<(&DeskItem, &SpineBone, &Transform)>,
    skeletons: Res<Skeletons>,
    game_assets: Res<GameAssets>,
    brains: Res<Assets<DemonBrainDef>>,
) {
    {
        let desk = desk.iter_mut().next();
        if desk.is_none() {
            return;
        }
        let mut desk = desk.unwrap();
        let mut lit_count = 0;
        ["candle0", "candle1", "candle2", "candle3", "candle4"]
            .iter()
            .for_each(|bone_name| {
                if let Some(bone) = desk.skeleton.find_bone(bone_name) {
                    if bone.scale_x() == 1.0 {
                        lit_count += 1;
                    }
                }
            });
        if lit_count == 5 {
            if desk
                .animation_state
                .get_current(SUMMONING_TRACK)
                .map(|current| {
                    if current.animation().name().contains("empty") {
                        false
                    } else {
                        true
                    }
                })
                .unwrap_or(false)
            {
                // Already playing
            } else {
                desk.animation_state
                    .set_animation_by_name(SUMMONING_TRACK, "summon", false)
                    .expect("Failed to set animation");
                desk.animation_state
                    .add_empty_animation(SUMMONING_TRACK, 0.0, 0.0);
            }
        }
    }
    for event in animation_events.read() {
        if let SpineEvent::Event { entity, name, .. } = event {
            if let Ok(mut desk) = desk.get_mut(*entity) {
                if *name == "Summon" {
                    if let Some(location) = items.iter().find_map(|(item, _, transform)| {
                        if let DeskItem::Summoning = item {
                            Some(transform.translation)
                        } else {
                            None
                        }
                    }) {
                        println!("Summoning demon");
                        spawn_demon(
                            &mut commands,
                            &game_assets,
                            skeletons.demon.clone(),
                            location.truncate(),
                            None,
                            &brains,
                        );
                        desk.skeleton
                            .find_bone_mut("candle0")
                            .unwrap()
                            .set_scale(Vec2::new(0., 0.));
                        desk.skeleton
                            .find_bone_mut("candle1")
                            .unwrap()
                            .set_scale(Vec2::new(0., 0.));
                        desk.skeleton
                            .find_bone_mut("candle2")
                            .unwrap()
                            .set_scale(Vec2::new(0., 0.));
                        desk.skeleton
                            .find_bone_mut("candle3")
                            .unwrap()
                            .set_scale(Vec2::new(0., 0.));
                        desk.skeleton
                            .find_bone_mut("candle4")
                            .unwrap()
                            .set_scale(Vec2::new(0., 0.));
                    } else {
                        panic!("Failed to find summoning location");
                    }
                }
            }
        }
    }
}

// Alembic
pub fn trigger_alembic(
    mut desk: Query<&mut Spine, With<Desk>>,
    mut items: Query<(&DeskItem, &mut DeskItemState)>,
) {
    let desk = desk.iter_mut().next();
    if desk.is_none() {
        return;
    }
    let mut desk = desk.unwrap();
    if let Some((alembic, mut state)) = items
        .iter_mut()
        .find(|(item, _)| matches!(item, DeskItem::Alembic))
    {
        {
            if let Some(dna) = state.just_completed {
                println!("Alembic just completed: {:?}", dna);
                state.completed.push(dna);
                state.just_completed = None;
                state.progress = 0.0;
                state.user = None;
            }
        }
    }
}

// Alembic
pub fn trigger_journal(
    mut desk: Query<&mut Spine, With<Desk>>,
    mut items: Query<(&DeskItem, &mut DeskItemState)>,
) {
    let desk = desk.iter_mut().next();
    if desk.is_none() {
        return;
    }
    let mut desk = desk.unwrap();
    if let Some((alembic, mut state)) = items
        .iter_mut()
        .find(|(item, _)| matches!(item, DeskItem::Journal))
    {
        {
            if let Some(dna) = state.just_completed {
                println!("Journal just completed: {:?}", dna);
                state.completed.push(dna);
                state.just_completed = None;
                state.progress = 0.0;
                state.user = None;
            }
        }
    }
}

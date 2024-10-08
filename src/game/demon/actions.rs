use behavior_bark::unpowered::UnpoweredFunctionState;

use crate::{
    game::{
        get_berate, get_complain, get_interrupted, spawn_demon_chat_box, spawn_main_chat_box,
        DeskItemState, MainChat, MainChatAttach,
    },
    prelude::*,
};

use super::{get_introduction, DemonBrain, DemonModel, Distraction};

pub const DEMON_MAIN_TRACK: usize = 0;

pub fn activate_demons(
    time: Res<Time>,
    mut desk: Query<&mut Spine, With<Desk>>,
    mut query: Query<
        (Entity, &Transform, &mut Demon, &mut Spine),
        (Without<DeskItem>, Without<Desk>),
    >,
    mut desk_items: Query<(&Transform, &DeskItem, &mut DeskItemState), Without<Demon>>,
    mut velocities: Query<&mut Velocity>,
    mut commands: Commands,
    main_chat: Query<(Entity, &MainChatAttach)>,
    skeletons: Res<Skeletons>,
) {
    let desk = desk.iter_mut().next();
    if desk.is_none() {
        return;
    }
    let desk = desk.unwrap();
    for (entity, transform, mut demon, mut spine) in query.iter_mut() {
        if demon.chat_attach.is_none() {
            // Not initialized yet!
            continue;
        }
        match &demon.action {
            DemonController::Introduce => {
                if spine
                    .animation_state
                    .get_current(DEMON_MAIN_TRACK)
                    .map(|current| current.animation().name().contains("introduce"))
                    .unwrap_or(false)
                {
                    // Already idle
                } else {
                    spine
                        .animation_state
                        .set_animation_by_name(DEMON_MAIN_TRACK, "introduce", true)
                        .expect("Failed to set animation");
                    let introduction = get_introduction(&demon.dna);
                    spawn_demon_chat_box(
                        &mut commands,
                        &mut demon,
                        &skeletons,
                        introduction,
                        "introduce",
                    );
                }
            }
            DemonController::MoveTo(target) => {
                if spine
                    .animation_state
                    .get_current(DEMON_MAIN_TRACK)
                    .map(|current| current.animation().name().contains("walk"))
                    .unwrap_or(false)
                {
                    // Already walking
                    continue;
                } else {
                    spine
                        .animation_state
                        .set_animation_by_name(DEMON_MAIN_TRACK, "walk", false)
                        .expect("Failed to set animation");
                    spine
                        .animation_state
                        .add_empty_animation(DEMON_MAIN_TRACK, 0., 0.);
                }
                let desk_item = desk_items.iter().find(|(_, item, _)| *item == target);
                if let Some((target_transform, _, _)) = desk_item {
                    let direction = target_transform.translation - transform.translation;
                    let direction = direction.truncate();
                    let mut velocity = velocities.get_mut(entity).unwrap();
                    velocity.linvel = direction.normalize() * 100.0;
                    if direction.x < 0.0 {
                        let scale = spine.skeleton.scale_x().abs();
                        spine.skeleton.set_scale_x(-scale);
                    } else {
                        let scale = spine.skeleton.scale_x().abs();
                        spine.skeleton.set_scale_x(scale);
                    }
                }
            }
            DemonController::UseTool => {
                if let Some((_, item, mut state)) = desk_items
                    .iter_mut()
                    .find(|(_, item, _)| Some(**item) == demon.in_area_for_tool)
                {
                    if state.user == Some(entity) {
                        // Already using tool
                        state.progress += time.delta_seconds();
                        continue;
                    } else if state.user.is_some() {
                        // Someone else is using the tool
                        demon.action = DemonController::Distracted(
                            Distraction::Berate,
                            Box::new(demon.action.clone()),
                        );
                        continue;
                    } else {
                        state.user = Some(entity);
                        state.progress = 0.0;
                    }
                }
                let mut velocity = velocities.get_mut(entity).unwrap();
                velocity.linvel = Vec2::ZERO;
            }
            DemonController::Distracted(something, _next) => {
                println!("Demon distracted: {:?}", something);
                match something {
                    Distraction::Sleep => {
                        if spine
                            .animation_state
                            .get_current(DEMON_MAIN_TRACK)
                            .map(|current| current.animation().name().contains("sleep"))
                            .unwrap_or(false)
                        {
                            // Already sleeping
                        } else {
                            spine
                                .animation_state
                                .set_animation_by_name(DEMON_MAIN_TRACK, "sleep", false)
                                .expect("Failed to set animation");
                            spine
                                .animation_state
                                .add_animation_by_name(DEMON_MAIN_TRACK, "sleep_loop", true, 0.)
                                .expect("Cannot add sleep loop");
                            let mut velocity = velocities.get_mut(entity).unwrap();
                            velocity.linvel = Vec2::ZERO;
                        }
                    }
                    Distraction::Wander => {
                        if spine
                            .animation_state
                            .get_current(DEMON_MAIN_TRACK)
                            .map(|current| current.animation().name().contains("walk"))
                            .unwrap_or(false)
                        {
                            // Already walking
                        } else {
                            spine
                                .animation_state
                                .set_animation_by_name(DEMON_MAIN_TRACK, "walk", false)
                                .expect("Failed to set animation");
                            spine
                                .animation_state
                                .add_empty_animation(DEMON_MAIN_TRACK, 0., 0.);
                            let direction =
                                Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5);
                            let mut velocity = velocities.get_mut(entity).unwrap();
                            velocity.linvel = direction.normalize() * 50.0;
                            if direction.x < 0.0 {
                                let scale = spine.skeleton.scale_x().abs();
                                spine.skeleton.set_scale_x(-scale);
                            } else {
                                let scale = spine.skeleton.scale_x().abs();
                                spine.skeleton.set_scale_x(scale);
                            }
                        }
                    }
                    Distraction::Complain | Distraction::Berate | Distraction::Interrupted => {
                        if spine
                            .animation_state
                            .get_current(DEMON_MAIN_TRACK)
                            .map(|current| current.animation().name().contains("complain"))
                            .unwrap_or(false)
                        {
                            // Already complaining
                        } else {
                            spine
                                .animation_state
                                .set_animation_by_name(DEMON_MAIN_TRACK, "complain", true)
                                .expect("Failed to set animation");
                            let mut velocity = velocities.get_mut(entity).unwrap();
                            velocity.linvel = Vec2::ZERO;
                            let text = if something == &Distraction::Berate {
                                get_berate(&demon.dna)
                            } else if something == &Distraction::Complain {
                                get_complain(&demon.dna)
                            } else {
                                get_interrupted(&demon.dna)
                            };
                            spawn_demon_chat_box(
                                &mut commands,
                                &mut demon,
                                &skeletons,
                                text,
                                "complain",
                            );
                        }
                    }
                    Distraction::Annoyed => {
                        if spine
                            .animation_state
                            .get_current(DEMON_MAIN_TRACK)
                            .map(|current| current.animation().name().contains("hit"))
                            .unwrap_or(false)
                        {
                            // Already annoyed
                        } else {
                            spine
                                .animation_state
                                .set_animation_by_name(DEMON_MAIN_TRACK, "hit", false)
                                .expect("Failed to set animation");
                            spine
                                .animation_state
                                .add_empty_animation(DEMON_MAIN_TRACK, 0., 0.);
                            let mut velocity = velocities.get_mut(entity).unwrap();
                            velocity.linvel = Vec2::ZERO;
                        }
                    }
                }
            }
            DemonController::FinishJob => {
                for (_, _, mut state) in desk_items.iter_mut() {
                    if state.user == Some(entity) {
                        state.just_completed = Some(demon.dna.clone());
                    }
                }
            }
            DemonController::Idle => {}
        }
    }
}

pub fn mark_demons_in_area(
    mut collision_events: EventReader<CollisionEvent>,
    mut demons: Query<&mut Demon>,
    desk_items: Query<&DeskItem>,
) {
    for event in collision_events.read() {
        println!("Collision event: {:?}", event);
        if let CollisionEvent::Started(a, b, _) = event {
            if let Ok(mut demon) = demons.get_mut(*a) {
                if let Ok(desk_item) = desk_items.get(*b) {
                    println!("Demon entered area for tool");
                    demon.in_area_for_tool = Some(*desk_item);
                }
            } else if let Ok(mut demon) = demons.get_mut(*b) {
                if let Ok(desk_item) = desk_items.get(*a) {
                    println!("Demon entered area for tool");
                    demon.in_area_for_tool = Some(*desk_item);
                }
            }
        } else if let CollisionEvent::Stopped(a, b, _) = event {
            if let Ok(mut demon) = demons.get_mut(*a) {
                if let Ok(desk_item) = desk_items.get(*b) {
                    println!("Demon left area for tool");
                    if demon.in_area_for_tool == Some(*desk_item) {
                        demon.in_area_for_tool = None;
                    }
                }
            } else if let Ok(mut demon) = demons.get_mut(*b) {
                if let Ok(desk_item) = desk_items.get(*a) {
                    println!("Demon left area for tool");
                    if demon.in_area_for_tool == Some(*desk_item) {
                        demon.in_area_for_tool = None;
                    }
                }
            }
        }
    }
}

pub fn mark_closest_item(
    mut demons: Query<(&Transform, &mut Demon), Without<DeskItem>>,
    desk_items: Query<(&Transform, &DeskItem), Without<Demon>>,
) {
    for (demon_transform, mut demon) in demons.iter_mut() {
        let mut closest_distance = f32::MAX;
        let mut closest_item = None;
        for (item_transform, item) in desk_items.iter() {
            if !item.demon_can_use() {
                continue;
            }
            let distance = demon_transform
                .translation
                .distance(item_transform.translation);
            if distance < closest_distance {
                closest_distance = distance;
                closest_item = Some(*item);
            }
        }
        if let Some(item) = closest_item {
            demon.nearest_tool = item;
        }
    }
}

pub fn control_demons(
    mut query: Query<(Entity, &mut Demon, &mut DemonBrain)>,
    tools: Query<(&DeskItem, &DeskItemState)>,
) {
    for (entity, mut demon, mut brains) in query.iter_mut() {
        let using_tool = tools.iter().find_map(|(item, state)| {
            if state.user == Some(entity) {
                Some(state.progress)
            } else {
                None
            }
        });
        let model = DemonModel {
            dna: demon.dna,
            nonce: demon.nonce,
            in_range_of_tool: demon.in_area_for_tool.is_some(),
            nearest_tool: demon.nearest_tool,
            using_tool,
        };
        if matches!(demon.action, DemonController::Distracted(_, _))
            || matches!(demon.action, DemonController::Introduce)
        {
            continue;
        }
        let result = brains.0.resume_with(&model, &mut demon.action);
        match result {
            UnpoweredFunctionState::Complete => {
                demon.nonce += 1;
            }
            UnpoweredFunctionState::Failed => {
                demon.nonce += 1;
            }
            UnpoweredFunctionState::Waiting => {}
        }
    }
}

pub fn untask_demons(
    mut query: Query<(&mut Demon, &mut Spine)>,
    mut events: EventReader<SpineEvent>,
) {
    for event in events.read() {
        if let SpineEvent::Complete {
            entity, animation, ..
        } = event
        {
            if let Ok((mut demon, mut spine)) = query.get_mut(*entity) {
                match animation.as_str() {
                    "hit" => {
                        if let DemonController::Distracted(Distraction::Annoyed, real_task) =
                            &demon.action
                        {
                            demon.action = *real_task.clone();
                        }
                    }
                    "walk" => {
                        demon.action = DemonController::Idle;
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn bother_demons(
    mut query: Query<(&mut Demon, &mut Spine)>,
    mut events: EventReader<InteractEvent>,
) {
    for event in events.read() {
        if let InteractEvent {
            interact_type: InteractType::Press,
            interactable: Interactable::Demon,
            entity,
            ..
        } = event
        {
            println!("Bothering demon");
            if let Ok((mut demon, mut spine)) = query.get_mut(*entity) {
                match &demon.action {
                    DemonController::Distracted(distraction, real_task) => {
                        if distraction != &Distraction::Annoyed {
                            demon.action = DemonController::Distracted(
                                Distraction::Annoyed,
                                real_task.clone(),
                            );
                        }
                    }
                    DemonController::Introduce => {
                        demon.action = DemonController::Distracted(
                            Distraction::Annoyed,
                            Box::new(DemonController::Idle),
                        );
                    }
                    _ => {
                        demon.action = DemonController::Distracted(
                            Distraction::Annoyed,
                            Box::new(DemonController::Distracted(
                                Distraction::Interrupted,
                                Box::new(demon.action.clone()),
                            )),
                        );
                    }
                }
            }
        }
    }
}

use crate::{
    game::{Contextable, DeskItemState},
    prelude::*,
};

pub const BASE_DESK_WIDTH: f32 = 512.0;
pub const BASE_DESK_HEIGHT: f32 = 512.0;

#[derive(Component)]
pub struct Desk {
    pub width: f32,
    pub height: f32,
    pub boundaries: Option<()>,
}

pub fn spawn_desk(mut commands: Commands, game_assets: Res<GameAssets>, skeletons: Res<Skeletons>) {
    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, -1.0));
    commands.spawn((
        SpineBundle {
            skeleton: skeletons.desk.clone(),
            transform,
            ..Default::default()
        },
        Desk {
            width: BASE_DESK_WIDTH,
            height: BASE_DESK_HEIGHT,
            boundaries: None,
        },
    ));
}

fn get_polygon_for_bounding_box(spine: &Spine, slot: &'static str) -> Collider {
    let slot = spine
        .skeleton
        .find_slot(slot)
        .expect("Missing boundaries slot");
    let box_attachment = slot
        .bounding_box_attachment()
        .expect("Missing boundaries box");
    let vertices: Vec<Vec2> = box_attachment
        .vertices2()
        .iter()
        .map(|v| Vec2::new(v.x, v.y))
        .collect();
    Collider::convex_hull(vertices.as_slice()).unwrap()
}

fn get_polyline_from_boundaries(spine: &Spine) -> Collider {
    let boundaries_slot = spine
        .skeleton
        .find_slot("boundaries")
        .expect("Missing boundaries slot");
    let boundaries_box = boundaries_slot
        .bounding_box_attachment()
        .expect("Missing boundaries box");
    let mut vertices: Vec<Vec2> = boundaries_box
        .vertices2()
        .iter()
        .map(|v| Vec2::new(v.x, v.y))
        .collect();
    vertices.push(vertices[0]);
    Collider::polyline(vertices, None)
}

pub fn initialize_desk(
    mut readies: EventReader<SpineReadyEvent>,
    mut query: Query<(&mut Desk, &mut Spine)>,
    mut commands: Commands,
) {
    // Setup new menus.
    for event in readies.read() {
        if let Ok((mut desk, mut spine)) = query.get_mut(event.entity) {
            println!("Initializing backdrop");

            commands
                .entity(event.entity)
                .insert((get_polyline_from_boundaries(&spine), RigidBody::Fixed));

            for bone_name in [
                "candle0", "candle1", "candle2", "candle3", "candle4", "water", "line0", "line1",
                "line2", "line3", "line4",
            ]
            .iter()
            {
                let mut bone = spine.skeleton.find_bone_mut(*bone_name).unwrap();
                bone.set_scale(Vec2::new(0., 0.));
            }

            for (bone_name, bone) in event.bones.iter() {
                println!("Adding context item: {}", bone_name);
                let (item, bounding_slot) = match bone_name.as_str() {
                    "alembic" => (DeskItem::Alembic, Some("alembic_interact")),
                    "summoning" => (DeskItem::Summoning, Some("summoning_interact")),
                    "journal" => (DeskItem::Journal, Some("journal_interact")),
                    "potion" => (DeskItem::Potion, None),
                    "candle0" => (DeskItem::Candle(0), None),
                    "candle1" => (DeskItem::Candle(1), None),
                    "candle2" => (DeskItem::Candle(2), None),
                    "candle3" => (DeskItem::Candle(3), None),
                    "candle4" => (DeskItem::Candle(4), None),
                    _ => continue,
                };
                let interactable = match item {
                    DeskItem::Candle(idx) => Interactable::Candle(idx),
                    DeskItem::Potion => Interactable::Potion,
                    DeskItem::Journal => Interactable::Journal,
                    // item => Interactable::Contextable(Contextable::DeskItem(item)),
                    _ => Interactable::Backdrop,
                };
                commands
                    .entity(*bone)
                    .insert((item, interactable, DeskItemState::default()));
                if let Some(bounding_slot) = bounding_slot {
                    let collider = get_polygon_for_bounding_box(&spine, bounding_slot);
                    commands.entity(*bone).insert((
                        Sensor,
                        ActiveEvents::COLLISION_EVENTS,
                        collider,
                    ));
                }
            }
        }
    }
}

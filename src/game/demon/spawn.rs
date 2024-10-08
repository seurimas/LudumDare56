use imp_encode::CursedConfig;

use crate::prelude::*;

use super::{get_name, get_skins, random_genes, DemonBrain, DemonBrainDef};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DemonDna(pub [u8; 16]);

#[derive(Component)]
pub struct Demon {
    pub dna: DemonDna,
    pub nonce: u32,
    pub action: DemonController,
    pub in_area_for_tool: Option<DeskItem>,
    pub nearest_tool: DeskItem,
    pub chatting: Option<(&'static str)>,
    pub chat_attach: Option<Entity>,
}

impl Demon {
    pub fn from_dna(dna: DemonDna) -> Self {
        Self {
            dna,
            nonce: 0,
            action: DemonController::Introduce,
            in_area_for_tool: None,
            nearest_tool: DeskItem::Summoning,
            chatting: None,
            chat_attach: None,
        }
    }
}

pub fn spawn_demon(
    commands: &mut Commands,
    game_assets: &GameAssets,
    skeleton: Handle<SkeletonData>,
    position: Vec2,
    dna: Option<DemonDna>,
    brains: &Assets<DemonBrainDef>,
) {
    let brains = brains.get(&game_assets.demon_brain).unwrap().create_tree();
    let mut transform = Transform::from_translation(Vec3::new(position.x, position.y, 0.0));
    transform.scale = Vec3::splat(0.1);
    let dna = dna.unwrap_or_else(random_genes);
    commands.spawn((
        SpineBundle {
            skeleton,
            transform,
            ..Default::default()
        },
        RigidBody::Dynamic,
        Restitution::coefficient(0.1),
        Collider::ball(100.),
        Velocity {
            linvel: Vec2::new(0.0, 0.0),
            ..Default::default()
        },
        LockedAxes::ROTATION_LOCKED,
        GravityScale(0.0),
        Demon::from_dna(dna),
        DemonBrain(brains),
        Interactable::Demon,
    ));
}

pub fn initialize_demon(
    mut readies: EventReader<SpineReadyEvent>,
    mut query: Query<(&mut Demon, &mut Spine)>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    for event in readies.read() {
        if let Ok((mut demon, mut spine)) = query.get_mut(event.entity) {
            let skins = get_skins(&demon.dna);
            println!("Initializing demon with skins: {:?}", skins);
            let conglomerate_skin = skins.join("-");
            spine
                .skeleton
                .set_skins_by_name(&conglomerate_skin, skins)
                .expect("Failed to set skin");
            if let Some(bone_entity) = event.bones.get("chat") {
                demon.chat_attach = Some(*bone_entity);
            }
        }
    }
}

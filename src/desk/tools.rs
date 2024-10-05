use crate::{desk::Contextable, prelude::*};

pub const BASE_DESK_WIDTH: f32 = 512.0;
pub const BASE_DESK_HEIGHT: f32 = 512.0;

#[derive(Component)]
pub struct Desk {
    pub width: f32,
    pub height: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum DeskItem {
    Alembic,
    Summoning,
    Journal,
    Candle(usize),
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
        },
    ));
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
            for (bone_name, bone) in event.bones.iter() {
                println!("Adding context item: {}", bone_name);
                let item = match bone_name.as_str() {
                    "alembic" => DeskItem::Alembic,
                    "summoning" => DeskItem::Summoning,
                    "journal" => DeskItem::Journal,
                    "candle0" => DeskItem::Candle(0),
                    "candle1" => DeskItem::Candle(1),
                    "candle2" => DeskItem::Candle(2),
                    "candle3" => DeskItem::Candle(3),
                    "candle4" => DeskItem::Candle(4),
                    _ => continue,
                };
                let interactable = match item {
                    DeskItem::Candle(idx) => Interactable::Candle(idx),
                    item => Interactable::Contextable(Contextable::DeskItem(item)),
                };
                commands.entity(*bone).insert((item, interactable));
            }
        }
    }
}

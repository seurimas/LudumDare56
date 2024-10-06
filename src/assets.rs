use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
};
use bevy_spine::{Atlas, SkeletonData, SkeletonJson};

use crate::{game::demon::DemonBrainDef, prelude::*};

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(Loading)
                .continue_to_state(Playing)
                .load_collection::<GameAssets>(),
        )
        .add_plugins(bevy_common_assets::ron::RonAssetPlugin::<DemonBrainDef>::new(&["brain"]))
        .add_systems(OnExit(Loading), create_skeletons);
    }
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "demon.brain")]
    pub demon_brain: Handle<DemonBrainDef>,
    #[asset(path = "Debug.png")]
    pub debug_texture: Handle<Image>,
    #[asset(path = "spines/context.atlas")]
    pub context_atlas: Handle<Atlas>,
    #[asset(path = "spines/context.json")]
    pub context_json: Handle<SkeletonJson>,
    #[asset(path = "spines/desk.atlas")]
    pub desk_atlas: Handle<Atlas>,
    #[asset(path = "spines/desk.json")]
    pub desk_json: Handle<SkeletonJson>,
    #[asset(path = "spines/demon.atlas")]
    pub demon_atlas: Handle<Atlas>,
    #[asset(path = "spines/demon.json")]
    pub demon_json: Handle<SkeletonJson>,
    #[asset(path = "spines/chat.atlas")]
    pub chat_atlas: Handle<Atlas>,
    #[asset(path = "spines/chat.json")]
    pub chat_json: Handle<SkeletonJson>,
}

#[derive(Resource)]
pub struct Skeletons {
    pub context: Handle<SkeletonData>,
    pub desk: Handle<SkeletonData>,
    pub demon: Handle<SkeletonData>,
    pub chat: Handle<SkeletonData>,
}

fn create_skeletons(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut skeletons: ResMut<Assets<SkeletonData>>,
) {
    let context_skeleton =
        SkeletonData::new_from_json(assets.context_json.clone(), assets.context_atlas.clone());
    let context = skeletons.add(context_skeleton);

    let desk_skeleton =
        SkeletonData::new_from_json(assets.desk_json.clone(), assets.desk_atlas.clone());
    let desk = skeletons.add(desk_skeleton);

    let demon_skeleton =
        SkeletonData::new_from_json(assets.demon_json.clone(), assets.demon_atlas.clone());
    let demon = skeletons.add(demon_skeleton);

    let chat_skeleton =
        SkeletonData::new_from_json(assets.chat_json.clone(), assets.chat_atlas.clone());
    let chat = skeletons.add(chat_skeleton);

    commands.insert_resource(Skeletons {
        context,
        desk,
        demon,
        chat,
    });
}

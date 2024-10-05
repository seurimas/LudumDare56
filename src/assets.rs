use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
};
use bevy_spine::{Atlas, SkeletonData, SkeletonJson};

use crate::prelude::*;

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(Loading)
                .continue_to_state(Playing)
                .load_collection::<GameAssets>(),
        )
        .add_systems(OnExit(Loading), create_skeletons);
    }
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "spines/context.atlas")]
    pub context_atlas: Handle<Atlas>,
    #[asset(path = "spines/context.json")]
    pub context_json: Handle<SkeletonJson>,
}

#[derive(Resource)]
pub struct Skeletons {
    pub context: Handle<SkeletonData>,
}

fn create_skeletons(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut skeletons: ResMut<Assets<SkeletonData>>,
) {
    let context_skeleton =
        SkeletonData::new_from_json(assets.context_json.clone(), assets.context_atlas.clone());
    let context = skeletons.add(context_skeleton);

    commands.insert_resource(Skeletons { context });
}

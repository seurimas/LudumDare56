mod assets;
mod game;
mod prelude;
mod state;

use bevy::window::WindowResolution;
use bevy_spine::SpinePlugin;
use game::chat::MainChatAttach;

use crate::assets::GameAssetsPlugin;
use crate::game::DeskPlugin;
use crate::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Demons On My Desk".to_string(),
                resolution: WindowResolution::new(948., 533.),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
        .init_state::<GameState>()
        .add_plugins((SpinePlugin, GameAssetsPlugin, DeskPlugin))
        .add_systems(OnEnter(Playing), spawn_camera)
        .add_systems(OnExit(Playing), despawn_camera)
        .run()
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .with_children(|parent| {
            parent.spawn((TransformBundle::default(), MainChatAttach));
        });
}

fn despawn_camera(mut commands: Commands, query: Query<Entity, With<Camera2d>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

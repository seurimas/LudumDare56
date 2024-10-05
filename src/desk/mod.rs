pub mod context;
pub mod interaction;
pub mod tools;

use context::*;
use interaction::*;
use tools::*;

use crate::prelude::*;

pub struct DeskPlugin;

impl Plugin for DeskPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractEvent>()
            .init_resource::<InteractState>()
            .add_systems(
                OnEnter(Playing),
                (add_backdrop_interactable, spawn_debug_item, spawn_desk),
            )
            .add_systems(
                Update,
                (
                    spawn_context_menu,
                    initialize_menu,
                    interact_menu,
                    initialize_desk,
                    debug_handle_events,
                )
                    .run_if(in_state(Playing)),
            )
            .add_systems(
                Update,
                (track_mouse, interactable_system).run_if(in_state(Playing)),
            );
    }
}

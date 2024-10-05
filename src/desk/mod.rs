pub mod context;
pub mod interaction;

use context::*;
use interaction::{add_backdrop_interactable, interactable_system, track_mouse, InteractEvent};

use crate::prelude::*;

pub struct DeskPlugin;

impl Plugin for DeskPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractEvent>()
            .init_resource::<InteractState>()
            .add_systems(OnEnter(Playing), add_backdrop_interactable)
            .add_systems(
                Update,
                (
                    spawn_debug_menu,
                    initialize_menu,
                    interact_menu,
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

pub mod backdrop;
pub mod chat;
pub mod demon;
pub mod input;

use crate::prelude::*;

use backdrop::*;
use chat::*;
use demon::*;
use input::*;

pub struct DeskPlugin;

impl Plugin for DeskPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractEvent>()
            .add_event::<ContextAction>()
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
                    handle_summoning_context,
                    trigger_summoning,
                    manage_chat_boxes,
                    light_candle,
                )
                    .run_if(in_state(Playing)),
            )
            .add_systems(
                Update,
                (
                    initialize_demon,
                    mark_closest_item,
                    mark_demons_in_area,
                    control_demons,
                    activate_demons,
                    bother_demons,
                    untask_demons,
                    trigger_alembic,
                    drink_potion,
                    trigger_journal,
                    despawn_demon_chat_boxes,
                    read_page,
                )
                    .run_if(in_state(Playing)),
            )
            .add_systems(
                Update,
                (track_mouse, interactable_system).run_if(in_state(Playing)),
            );
    }
}

pub use crate::assets::{GameAssets, Skeletons};
pub use crate::game::backdrop::Desk;
pub use crate::game::backdrop::DeskItem;
pub use crate::game::chat::{spawn_chat_box, ChatBox, MainChatAttach};
pub use crate::game::demon::{Demon, DemonController};
pub use crate::game::input::*;
pub use crate::state::GameState;
pub use crate::state::GameState::*;
pub use bevy::prelude::*;
pub use bevy_rapier2d::prelude::*;
pub use bevy_spine::prelude::*;

pub fn get_current_animation(spine: &Spine, animation_track: usize) -> Option<String> {
    spine
        .animation_state
        .get_current(animation_track)
        .map(|current| current.animation().name().to_string())
}

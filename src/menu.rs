use crate::loading::FontAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(go));
    }
}

/// placeholder
fn go(
    mut state: ResMut<State<GameState>>
) {
    state.set(GameState::Playing).unwrap();
}

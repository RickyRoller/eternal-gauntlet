use crate::state::GameState;
use bevy::prelude::*;

pub struct UpgradeMenu;

impl Plugin for UpgradeMenu {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (pause_game).run_if(in_state(GameState::UpgradeMenu)),
        );
    }
}

fn pause_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::UpgradeMenu);
    println!("UpgradeMenu");
}

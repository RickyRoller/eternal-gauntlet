use crate::enemy::{EnemiesDataHandle, EnemiesSpawnDataHandle};
use crate::enemy_textures::EnemyTextureAtlasHandle;
use crate::hit_textures::HitTextureAtlasHandle;
use crate::state::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (check_assets).run_if(in_state(GameState::Loading)));
    }
}

fn check_assets(
    asset_server: Res<AssetServer>,
    enemy_texture_atlas_handle: Res<EnemyTextureAtlasHandle>,
    hit_texture_atlas_handle: Res<HitTextureAtlasHandle>,
    enemies_data_handle: Res<EnemiesDataHandle>,
    enemies_spawn_data_handle: Res<EnemiesSpawnDataHandle>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let enemy_loaded =
        asset_server.get_load_state(enemy_texture_atlas_handle.0.id()) == Some(LoadState::Loaded);
    let hit_loaded =
        asset_server.get_load_state(hit_texture_atlas_handle.0.id()) == Some(LoadState::Loaded);
    let enemies_loaded =
        asset_server.get_load_state(enemies_data_handle.0.id()) == Some(LoadState::Loaded);
    let enemies_spawn_loaded =
        asset_server.get_load_state(enemies_spawn_data_handle.0.id()) == Some(LoadState::Loaded);

    if enemy_loaded && hit_loaded && enemies_loaded && enemies_spawn_loaded {
        next_state.set(GameState::MainMenu);
    }
}

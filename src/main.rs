use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::close_on_esc;
use log::info;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

use bevy_common_assets::json::JsonAssetPlugin;
use eternal_gauntlet::animation::AnimationPlugin;
use eternal_gauntlet::asset_loading::AssetLoadingPlugin;
use eternal_gauntlet::audio::AudioPlugin;
use eternal_gauntlet::camera::FollowCameraPlugin;
use eternal_gauntlet::collision::CollisionPlugin;
use eternal_gauntlet::enemy::{EnemiesData, EnemyPlugin, SpawnData};
use eternal_gauntlet::enemy_textures::EnemyTexturesPlugin;
use eternal_gauntlet::generate_texture_atlas::{
    GenerateTextureAtlasPlugin, SerializableTextureAtlasLayout,
};
use eternal_gauntlet::gui::GuiPlugin;
use eternal_gauntlet::hit_textures::HitTexturesPlugin;
use eternal_gauntlet::player::PlayerPlugin;
use eternal_gauntlet::state::GameState;
use eternal_gauntlet::upgrade_menu::UpgradeMenu;
use eternal_gauntlet::wand::WandPlugin;
use eternal_gauntlet::world::{SelectedCharacter, WorldPlugin};
use eternal_gauntlet::*;

fn main() {
    App::new()
        .init_state::<GameState>()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // resizable: true,
                        // focused: true,
                        // resolution: (WW, WH).into(),
                        canvas: Some("#game-canvas".into()),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::rgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .add_plugins((
            JsonAssetPlugin::<SerializableTextureAtlasLayout>::new(&[
                "enemy_texture_atlas_layout.json",
                "hit_texture_atlas_layout.json",
            ]),
            JsonAssetPlugin::<SpawnData>::new(&["enemy_spawns.json"]),
            JsonAssetPlugin::<EnemiesData>::new(&["enemies.json"]),
        ))
        .add_plugins(AnimationPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(EnemyTexturesPlugin)
        .add_plugins(FollowCameraPlugin)
        .add_plugins(AssetLoadingPlugin)
        // .add_plugins(GenerateTextureAtlasPlugin)
        .add_plugins(GuiPlugin)
        .add_plugins(HitTexturesPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(UpgradeMenu)
        .add_plugins(WandPlugin)
        .add_plugins(WorldPlugin)
        .insert_resource(Msaa::Off)
        .add_systems(Update, close_on_esc)
        .init_resource::<SelectedCharacter>()
        .add_systems(Update, process_js_messages)
        .add_systems(OnEnter(GameState::MainMenu), on_game_end)
        .run();
}

static JS_MESSAGE: Mutex<Option<String>> = Mutex::new(None);

#[wasm_bindgen]
pub fn send_message_to_bevy(message: String) {
    *JS_MESSAGE.lock().unwrap() = Some(message);
}

fn process_js_messages(mut selected_character: ResMut<SelectedCharacter>) {
    if let Some(message) = JS_MESSAGE.lock().unwrap().take() {
        info!("Message: {}", message.clone());
        selected_character.0 = Some(message);
    }
}

fn on_game_end(mut score: ResMut<Score>) {
    info!("Score: {}", score.0);
    if score.0 > 0 {
        send_to_js(score.0);
        score.0 = 0;
    }
}

#[wasm_bindgen(module = "/js-link.js")]
extern "C" {
    fn send_to_js(score: u32);
}

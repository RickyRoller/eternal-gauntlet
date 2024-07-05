use bevy::prelude::*;
use bevy::window::close_on_esc;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

use crate::resources::HeroTextureAtlases;
use bevy::sprite::TextureAtlas;
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
use eternal_gauntlet::player::{Player, PlayerPlugin};
use eternal_gauntlet::state::GameState;
use eternal_gauntlet::upgrade_menu::UpgradeMenu;
use eternal_gauntlet::wand::WandPlugin;
use eternal_gauntlet::world::{SelectedCharacter, WorldPlugin};
use eternal_gauntlet::*;

fn main() {
    App::new()
        .init_state::<GameState>()
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
        .run();
}

static JS_MESSAGE: Mutex<Option<String>> = Mutex::new(None);

#[wasm_bindgen]
pub fn send_message_to_bevy(message: String) {
    *JS_MESSAGE.lock().unwrap() = Some(message);
}

fn process_js_messages(
    mut selected_character: ResMut<SelectedCharacter>,
    hero_atlases: Res<HeroTextureAtlases>,
    mut player_query: Query<(&mut TextureAtlas, &mut Handle<Image>), With<Player>>,
) {
    if let Some(message) = JS_MESSAGE.lock().unwrap().take() {
        let message_clone = message.clone();
        selected_character.0 = Some(message);

        if let Some(hero_atlas) = hero_atlases.get_hero(&message_clone) {
            if let Ok((mut texture_atlas, mut image_handle)) = player_query.get_single_mut() {
                if let (Some(layout), Some(image)) = (hero_atlas.layout, hero_atlas.image) {
                    texture_atlas.layout = layout;
                    *image_handle = image;
                }
            }
        }
    }
}

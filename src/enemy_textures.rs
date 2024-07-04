use crate::generate_texture_atlas::SerializableTextureAtlasLayout;
use crate::state::GameState;
use bevy::prelude::*;

#[derive(Resource)]
pub struct EnemyTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

#[derive(Resource)]
pub struct EnemyTextureAtlasHandle(pub Handle<SerializableTextureAtlasLayout>);

pub struct EnemyTexturesPlugin;

impl Plugin for EnemyTexturesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyTextureAtlas>()
            .add_systems(Startup, load_json_resources)
            .add_systems(OnEnter(GameState::MainMenu), setup_texture_atlas_from_files);
    }
}

fn load_json_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let enemy_texture_atlas_handle =
        EnemyTextureAtlasHandle(asset_server.load("enemy_texture_atlas_layout.json"));
    commands.insert_resource(enemy_texture_atlas_handle);
}

fn setup_texture_atlas_from_files(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut enemy_texture_atlas: ResMut<EnemyTextureAtlas>,
    enemy_texture_atlas_handle: Res<EnemyTextureAtlasHandle>,
    serializable_layouts: Res<Assets<SerializableTextureAtlasLayout>>,
) {
    // Load the pre-generated texture atlas image
    let texture_handle: Handle<Image> = asset_server.load("enemy_textures.png");

    if let Some(layout) = serializable_layouts.get(&enemy_texture_atlas_handle.0) {
        // Create a TextureAtlasLayout from the loaded data
        let mut texture_atlas_layout = TextureAtlasLayout::new_empty(layout.size);

        for rect in layout.textures.clone() {
            texture_atlas_layout.add_texture(rect);
        }

        // Add the layout to the texture atlases assets
        let layout_handle = texture_atlases.add(texture_atlas_layout);

        // Update the HitTextureAtlas resource
        enemy_texture_atlas.image = Some(texture_handle);
        enemy_texture_atlas.layout = Some(layout_handle);
    }
}

impl Default for EnemyTextureAtlas {
    fn default() -> Self {
        Self {
            layout: None,
            image: None,
        }
    }
}

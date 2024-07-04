use crate::generate_texture_atlas::SerializableTextureAtlasLayout;
use crate::state::GameState;
use bevy::prelude::*;

#[derive(Resource)]
pub struct HitTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

#[derive(Resource)]
pub struct HitTextureAtlasHandle(pub Handle<SerializableTextureAtlasLayout>);

pub struct HitTexturesPlugin;

impl Plugin for HitTexturesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HitTextureAtlas>()
            .add_systems(Startup, load_json_resources)
            .add_systems(OnEnter(GameState::MainMenu), setup_texture_atlas_from_files);
    }
}

fn load_json_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let hit_texture_atlas_handle =
        HitTextureAtlasHandle(asset_server.load("hit_texture_atlas_layout.json"));
    commands.insert_resource(hit_texture_atlas_handle);
}

fn setup_texture_atlas_from_files(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut hit_texture_atlas: ResMut<HitTextureAtlas>,
    hit_texture_atlas_handle: Res<HitTextureAtlasHandle>,
    serializable_layouts: Res<Assets<SerializableTextureAtlasLayout>>,
) {
    // Load the pre-generated texture atlas image
    let texture_handle: Handle<Image> = asset_server.load("hit_textures.png");

    // Load and parse the texture atlas layout JSON
    // let hit_texture_atlas_data =
    //     fs::read_to_string("assets/hit_texture_atlas_layout.json").expect("Unable to read file");
    // let layout: SerializableTextureAtlasLayout =
    //     serde_json::from_str(&hit_texture_atlas_data).expect("Unable to parse JSON");

    if let Some(layout) = serializable_layouts.get(&hit_texture_atlas_handle.0) {
        // Create a TextureAtlasLayout from the loaded data
        let mut texture_atlas_layout = TextureAtlasLayout::new_empty(layout.size);

        for rect in layout.textures.clone() {
            texture_atlas_layout.add_texture(rect);
        }

        // Add the layout to the texture atlases assets
        let layout_handle = texture_atlases.add(texture_atlas_layout);

        // Update the HitTextureAtlas resource
        hit_texture_atlas.image = Some(texture_handle);
        hit_texture_atlas.layout = Some(layout_handle);
    }
}

impl Default for HitTextureAtlas {
    fn default() -> Self {
        Self {
            layout: None,
            image: None,
        }
    }
}

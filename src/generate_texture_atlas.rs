use crate::state::GameState;
use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::render::texture::ImageSampler;

use image::ImageFormat;
use serde::{Deserialize, Serialize};

use serde_json;
use std::fs::File;
use std::io::Write;

#[derive(Resource, Default)]
struct HitSpriteFolder(Handle<LoadedFolder>);

#[derive(Resource, Default)]
struct EnemySpriteFolder(Handle<LoadedFolder>);

#[derive(Serialize, Deserialize, Asset, TypePath)]
pub struct SerializableTextureAtlasLayout {
    pub size: Vec2,
    pub textures: Vec<Rect>,
}

impl From<&TextureAtlasLayout> for SerializableTextureAtlasLayout {
    fn from(layout: &TextureAtlasLayout) -> Self {
        Self {
            size: layout.size,
            textures: layout.textures.clone(),
        }
    }
}

pub struct GenerateTextureAtlasPlugin;

impl Plugin for GenerateTextureAtlasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_folders)
            .add_systems(OnEnter(GameState::GameInit), generate_enemy_textures)
            .add_systems(OnEnter(GameState::GameInit), generate_hit_textures);
    }
}
fn load_folders(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(HitSpriteFolder(asset_server.load_folder("hits")));
    commands.insert_resource(EnemySpriteFolder(asset_server.load_folder("enemies")));
}

fn generate_enemy_textures(
    mut textures: ResMut<Assets<Image>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    sprite_handles: Res<EnemySpriteFolder>,
) {
    let loaded_folder = loaded_folders.get(&sprite_handles.0).unwrap();
    let (texture_atlas_linear, linear_texture) = create_texture_atlas(
        loaded_folder,
        None,
        Some(ImageSampler::nearest()),
        &mut textures,
    );

    // Save the texture atlas image to a file
    if let Some(image) = textures.get(&linear_texture) {
        let buffer = image.clone().try_into_dynamic().unwrap();
        buffer
            .save_with_format("assets/enemy_textures.png", ImageFormat::Png)
            .unwrap();
        println!("Texture atlas saved to assets/enemy_textures.png");
    }

    let serializable_layout = SerializableTextureAtlasLayout::from(&texture_atlas_linear);
    let layout_json = serde_json::to_string_pretty(&serializable_layout).unwrap();
    let mut file = File::create("assets/enemy_texture_atlas_layout.json").unwrap();
    file.write_all(layout_json.as_bytes()).unwrap();
    println!("Texture atlas layout saved to assets/enemy_texture_atlas_layout.json");
}

fn generate_hit_textures(
    mut textures: ResMut<Assets<Image>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    sprite_handles: Res<HitSpriteFolder>,
) {
    let loaded_folder = loaded_folders.get(&sprite_handles.0).unwrap();
    let (texture_atlas_linear, linear_texture) = create_texture_atlas(
        loaded_folder,
        None,
        Some(ImageSampler::nearest()),
        &mut textures,
    );

    // Save the texture atlas image to a file
    if let Some(image) = textures.get(&linear_texture) {
        let buffer = image.clone().try_into_dynamic().unwrap();
        buffer
            .save_with_format("assets/hit_textures.png", ImageFormat::Png)
            .unwrap();
        println!("Texture atlas saved to assets/hit_textures.png");
    }

    let serializable_layout = SerializableTextureAtlasLayout::from(&texture_atlas_linear);
    let layout_json = serde_json::to_string_pretty(&serializable_layout).unwrap();
    let mut file = File::create("assets/hit_texture_atlas_layout.json").unwrap();
    file.write_all(layout_json.as_bytes()).unwrap();
    println!("Texture atlas layout saved to assets/hit_texture_atlas_layout.json");
}

fn create_texture_atlas(
    folder: &LoadedFolder,
    padding: Option<UVec2>,
    sampling: Option<ImageSampler>,
    textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, Handle<Image>) {
    // Build a texture atlas using the individual sprites
    let mut texture_atlas_builder =
        TextureAtlasBuilder::default().padding(padding.unwrap_or_default());
    for handle in folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        let Some(texture) = textures.get(id) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                handle.path().unwrap()
            );
            continue;
        };

        texture_atlas_builder.add_texture(Some(id), texture);
    }

    let (texture_atlas_layout, texture) = texture_atlas_builder.finish().unwrap();
    let texture = textures.add(texture);

    // Update the sampling settings of the texture atlas
    let image = textures.get_mut(&texture).unwrap();
    image.sampler = sampling.unwrap_or_default();

    (texture_atlas_layout, texture)
}

// DEUBG ATLAS WITH SHOWING INDEX NUMBERS
// let atlas_handle = texture_atlases.add(texture_atlas_linear.clone());

// // Calculate grid dimensions
// let sprites_count = texture_atlas_linear.textures.len();
// let grid_size = (sprites_count as f32).sqrt().ceil() as usize;
// let sprite_size = 32.0; // Adjust this based on your sprite size
// let spacing = 10.0;

// for (index, texture) in texture_atlas_linear.textures.iter().enumerate() {
//     let row = index / grid_size;
//     let col = index % grid_size;
//     let x = col as f32 * (sprite_size + spacing)
//         - (grid_size as f32 * (sprite_size + spacing)) / 2.0;
//     let y = -(row as f32 * (sprite_size + spacing))
//         + (grid_size as f32 * (sprite_size + spacing)) / 2.0;

//     // Spawn sprite
//     commands.spawn(SpriteSheetBundle {
//         texture: linear_texture.clone(),
//         atlas: TextureAtlas {
//             index,
//             layout: atlas_handle.clone(),
//         },
//         transform: Transform::from_xyz(x, y, 0.0),
//         ..default()
//     });

//     // Spawn label
//     create_label(
//         &mut commands,
//         (x, y - sprite_size / 2.0 - 5.0, 1.0),
//         &index.to_string(),
//         TextStyle {
//             font_size: 8.0,
//             color: Color::WHITE,
//             ..default()
//         },
//     );
// }

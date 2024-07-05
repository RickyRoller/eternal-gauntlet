use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::state::GameState;
use crate::*;

pub struct ResourcesPlugin;

#[derive(Resource, Clone)]
pub struct GlobalTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

#[derive(Resource)]
pub struct HeroTextureAtlases {
    pub dwarf_f: GlobalTextureAtlas,
    pub dwarf_m: GlobalTextureAtlas,
    pub wizzard_f: GlobalTextureAtlas,
    pub wizzard_m: GlobalTextureAtlas,
    pub elf_f: GlobalTextureAtlas,
    pub elf_m: GlobalTextureAtlas,
    pub knight_f: GlobalTextureAtlas,
    pub knight_m: GlobalTextureAtlas,
    pub lizard_f: GlobalTextureAtlas,
    pub lizard_m: GlobalTextureAtlas,
    pub doc: GlobalTextureAtlas,
}

#[derive(Resource)]
pub struct CursorPosition(pub Option<Vec2>);

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalTextureAtlas::default())
            .insert_resource(HeroTextureAtlases::default())
            .insert_resource(CursorPosition(None))
            .add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(
                Update,
                update_cursor_position.run_if(in_state(GameState::InGame)),
            );
    }
}

fn load_assets(
    mut handle: ResMut<GlobalTextureAtlas>,
    mut hero_atlases_handle: ResMut<HeroTextureAtlases>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    handle.image = Some(asset_server.load("assets-wand.png"));

    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(TILE_W as f32, TILE_H as f32),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    handle.layout = Some(texture_atlas_layouts.add(layout));

    let hero_layout =
        TextureAtlasLayout::from_grid(Vec2::new(16 as f32, 28 as f32), 4, 2, None, None);

    hero_atlases_handle.doc = GlobalTextureAtlas {
        image: Some(asset_server.load("heroes/doc.png")),
        layout: Some(texture_atlas_layouts.add(hero_layout.clone())),
    };
    hero_atlases_handle.dwarf_f = GlobalTextureAtlas {
        image: Some(asset_server.load("heroes/dwarf-f.png")),
        layout: Some(texture_atlas_layouts.add(hero_layout.clone())),
    };
    hero_atlases_handle.dwarf_m = GlobalTextureAtlas {
        image: Some(asset_server.load("heroes/dwarf-m.png")),
        layout: Some(texture_atlas_layouts.add(hero_layout.clone())),
    };
    hero_atlases_handle.elf_f = GlobalTextureAtlas {
        image: Some(asset_server.load("heroes/elf-f.png")),
        layout: Some(texture_atlas_layouts.add(hero_layout.clone())),
    };
    hero_atlases_handle.elf_m = GlobalTextureAtlas {
        image: Some(asset_server.load("heroes/elf-m.png")),
        layout: Some(texture_atlas_layouts.add(hero_layout.clone())),
    };
    hero_atlases_handle.knight_f = GlobalTextureAtlas {
        image: Some(asset_server.load("heroes/knight-f.png")),
        layout: Some(texture_atlas_layouts.add(hero_layout.clone())),
    };
    hero_atlases_handle.knight_m = GlobalTextureAtlas {
        image: Some(asset_server.load("heroes/knight-m.png")),
        layout: Some(texture_atlas_layouts.add(hero_layout.clone())),
    };
    hero_atlases_handle.lizard_f = GlobalTextureAtlas {
        image: Some(asset_server.load("heroes/lizard-f.png")),
        layout: Some(texture_atlas_layouts.add(hero_layout.clone())),
    };
    hero_atlases_handle.lizard_m = GlobalTextureAtlas {
        image: Some(asset_server.load("heroes/lizard-m.png")),
        layout: Some(texture_atlas_layouts.add(hero_layout.clone())),
    };
    hero_atlases_handle.wizzard_f = GlobalTextureAtlas {
        image: Some(asset_server.load("heroes/wizzard-f.png")),
        layout: Some(texture_atlas_layouts.add(hero_layout.clone())),
    };
    hero_atlases_handle.wizzard_m = GlobalTextureAtlas {
        image: Some(asset_server.load("heroes/wizzard-m.png")),
        layout: Some(texture_atlas_layouts.add(hero_layout.clone())),
    };
}

fn update_cursor_position(
    mut cursor_pos: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if window_query.is_empty() || camera_query.is_empty() {
        cursor_pos.0 = None;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();
    cursor_pos.0 = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());
}

impl Default for GlobalTextureAtlas {
    fn default() -> Self {
        Self {
            layout: None,
            image: None,
        }
    }
}

impl Default for HeroTextureAtlases {
    fn default() -> Self {
        Self {
            doc: GlobalTextureAtlas::default(),
            dwarf_f: GlobalTextureAtlas::default(),
            dwarf_m: GlobalTextureAtlas::default(),
            wizzard_f: GlobalTextureAtlas::default(),
            wizzard_m: GlobalTextureAtlas::default(),
            elf_f: GlobalTextureAtlas::default(),
            elf_m: GlobalTextureAtlas::default(),
            knight_f: GlobalTextureAtlas::default(),
            knight_m: GlobalTextureAtlas::default(),
            lizard_f: GlobalTextureAtlas::default(),
            lizard_m: GlobalTextureAtlas::default(),
        }
    }
}

impl HeroTextureAtlases {
    pub fn get_hero(&self, hero: &str) -> Option<GlobalTextureAtlas> {
        match hero {
            "doc" => Some(self.doc.clone()),
            "dwarf_f" => Some(self.dwarf_f.clone()),
            "dwarf_m" => Some(self.dwarf_m.clone()),
            "wizzard_f" => Some(self.wizzard_f.clone()),
            "wizzard_m" => Some(self.wizzard_m.clone()),
            "elf_f" => Some(self.elf_f.clone()),
            "elf_m" => Some(self.elf_m.clone()),
            "knight_f" => Some(self.knight_f.clone()),
            "knight_m" => Some(self.knight_m.clone()),
            "lizard_f" => Some(self.lizard_f.clone()),
            "lizard_m" => Some(self.lizard_m.clone()),
            _ => None,
        }
    }
}

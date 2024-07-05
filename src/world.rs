use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::Rng;

use crate::animation::AnimationTimer;
use crate::player::{Experience, Health, Level, Player, PlayerState};
use crate::wand::{Wand, WandTimer};
use crate::*;
use crate::{state::GameState, GlobalTextureAtlas, HeroTextureAtlases};

pub struct WorldPlugin;

#[derive(Component)]
pub struct GameEntity;

#[derive(Resource)]
pub struct SelectedCharacter(pub Option<String>);

impl Default for SelectedCharacter {
    fn default() -> Self {
        SelectedCharacter(Some("wizzard-m".to_string()))
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameInit),
            (init_world, spawn_world_decorations),
        )
        .add_systems(OnExit(GameState::InGame), despawn_all_game_entities);
    }
}

fn init_world(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    selected_character: Res<SelectedCharacter>,
    hero_atlases: Res<HeroTextureAtlases>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let hero = selected_character.0.as_ref().unwrap();
    let hero_texture_atlas = hero_atlases.get_hero(hero);

    if let Some(hero_texture_atlas) = hero_texture_atlas {
        commands.spawn((
            SpriteSheetBundle {
                texture: hero_texture_atlas.image.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: hero_texture_atlas.layout.clone().unwrap(),
                    index: 0,
                },
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            Player,
            Health(PLAYER_HEALTH),
            Experience(0.0),
            Level(1),
            PlayerState::default(),
            AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
            GameEntity,
        ));
        commands.spawn((
            SpriteSheetBundle {
                texture: handle.image.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: 17,
                },
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            Wand,
            WandTimer(Stopwatch::new()),
            GameEntity,
        ));

        next_state.set(GameState::InGame);
    }
}

fn spawn_world_decorations(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_WORLD_DECORATIONS {
        let x = rng.gen_range(-WORLD_W..WORLD_W);
        let y = rng.gen_range(-WORLD_H..WORLD_H);
        commands.spawn((
            SpriteSheetBundle {
                texture: handle.image.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: rng.gen_range(24..=25),
                },
                transform: Transform::from_translation(vec3(x, y, 0.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            GameEntity,
        ));
    }
}

fn despawn_all_game_entities(
    mut commands: Commands,
    all_entities: Query<Entity, With<GameEntity>>,
) {
    for e in all_entities.iter() {
        commands.entity(e).despawn_recursive();
    }
}

use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;

use crate::animation::AnimationTimer;
use crate::enemy::{Enemy, EnemyStats, EnemyType};
use crate::world::GameEntity;
use crate::*;

use self::animation::AnimationIndices;
use self::enemy_textures::EnemyTextureAtlas;

#[derive(Bundle)]
pub struct EnemyBundle {
    sprite_sheet: SpriteSheetBundle,
    enemy: Enemy,
    enemy_type: EnemyType,
    animation_indicies: AnimationIndices,
    animation_timer: AnimationTimer,
    game_entity: GameEntity,
}

impl EnemyBundle {
    pub fn new(
        handle: &EnemyTextureAtlas,
        enemy_type: EnemyType,
        animation_indicies: AnimationIndices,
        position: Vec3,
        stats: EnemyStats,
    ) -> Self {
        Self {
            sprite_sheet: SpriteSheetBundle {
                texture: handle.image.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: animation_indicies.first,
                },
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            enemy: Enemy::new(stats),
            enemy_type,
            animation_indicies,
            animation_timer: AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
            game_entity: GameEntity,
        }
    }
}

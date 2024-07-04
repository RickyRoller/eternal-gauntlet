use bevy::ecs::bundle::Bundle;
use bevy::prelude::*;

use crate::animation::AnimationTimer;
use crate::world::GameEntity;
use crate::*;

use self::animation::AnimationIndices;
use self::hit_textures::HitTextureAtlas;

#[derive(Component)]
pub struct LightningHit;

#[derive(Bundle)]
pub struct LightningHitBundle {
    sprite_sheet: SpriteSheetBundle,
    animation_indicies: AnimationIndices,
    animation_timer: AnimationTimer,
    game_entity: GameEntity,
}

impl LightningHitBundle {
    pub fn new(
        handle: &HitTextureAtlas,
        animation_indicies: AnimationIndices,
        position: Vec3,
    ) -> Self {
        Self {
            sprite_sheet: SpriteSheetBundle {
                texture: handle.image.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: animation_indicies.first,
                },
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 10.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            animation_indicies,
            animation_timer: AnimationTimer(Timer::from_seconds(0.06, TimerMode::Repeating)),
            game_entity: GameEntity,
        }
    }
}

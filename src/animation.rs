use bevy::{animation, prelude::*};

use crate::{
    enemy::Enemy,
    gun::Gun,
    lightning_hit_bundle::LightningHit,
    player::{Player, PlayerState},
    state::GameState,
    wand::Wand,
    CursorPosition,
};

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

pub struct AnimationPlugin;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                animation_timer_tick,
                animate_player,
                animate_enemy,
                animate_hit,
                flip_gun_sprite_y,
                flip_wand_sprite_y,
                flip_player_sprite_x,
                flip_enemy_sprite_x,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn animation_timer_tick(
    time: Res<Time>,
    mut query: Query<&mut AnimationTimer, With<AnimationTimer>>,
) {
    for mut timer in query.iter_mut() {
        timer.tick(time.delta());
    }
}

fn animate_player(
    mut player_query: Query<(&mut TextureAtlas, &PlayerState, &AnimationTimer), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut atlas, state, timer) = player_query.single_mut();
    if timer.just_finished() {
        let base_sprite_index = match state {
            PlayerState::Idle => 0,
            PlayerState::Run => 4,
        };
        atlas.index = base_sprite_index + (atlas.index + 1) % 4;
    }
}

fn animate_enemy(
    mut enemy_query: Query<(&mut TextureAtlas, &AnimationTimer, &AnimationIndices), With<Enemy>>,
) {
    if enemy_query.is_empty() {
        return;
    }

    for (mut atlas, timer, animation_indicies) in enemy_query.iter_mut() {
        if timer.just_finished() {
            atlas.index = if atlas.index == animation_indicies.last {
                animation_indicies.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn animate_hit(
    mut commands: Commands,
    mut hit_query: Query<
        (
            &mut TextureAtlas,
            &AnimationTimer,
            &AnimationIndices,
            Entity,
        ),
        With<LightningHit>,
    >,
) {
    if hit_query.is_empty() {
        return;
    }

    for (mut atlas, timer, animation_indicies, entity) in hit_query.iter_mut() {
        if timer.just_finished() {
            if atlas.index == animation_indicies.last {
                commands.entity(entity).despawn();
            } else {
                atlas.index = atlas.index + 1;
            }
        }
    }
}

fn flip_player_sprite_x(
    cursor_position: Res<CursorPosition>,
    mut player_query: Query<(&mut Sprite, &Transform), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut sprite, transform) = player_query.single_mut();
    if let Some(cursor_position) = cursor_position.0 {
        if cursor_position.x > transform.translation.x {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    }
}

fn flip_enemy_sprite_x(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Sprite, &Transform), With<Enemy>>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation;
    for (mut sprite, transform) in enemy_query.iter_mut() {
        if transform.translation.x < player_pos.x {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    }
}

fn flip_gun_sprite_y(
    cursor_position: Res<CursorPosition>,
    mut gun_query: Query<(&mut Sprite, &Transform), With<Gun>>,
) {
    if gun_query.is_empty() {
        return;
    }

    let (mut sprite, transform) = gun_query.single_mut();
    if let Some(cursor_position) = cursor_position.0 {
        if cursor_position.x > transform.translation.x {
            sprite.flip_y = false;
        } else {
            sprite.flip_y = true;
        }
    }
}

fn flip_wand_sprite_y(
    cursor_position: Res<CursorPosition>,
    mut wand_query: Query<(&mut Sprite, &Transform), With<Wand>>,
) {
    if wand_query.is_empty() {
        return;
    }

    let (mut sprite, transform) = wand_query.single_mut();
    if let Some(cursor_position) = cursor_position.0 {
        if cursor_position.x > transform.translation.x {
            sprite.flip_y = false;
        } else {
            sprite.flip_y = true;
        }
    }
}

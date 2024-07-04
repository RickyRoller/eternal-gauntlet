use bevy::math::vec3;
use bevy::prelude::*;

use crate::state::GameState;
use crate::utils::{ease_in_out_quint, scale_value};
use crate::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Health(pub f32);
#[derive(Component)]
pub struct Experience(pub f32);
#[derive(Component)]
pub struct Level(pub u32);

#[derive(Component, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Run,
}

#[derive(Event)]
pub struct PlayerEnemyCollisionEvent;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerEnemyCollisionEvent>().add_systems(
            Update,
            (
                handle_player_death,
                handle_player_input,
                handle_player_enemy_collision_events,
                handle_player_level_up,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_player_level_up(
    mut experience_query: Query<&mut Experience, With<Player>>,
    mut level_query: Query<&mut Level, With<Player>>,
) {
    if experience_query.is_empty() || level_query.is_empty() {
        return;
    }
    let mut experience = experience_query.single_mut();
    let mut level = level_query.single_mut();
    let experience_per_level = experience_curve(level.0);
    if experience.0 >= experience_per_level {
        experience.0 -= experience_per_level;
        level.0 += 1;
    }
}

fn experience_curve(level: u32) -> f32 {
    let experience_per_level = EXPERIENCE_PER_LEVEL;
    let leveling_progress = if level as f32 >= MAX_LEVEL as f32 {
        1.0
    } else {
        level as f32 / MAX_LEVEL as f32
    };
    let multiplier = scale_value(ease_in_out_quint(leveling_progress), 10.0);
    let exp_progress = leveling_progress * leveling_progress; // easeInQuad easing fn
    let experience = (multiplier + exp_progress) * experience_per_level;
    return experience;
}

fn handle_player_enemy_collision_events(
    mut player_query: Query<&mut Health, With<Player>>,
    mut events: EventReader<PlayerEnemyCollisionEvent>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut health = player_query.single_mut();
    for _ in events.read() {
        health.0 -= ENEMY_DAMAGE;
    }
}

fn handle_player_death(
    player_query: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player_query.is_empty() {
        return;
    }
    let health = player_query.single();
    if health.0 <= 0.0 {
        next_state.set(GameState::MainMenu);
    }
}

fn handle_player_input(
    mut player_query: Query<(&mut Transform, &mut PlayerState), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut transform, mut player_state) = player_query.single_mut();
    let w_key = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    let a_key = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    let s_key = keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    let d_key =
        keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);

    let mut delta = Vec2::ZERO;
    if w_key {
        delta.y += 1.0;
    }
    if s_key {
        delta.y -= 1.0;
    }
    if a_key {
        delta.x -= 1.0;
    }
    if d_key {
        delta.x += 1.0;
    }
    delta = delta.normalize();

    if delta.is_finite() && (w_key || a_key || s_key || d_key) {
        transform.translation += vec3(delta.x, delta.y, 0.0) * PLAYER_SPEED;
        transform.translation.z = 10.0;
        *player_state = PlayerState::Run;
    } else {
        *player_state = PlayerState::Idle;
    }
}

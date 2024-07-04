use std::f32::consts::PI;

use bevy::math::vec3;
use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

use crate::enemy_bundle::EnemyBundle;
use crate::enemy_textures::EnemyTextureAtlas;
use crate::player::Experience;
use crate::player::Player;
use crate::state::GameState;
use crate::*;

use self::animation::AnimationIndices;

#[derive(Resource, Debug)]
struct GameTime(f32);

#[derive(Resource)]
pub struct EnemiesDataHandle(pub Handle<EnemiesData>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnemyStats {
    pub health: u32,
    pub damage: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnemyLevels {
    #[serde(flatten)]
    pub levels: HashMap<String, EnemyStats>,
}

#[derive(Deserialize, Asset, TypePath, Clone)]
pub struct EnemiesData {
    pub undead: EnemyLevels,
    pub orc: EnemyLevels,
    pub demon: EnemyLevels,
}

#[derive(Resource)]
pub struct EnemiesDataResource(pub EnemiesData);

#[derive(Resource)]
pub struct EnemiesSpawnDataHandle(pub Handle<SpawnData>);

#[derive(Deserialize, Asset, TypePath, Clone)]
pub struct SpawnData {
    pub enemy_spawns: Vec<EnemySpawn>,
}

#[derive(Resource)]
pub struct SpawnDataResource(pub SpawnData);

#[derive(Deserialize, Debug, Clone)]
pub struct EnemySpawn {
    id: String,
    race: String,
    power: String,
    count: u32,
    start_time: String,
    end_time: String,
}

#[derive(Resource, Debug)]
struct SpawnTimer(Timer);

#[derive(Resource)]
struct SpawnTracker(HashMap<String, u32>);

#[derive(Resource)]
struct EnemyMaxSpawnTime(f32);

#[derive(Component)]
pub struct Enemy {
    pub current_health: f32,
    pub stats: EnemyStats,
}

#[derive(Component)]
pub enum EnemyType {
    Undead,
    Orc,
    Demon,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_json_resources)
            .add_systems(OnEnter(GameState::InGame), setup)
            .add_systems(
                Update,
                (
                    spawn_enemies_system,
                    update_enemy_transform,
                    despawn_dead_enemies,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

fn load_json_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let enemies_data_handle = EnemiesDataHandle(asset_server.load("enemies.json"));
    commands.insert_resource(enemies_data_handle);
    let enemies_spawn_data_handle = EnemiesSpawnDataHandle(asset_server.load("enemy_spawns.json"));
    commands.insert_resource(enemies_spawn_data_handle);
}

fn setup(
    mut commands: Commands,
    enemies_data_handle: Res<EnemiesDataHandle>,
    enemies_data_assets: Res<Assets<EnemiesData>>,
    spawn_data_handle: Res<EnemiesSpawnDataHandle>,
    spawn_data_assets: Res<Assets<SpawnData>>,
) {
    if let Some(enemies_data) = enemies_data_assets.get(&enemies_data_handle.0) {
        commands.insert_resource(EnemiesDataResource(enemies_data.clone()));
    }

    if let Some(spawn_data) = spawn_data_assets.get(&spawn_data_handle.0) {
        let mut spawn_data = spawn_data.clone();
        let mut max_spawn_time = 0.0;
        for spawn in &mut spawn_data.enemy_spawns {
            spawn.id = Uuid::new_v4().to_string();
            let end_seconds = parse_time_to_seconds(&spawn.end_time);
            if end_seconds > max_spawn_time {
                max_spawn_time = end_seconds;
            }
        }

        commands.insert_resource(EnemyMaxSpawnTime(max_spawn_time));
        commands.insert_resource(SpawnDataResource(spawn_data.clone()));
    }

    commands.insert_resource(GameTime(0.0));
    commands.insert_resource(SpawnTimer(Timer::new(
        Duration::from_secs_f32(0.1),
        TimerMode::Repeating,
    )));
    commands.insert_resource(SpawnTracker(HashMap::new()));
}

fn spawn_enemies_system(
    mut commands: Commands,
    time: Res<Time>,
    mut game_time: ResMut<GameTime>,
    enemy_data: Res<EnemiesDataResource>,
    spawn_data: Res<SpawnDataResource>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut spawn_tracker: ResMut<SpawnTracker>,
    enemy_texture_atlas: Res<EnemyTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    max_spawn_time: Res<EnemyMaxSpawnTime>,
) {
    // Update game time
    game_time.0 += time.delta_seconds();
    let current_game_time = game_time.0;

    // Update spawn timer
    spawn_timer.0.tick(time.delta());

    if player_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    let max_spawn_time = max_spawn_time.0;

    // Check if it's time to attempt spawning
    if spawn_timer.0.just_finished() {
        let spawn_multiplier = (current_game_time / max_spawn_time).floor();

        for spawn in &spawn_data.0.enemy_spawns {
            let start_seconds =
                parse_time_to_seconds(&spawn.start_time) + (max_spawn_time * spawn_multiplier);
            let end_seconds =
                parse_time_to_seconds(&spawn.end_time) + (max_spawn_time * spawn_multiplier);

            if current_game_time >= start_seconds && current_game_time <= end_seconds {
                let spawn_id = format!("{}_{}", spawn.id, spawn_multiplier);
                let total_to_spawn =
                    calculate_spawn_count(spawn, start_seconds, end_seconds, current_game_time);
                let already_spawned = spawn_tracker.0.entry(spawn_id).or_insert(0);
                let new_spawns = total_to_spawn.saturating_sub(*already_spawned);

                let enemy_stats = enemy_data
                    .0
                    .get_enemy_stats(&spawn.race, &spawn.power)
                    .unwrap();

                for _ in 0..new_spawns {
                    let (x, y) = get_random_position_around(player_pos);
                    let enemy_type = EnemyType::get_race(&spawn.race);
                    let animation_indicies =
                        EnemyType::get_animation_indicies(&enemy_type, &spawn.power);
                    // Spawn enemy
                    let enemy_stats = enemy_stats.clone();
                    commands.spawn(EnemyBundle::new(
                        &enemy_texture_atlas,
                        enemy_type,
                        animation_indicies,
                        vec3(x, y, 1.0),
                        EnemyStats {
                            health: enemy_stats.health
                                + (enemy_stats.health * (1.2 * spawn_multiplier).floor() as u32),
                            damage: enemy_stats.damage,
                        },
                    ));
                }

                *already_spawned = total_to_spawn;
            }
        }
    }
}

fn parse_time_to_seconds(time_str: &str) -> f32 {
    let parts: Vec<&str> = time_str.split(':').collect();
    let minutes: f32 = parts[0].parse().unwrap_or(0.0);
    let seconds: f32 = parts[1].parse().unwrap_or(0.0);
    minutes * 60.0 + seconds
}

fn calculate_spawn_count(spawn: &EnemySpawn, start: f32, end: f32, current: f32) -> u32 {
    let total_time = end - start;
    let elapsed_time = current - start;
    let spawn_ratio = elapsed_time / total_time;
    let total_to_spawn = (spawn.count as f32 * spawn_ratio) as u32;
    total_to_spawn
}

fn despawn_dead_enemies(
    mut commands: Commands,
    enemy_query: Query<(&Enemy, Entity), With<Enemy>>,
    mut experience_query: Query<&mut Experience, With<Player>>,
) {
    if enemy_query.is_empty() || experience_query.is_empty() {
        return;
    }
    let mut experience = experience_query.single_mut();

    for (enemy, entity) in enemy_query.iter() {
        if enemy.current_health <= 0.0 {
            experience.0 += 1.0;
            commands.entity(entity).despawn();
        }
    }
}

fn update_enemy_transform(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation;
    for mut transform in enemy_query.iter_mut() {
        let dir = (player_pos - transform.translation).normalize();
        transform.translation += dir * ENEMY_SPEED;
    }
}

fn get_random_position_around(pos: Vec2) -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..PI * 2.0);
    let dist = rng.gen_range(1000.0..5000.0);

    let offset_x = angle.cos() * dist;
    let offset_y = angle.sin() * dist;

    let random_x = pos.x + offset_x;
    let random_y = pos.y + offset_y;

    (random_x, random_y)
}

impl Enemy {
    pub fn new(stats: EnemyStats) -> Self {
        Self {
            current_health: stats.health as f32,
            stats,
        }
    }
}

impl EnemyType {
    fn get_race(race: &str) -> Self {
        match race {
            "Undead" => Self::Undead,
            "Orc" => Self::Orc,
            "Demon" => Self::Demon,
            _ => Self::Undead,
        }
    }

    pub fn get_animation_indicies(&self, level: &str) -> AnimationIndices {
        match self {
            EnemyType::Undead => match level {
                "1" => AnimationIndices {
                    first: 52,
                    last: 55,
                },
                "2" => AnimationIndices {
                    first: 56,
                    last: 59,
                },
                "3" => AnimationIndices {
                    first: 48,
                    last: 51,
                },
                "4" => AnimationIndices {
                    first: 44,
                    last: 47,
                },
                "5" => AnimationIndices {
                    first: 40,
                    last: 43,
                },
                _ => AnimationIndices { first: 0, last: 0 },
            },
            EnemyType::Orc => match level {
                "1" => AnimationIndices {
                    first: 20,
                    last: 23,
                },
                "2" => AnimationIndices {
                    first: 36,
                    last: 39,
                },
                "3" => AnimationIndices {
                    first: 32,
                    last: 35,
                },
                "4" => AnimationIndices {
                    first: 24,
                    last: 27,
                },
                "5" => AnimationIndices {
                    first: 28,
                    last: 31,
                },
                _ => AnimationIndices { first: 0, last: 0 },
            },
            EnemyType::Demon => match level {
                "1" => AnimationIndices { first: 8, last: 11 },
                "2" => AnimationIndices {
                    first: 16,
                    last: 19,
                },
                "3" => AnimationIndices { first: 4, last: 7 },
                "4" => AnimationIndices {
                    first: 12,
                    last: 15,
                },
                "5" => AnimationIndices { first: 0, last: 3 },
                _ => AnimationIndices { first: 0, last: 0 },
            },
        }
    }
}

impl EnemiesData {
    pub fn get_enemy_stats(&self, race: &str, level: &str) -> Option<&EnemyStats> {
        match race.to_lowercase().as_str() {
            "undead" => self.undead.levels.get(level),
            "orc" => self.orc.levels.get(level),
            "demon" => self.demon.levels.get(level),
            _ => None,
        }
    }
}

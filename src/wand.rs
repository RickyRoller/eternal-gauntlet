use std::f32::consts::PI;

use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::time::Stopwatch;
// use bevy_prototype_lyon::prelude::*;

use crate::audio::{LightningEffectHandle, LightningSoundEffect};
use crate::enemy::Enemy;
use crate::hit_textures::HitTextureAtlas;
use crate::lightning_hit_bundle::{LightningHit, LightningHitBundle};
use crate::player::Player;
use crate::state::GameState;
use crate::*;

use self::animation::AnimationIndices;
use self::player::Level;

pub struct WandPlugin;

#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: f32,
    pub arcs: u32,
}

#[derive(Event)]
pub struct SecondaryArc {
    pub from_target: Entity,
    pub damage_event: DamageEvent,
}

#[derive(Component)]
pub struct LightningEffect {
    pub lifetime: f32,
}

#[derive(Component)]
pub struct Wand;
#[derive(Component)]
pub struct WandTimer(pub Stopwatch);
#[derive(Component)]
pub struct Lightning;

impl Plugin for WandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_event::<SecondaryArc>()
            .add_systems(
                Update,
                (
                    update_wand_transform,
                    // update_bullets,
                    handle_wand_input,
                    // despawn_old_bullets,
                    // draw_debug_cone,
                    apply_damage,
                    secondary_arc,
                    despawn_lightning,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

fn secondary_arc(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<ColorMaterial>>,
    mut secondary_arc_events: EventReader<SecondaryArc>,
    mut damage_events: EventWriter<DamageEvent>,
    mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for event in secondary_arc_events.read() {
        if let Ok((_entity, &transform)) = enemy_query.get_mut(event.damage_event.target) {
            let remaining_arcs = event.damage_event.arcs - 1;
            if remaining_arcs > 0 {
                if let Some((target, target_transform)) = get_nearest_enemy_in_radius(
                    &enemy_query,
                    transform.translation.truncate(),
                    150.0,
                    event.from_target,
                ) {
                    damage_events.send(DamageEvent {
                        target,
                        amount: event.damage_event.amount,
                        arcs: remaining_arcs,
                    });

                    draw_vector_path(
                        &mut commands,
                        &mut mesh_assets,
                        &mut material_assets,
                        transform.translation.truncate(),
                        target_transform.translation.truncate(),
                        Color::YELLOW,
                        2.0,
                    );
                }
            }
        }
    }
}

fn apply_damage(
    mut commands: Commands,
    hit_texture_atlas: Res<HitTextureAtlas>,
    mut damage_events: EventReader<DamageEvent>,
    mut secondary_arc_events: EventWriter<SecondaryArc>,
    mut enemy_query: Query<(&Transform, &mut Enemy), With<Enemy>>,
    lightning_sound: ResMut<LightningEffectHandle>,
) {
    for event in damage_events.read() {
        if let Ok((transform, mut enemy)) = enemy_query.get_mut(event.target) {
            // if # of arcs is > 0 then spawn another event and choose a random enemy to target nearby
            enemy.current_health -= event.amount;
            commands
                .spawn(LightningHitBundle::new(
                    &hit_texture_atlas,
                    AnimationIndices { first: 0, last: 3 },
                    transform.translation.clone(),
                ))
                .insert(LightningHit);

            secondary_arc_events.send(SecondaryArc {
                from_target: event.target,
                damage_event: DamageEvent {
                    target: event.target,
                    amount: event.amount,
                    arcs: event.arcs,
                },
            });

            commands.spawn((
                AudioBundle {
                    source: lightning_sound.handle.clone(),
                    ..default()
                },
                LightningSoundEffect,
            ));
        }
    }
}

// fn despawn_old_bullets(
//     mut commands: Commands,
//     bullet_query: Query<(&SpawnInstant, Entity), With<Bullet>>,
// ) {
//     for (instant, e) in bullet_query.iter() {
//         if instant.0.elapsed().as_secs_f32() > BULLET_TIME_SECS {
//             commands.entity(e).despawn();
//         }
//     }
// }

fn despawn_lightning(
    mut commands: Commands,
    mut lightning_query: Query<(&mut LightningEffect, Entity), With<Lightning>>,
    time: Res<Time>,
) {
    for (mut effect, entity) in lightning_query.iter_mut() {
        effect.lifetime = effect.lifetime - time.delta().as_millis() as f32;
        if effect.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn update_wand_transform(
    cursor_pos: Res<CursorPosition>,
    player_query: Query<&Transform, With<Player>>,
    mut wand_query: Query<&mut Transform, (With<Wand>, Without<Player>)>,
) {
    if player_query.is_empty() || wand_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation.truncate();
    let cursor_pos = match cursor_pos.0 {
        Some(pos) => pos,
        None => player_pos,
    };
    let mut wand_transform = wand_query.single_mut();

    let angle = (player_pos.y - cursor_pos.y).atan2(player_pos.x - cursor_pos.x) + PI;
    wand_transform.rotation = Quat::from_rotation_z(angle);

    let offset = 20.0;
    let new_wand_pos = vec2(
        player_pos.x + offset * angle.cos() - 5.0,
        player_pos.y + offset * angle.sin() - 10.0,
    );

    wand_transform.translation = vec3(new_wand_pos.x, new_wand_pos.y, wand_transform.translation.z);
    wand_transform.translation.z = 15.0;
}

fn handle_wand_input(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut wand_query: Query<(&Transform, &mut WandTimer), With<Wand>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    player_query: Query<(&Transform, &Level), With<Player>>,
    cursor_pos: Res<CursorPosition>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    if wand_query.is_empty() || player_query.is_empty() {
        return;
    }

    let (player_transform, player_level) = player_query.single();
    let (wand_transform, mut wand_timer) = wand_query.single_mut();
    wand_timer.0.tick(time.delta());

    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }

    if wand_timer.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL {
        wand_timer.0.reset();
        let target = get_enemies_in_cone(
            player_transform,
            &enemy_query,
            &cursor_pos.0.unwrap(),
            300.0,
            90.0,
        );
        if let Some(target) = target {
            damage_events.send(DamageEvent {
                target,
                amount: 5.0 * (1.0 + ((player_level.0 - 1) as f32 * 0.05)),
                arcs: 3,
            });

            if let Ok((_, target_transform)) = enemy_query.get(target) {
                draw_vector_path(
                    &mut commands,
                    &mut mesh_assets,
                    &mut material_assets,
                    wand_transform.translation.truncate(),
                    target_transform.translation.truncate(),
                    Color::YELLOW,
                    2.0,
                );
            }
        }
    }
}

// fn update_bullets(mut bullet_query: Query<(&mut Transform, &BulletDirection), With<Lightning>>) {
//     if bullet_query.is_empty() {
//         return;
//     }

//     for (mut t, dir) in bullet_query.iter_mut() {
//         t.translation += dir.0.normalize() * Vec3::splat(BULLET_SPEED);
//         t.translation.z = 10.0;
//     }
// }

fn get_enemies_in_cone(
    player_transform: &Transform,
    enemy_query: &Query<(Entity, &Transform), With<Enemy>>,
    cursor_pos: &Vec2,
    max_distance: f32,
    cone_angle_degrees: f32,
) -> Option<Entity> {
    let player_pos = player_transform.translation.truncate();
    let to_cursor = (*cursor_pos - player_pos).normalize();
    let cone_angle_radians = cone_angle_degrees.to_radians();

    enemy_query
        .iter()
        .filter_map(|(entity, enemy_transform)| {
            let enemy_pos = enemy_transform.translation.truncate();
            let to_enemy = enemy_pos - player_pos;
            let distance = to_enemy.length();

            if distance <= max_distance {
                let angle = to_enemy.normalize().dot(to_cursor).acos();
                if angle <= cone_angle_radians / 2.0 {
                    Some((entity, distance))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(entity, _)| entity)
}

fn get_nearest_enemy_in_radius(
    enemy_query: &Query<(Entity, &Transform), With<Enemy>>,
    position: Vec2,
    radius: f32,
    exclude_entity: Entity,
) -> Option<(Entity, Transform)> {
    enemy_query
        .iter()
        .filter_map(|(entity, enemy_transform)| {
            if entity == exclude_entity {
                return None;
            }

            let enemy_pos = enemy_transform.translation.truncate();
            let distance = enemy_pos.distance(position);

            if distance <= radius {
                Some((entity, distance, enemy_transform.clone()))
            } else {
                None
            }
        })
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(entity, _, enemy_transform)| (entity, enemy_transform))
}

fn draw_vector_path(
    commands: &mut Commands,
    mesh_assets: &mut ResMut<Assets<Mesh>>,
    material_assets: &mut ResMut<Assets<ColorMaterial>>,
    start: Vec2,
    end: Vec2,
    color: Color,
    thickness: f32,
) {
    let vector = end - start;
    let center = start + vector / 2.0;
    let length = vector.length();
    let angle = vector.y.atan2(vector.x);

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: mesh_assets.add(Rectangle::new(length, thickness)).into(),
            material: material_assets.add(ColorMaterial::from(color)),
            transform: Transform::from_translation(Vec3::new(center.x, center.y, 0.0))
                .with_rotation(Quat::from_rotation_z(angle)),
            ..default()
        })
        .insert(LightningEffect { lifetime: 1.4 })
        .insert(Lightning);
}

// fn draw_debug_cone(
//     mut commands: Commands,
//     player_query: Query<&Transform, With<Player>>,
//     cursor_pos: Res<CursorPosition>,
// ) {
//     if let Ok(player_transform) = player_query.get_single() {
//         let player_pos = player_transform.translation.truncate();
//         let cursor_pos = cursor_pos.0.unwrap_or(player_pos);
//         let to_cursor = (cursor_pos - player_pos).normalize();
//         let angle = 90.0_f32.to_radians();
//         let radius = 300.0;

//         let points = (0..=20)
//             .map(|i| {
//                 let t = i as f32 / 20.0;
//                 let current_angle = angle * (t - 0.5);
//                 let rotation = Mat2::from_angle(current_angle);
//                 player_pos + rotation.mul_vec2(to_cursor) * radius
//             })
//             .collect::<Vec<Vec2>>();

//         let shape = shapes::Polygon {
//             points: points.clone(),
//             closed: true,
//         };

//         // commands.spawn(GeometryBuilder::new().add(&shape).build(
//         //     DrawMode::Fill(FillMode::color(Color::rgba(1.0, 1.0, 0.0, 0.2))),
//         //     Transform::from_translation(player_transform.translation + Vec3::new(0.0, 0.0, 5.0)),
//         // ));

//         commands.spawn((
//             ShapeBundle {
//                 path: GeometryBuilder::build_as(&shape),
//                 ..default()
//             },
//             Fill::color(Color::CYAN),
//         ));
//     }
// }

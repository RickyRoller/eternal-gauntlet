use bevy::prelude::*;

use crate::audio::BackgroundMusic;
use crate::player::{Health, Level, Player};
use crate::resources::Score;
use crate::state::GameState;
use crate::world::GameEntity;
use crate::PLAYER_HEALTH;

pub struct GuiPlugin;

#[derive(Component)]
struct DebugText;
#[derive(Component)]
struct MainMenuItem;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct HealthBar;

#[derive(Component)]
struct HealthHeart;

#[derive(Component)]
struct LevelText;

#[derive(Resource)]
struct HeartAssets {
    full: Handle<Image>,
    empty: Handle<Image>,
}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
            .add_systems(OnEnter(GameState::GameInit), load_heart_assets)
            .add_systems(
                Update,
                handle_main_menu_buttons.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnEnter(GameState::GameInit), spawn_debug_text)
            .add_systems(
                Update,
                update_score_text.run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnEnter(GameState::GameInit), spawn_health_bar)
            .add_systems(
                Update,
                update_health_bar.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                update_experience_bar.run_if(in_state(GameState::InGame)),
            );
    }
}

fn load_heart_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let heart_assets = HeartAssets {
        full: asset_server.load("ui_heart_full.png"),
        empty: asset_server.load("ui_heart_empty.png"),
    };
    commands.insert_resource(heart_assets);
}

fn spawn_debug_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TODO: Add a score text
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    padding: UiRect::px(0.0, 20.0, 20.0, 0.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::End,
                    ..default()
                },
                ..default()
            },
            GameEntity,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Score: 0",
                    TextStyle {
                        font: asset_server.load("monogram.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ScoreText,
            ));
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            GameEntity,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(380.0),
                        height: Val::Px(185.0),
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Val::Px(8.0)),
                        margin: UiRect::px(10.0, 10.0, 10.0, 0.0),
                        ..default()
                    },
                    background_color: BackgroundColor::from(Color::BLACK.with_a(0.9)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Hello Bevy!",
                            TextStyle {
                                font: asset_server.load("monogram.ttf"),
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        DebugText,
                    ));
                });
        });
}

fn update_score_text(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if query.is_empty() {
        return;
    }
    let mut text = query.single_mut();
    text.sections[0].value = format!("Score: {}", score.0);
}

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            MainMenuItem,
        ))
        .with_children(|parent| {
            // Splash image
            parent.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    width: Val::Px(1300.0),
                    ..default()
                },
                image: UiImage::new(asset_server.load("splash.png")),
                ..default()
            });

            // Title image
            parent.spawn(ImageBundle {
                style: Style {
                    margin: UiRect::px(0.0, 0.0, 120.0, 0.0),
                    width: Val::Px(800.0), // Adjust size as needed
                    height: Val::Px(119.0),
                    ..default()
                },
                image: UiImage::new(asset_server.load("eternal-gauntlet.png")),
                ..default()
            });

            // Play button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::px(0.0, 0.0, 20.0, 0.0),
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ));
                });
        });
}

fn handle_main_menu_buttons(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut background_music: Query<&mut AudioSink, With<BackgroundMusic>>,
) {
    for interaction in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
                if let Ok(mut sink) = background_music.get_single_mut() {
                    sink.play();
                }
                next_state.set(GameState::GameInit);
            }
            _ => {}
        }
    }
}

fn despawn_main_menu(mut commands: Commands, menu_items_query: Query<Entity, With<MainMenuItem>>) {
    for e in menu_items_query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn spawn_health_bar(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::px(20.0, 0.0, 20.0, 0.0),
                    ..default()
                },
                ..default()
            },
            GameEntity,
        ))
        .with_children(|parent| {
            // Health hearts
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    },
                    HealthBar,
                ))
                .with_children(|parent| {
                    for _ in 0..5 {
                        parent.spawn((
                            ImageBundle {
                                style: Style {
                                    width: Val::Px(32.0),
                                    height: Val::Px(32.0),
                                    margin: UiRect::px(0.0, 5.0, 0.0, 0.0),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("ui_heart_full.png")),
                                ..default()
                            },
                            HealthHeart,
                        ));
                    }
                });

            // Experience bar and level
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(20.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        margin: UiRect::px(0.0, 0.0, 10.0, 0.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Level text
                    parent.spawn((
                        TextBundle::from_section(
                            "Lvl 1",
                            TextStyle {
                                font: asset_server.load("monogram.ttf"),
                                font_size: 24.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::px(10.0, 0.0, 0.0, 0.0),
                            ..default()
                        }),
                        LevelText,
                    ));
                });
        });
}

fn update_health_bar(
    player_query: Query<&Health, With<Player>>,
    mut heart_query: Query<&mut UiImage, With<HealthHeart>>,
    heart_assets: Res<HeartAssets>,
) {
    if let Ok(player_health) = player_query.get_single() {
        let total_hearts = heart_query.iter().count();
        let health_percentage = player_health.0 as f32 / PLAYER_HEALTH as f32;

        for (index, mut heart_image) in heart_query.iter_mut().enumerate() {
            let heart_threshold = (index + 1) as f32 / total_hearts as f32;
            if health_percentage >= heart_threshold {
                heart_image.texture = heart_assets.full.clone();
            } else {
                heart_image.texture = heart_assets.empty.clone();
            }
        }
    }
}

fn update_experience_bar(
    mut level_text_query: Query<&mut Text, With<LevelText>>,
    player_query: Query<&Level, With<Player>>,
) {
    if let Ok(level) = player_query.get_single() {
        // Update level text
        if let Ok(mut text) = level_text_query.get_single_mut() {
            text.sections[0].value = format!("Lvl {}", level.0);
        }
    }
}

use bevy::audio::{AudioSource, PlaybackMode, Volume};
use bevy::prelude::*;

use crate::state::GameState;

pub struct AudioPlugin;

#[derive(Component)]
pub struct BackgroundMusic;

#[derive(Resource)]
pub struct LightningEffectHandle {
    pub handle: Handle<AudioSource>,
}

#[derive(Component)]
pub struct LightningSoundEffect;

#[derive(Resource)]
pub struct LevelUpEffectHandle {
    pub handle: Handle<AudioSource>,
}

#[derive(Component)]
pub struct LevelUpSoundEffect;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup)
            .add_systems(Update, (clean_up_lightning, clean_up_level_up));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/eternal_labyrinth_ext.ogg"),
            settings: PlaybackSettings {
                paused: true,
                mode: PlaybackMode::Loop,
                volume: Volume::new(0.8),
                ..default()
            },
        },
        BackgroundMusic,
    ));

    commands.insert_resource(LightningEffectHandle {
        handle: asset_server.load("audio/lightning.ogg"),
    });

    commands.insert_resource(LevelUpEffectHandle {
        handle: asset_server.load("audio/retro_level_up_sound.ogg"),
    });
}

fn clean_up_lightning(
    mut commands: Commands,
    audio_sink: Query<(Entity, &AudioSink), With<LightningSoundEffect>>,
) {
    for (entity, sink) in audio_sink.iter() {
        if sink.empty() {
            commands.entity(entity).despawn();
        }
    }
}

fn clean_up_level_up(
    mut commands: Commands,
    audio_sink: Query<(Entity, &AudioSink), With<LevelUpSoundEffect>>,
) {
    for (entity, sink) in audio_sink.iter() {
        if sink.empty() {
            commands.entity(entity).despawn();
        }
    }
}

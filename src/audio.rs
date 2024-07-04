use bevy::audio::{AudioSource, PlaybackMode, Volume};
use bevy::prelude::*;

pub struct AudioPlugin;

#[derive(Component)]
pub struct BackgroundMusic;

#[derive(Resource)]
pub struct LightningEffectHandle {
    pub handle: Handle<AudioSource>,
}

#[derive(Component)]
pub struct LightningSoundEffect;

#[derive(Component)]
pub struct LevelUpSoundEffect;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, clean_up);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/eternal_labyrinth_ext.ogg"),
            settings: PlaybackSettings {
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
}

fn clean_up(
    mut commands: Commands,
    audio_sink: Query<(Entity, &AudioSink), With<LightningSoundEffect>>,
) {
    for (entity, sink) in audio_sink.iter() {
        if sink.empty() {
            commands.entity(entity).despawn();
        }
    }
}

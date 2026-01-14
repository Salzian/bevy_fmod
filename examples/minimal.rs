//! This example demonstrates how to use the FmodPlugin to play a sound.
//! Make sure to follow the instructions in the README.md to set up the demo project.

use bevy::prelude::*;
use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // Add your audio banks here to the plugin. Make sure to include at
            // least the master bank and it's strings bank.
            FmodPlugin::new(&[
                "./assets/audio/demo_project/Build/Desktop/Master.bank",
                "./assets/audio/demo_project/Build/Desktop/Master.strings.bank",
                "./assets/audio/demo_project/Build/Desktop/Music.bank",
            ]),
        ))
        .add_systems(Startup, startup)
        .add_systems(PostStartup, play_music)
        .run();
}

#[derive(Component)]
struct MyMusicPlayer;

fn startup(mut commands: Commands, studio: Res<FmodStudio>) -> Result {
    // To play a sound, you need to create an event instance.
    // To create an event instance, you need to get the event description.
    let event_description = studio.get_event("event:/Music/Level 03")?;

    // With the event description, you can create the event instance.
    let event_instance = event_description.create_instance()?;

    // To place the event instance in the world, you need to spawn a AudioSource component.
    commands.spawn(MyMusicPlayer).insert(AudioSource {
        event_instance: event_instance,
        despawn_stop_mode: StopMode::AllowFadeout,
    });

    Ok(())
}

fn play_music(audio_source: Single<&AudioSource, With<MyMusicPlayer>>) -> Result {
    // To play the event instance, you need to start it.
    audio_source.start()?;
    Ok(())
}

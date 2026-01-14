//! Spatial audio:
//! The spatial audio bundles provide all the components necessary for spatial audio.
//! Make sure your sound has a spatializer assigned to it in FMOD Studio.
//!
//! Controls:
//! Use the arrow keys to move around.

use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};
use bevy::prelude::*;
use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FreeCameraPlugin,
            FmodPlugin::new(&[
                "./assets/audio/demo_project/Build/Desktop/Master.bank",
                "./assets/audio/demo_project/Build/Desktop/Master.strings.bank",
                "./assets/audio/demo_project/Build/Desktop/Music.bank",
            ]),
        ))
        .add_systems(Startup, (setup_scene, display_controls))
        .add_systems(PostStartup, play_music)
        .add_systems(Update, orbit_audio_source)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    studio: Res<FmodStudio>,
) -> Result {
    // Plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, -1.0, 0.0),
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera
    commands.spawn(SpatialListenerBundle::default()).insert((
        Camera3d::default(),
        FreeCamera::default(),
        Transform::from_xyz(0.0, 0.0, 4.0),
    ));

    // Audio source: Orbiting cube
    let event_description = studio.get_event("event:/Music/Radio Station")?;

    let audio_source = AudioSource {
        event_instance: event_description.create_instance()?,
        despawn_stop_mode: StopMode::AllowFadeout,
    };

    commands
        .spawn(SpatialAudioBundle::from(audio_source))
        .insert((
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            Transform::from_scale(Vec3::splat(0.2)),
        ));

    Ok(())
}

fn play_music(audio_sources: Single<&AudioSource>) -> Result {
    audio_sources.start()?;
    Ok(())
}

fn orbit_audio_source(
    time: Res<Time>,
    mut audio_sources: Query<&mut Transform, With<AudioSource>>,
) {
    for mut audio_source in audio_sources.iter_mut() {
        audio_source.translation.x = time.elapsed_secs().sin() * 2.0;
        audio_source.translation.z = time.elapsed_secs().cos() * 2.0;
    }
}

fn display_controls(mut commands: Commands) {
    commands.spawn(Text::from(
        "Controls: Use the WASD keys and mouse to move around",
    ));
}

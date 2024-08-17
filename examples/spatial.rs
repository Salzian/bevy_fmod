//! Spatial audio:
//! The spatial audio bundles provide all the components necessary for spatial audio.
//! Make sure your sound has a spatializer assigned to it in FMOD Studio.
//!
//! Controls:
//! Use the arrow keys to move around.

use bevy::prelude::*;
use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::*;
use libfmod::StopMode;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FmodPlugin::new(&[
                "./assets/audio/demo_project/Build/Desktop/Master.bank",
                "./assets/audio/demo_project/Build/Desktop/Master.strings.bank",
                "./assets/audio/demo_project/Build/Desktop/Music.bank",
            ]),
        ))
        .add_systems(Startup, setup_scene)
        .add_systems(PostStartup, play_music)
        .add_systems(Update, orbit_audio_source)
        .add_systems(Update, update_listener)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    studio: Res<FmodStudio>,
) {
    // Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        transform: Transform::from_xyz(0.0, -1.0, 0.0),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    commands
        .spawn(SpatialListenerBundle::default())
        .insert(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 4.0),
            ..default()
        });

    // Audio source: Orbiting cube
    let event_description = studio.get_event("event:/Music/Radio Station").unwrap();

    let audio_source = AudioSource {
        event_instance: event_description.create_instance().unwrap(),
        despawn_stop_mode: StopMode::AllowFadeout,
    };

    commands
        .spawn(SpatialAudioBundle::from(audio_source))
        .insert(PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
            transform: Transform::from_scale(Vec3::splat(0.2)),
            ..default()
        });
}

fn play_music(mut audio_sources: Query<&AudioSource>) {
    audio_sources.single_mut().start().unwrap();
}

fn orbit_audio_source(
    time: Res<Time>,
    mut audio_sources: Query<&mut Transform, With<AudioSource>>,
) {
    for mut audio_source in audio_sources.iter_mut() {
        audio_source.translation.x = time.elapsed_seconds().sin() * 2.0;
        audio_source.translation.z = time.elapsed_seconds().cos() * 2.0;
    }
}

fn update_listener(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut listeners: Query<&mut Transform, With<AudioListener>>,
) {
    let mut transform = listeners.single_mut();

    let speed = 4.;

    if keyboard.pressed(KeyCode::ArrowRight) {
        transform.translation.x += speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        transform.translation.z += speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::ArrowUp) {
        transform.translation.z -= speed * time.delta_seconds();
    }
}

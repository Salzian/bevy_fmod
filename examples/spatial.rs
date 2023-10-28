//! Spatial audio:
//! The spatial audio bundles provide all the components necessary for spatial audio.
//! Make sure your sound has a spatializer assigned to it in FMOD Studio.
//!
//! Controls:
//! Use WASD, Space, Shift and the mouse to move around.

use bevy::prelude::*;
use bevy_fmod::prelude::AudioSource;
use bevy_fmod::prelude::SpatialAudioBundle;
use bevy_fmod::prelude::*;
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FmodPlugin {
                audio_banks_paths: &[
                    "./assets/audio/demo_project/Build/Desktop/Master.bank",
                    "./assets/audio/demo_project/Build/Desktop/Master.strings.bank",
                    "./assets/audio/demo_project/Build/Desktop/Music.bank",
                ],
            },
        ))
        .add_plugins(LookTransformPlugin)
        .add_plugins(FpsCameraPlugin::default())
        .add_systems(Startup, setup_scene)
        .add_systems(PostStartup, play_music)
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
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // Camera
    commands
        .spawn(Camera3dBundle::default())
        .insert(SpatialListenerBundle::default())
        .insert(FpsCameraBundle::new(
            FpsCameraController::default(),
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
    // Audio source
    let event_description = studio.0.get_event("event:/Music/Radio Station").unwrap();

    commands
        .spawn(SpatialAudioBundle::new(event_description))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(-1.0, 0.0, 1.0).with_scale(Vec3::splat(0.2)),
            ..default()
        });
}

fn play_music(mut audio_sources: Query<&AudioSource>) {
    audio_sources.single_mut().play();
}

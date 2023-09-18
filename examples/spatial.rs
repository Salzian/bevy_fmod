//! Spatial audio:
//! Having a TransformBundle on the FmodAudioSource
//! and a FmodListener on the camera (for example) is enough to get the spatial audio working.
//!
//! Controls:
//! Use WASD, Space, Shift and the mouse to move around.
//! Press F to spawn an audio source.
//! Press O, P and T to stop, play and toggle the sounds, respectively.

use bevy::app::App;
use bevy::prelude::{
    default, shape, Assets, AudioSinkPlayback, Camera3dBundle, Color, Commands, Input, KeyCode,
    Mesh, PbrBundle, PointLight, PointLightBundle, Query, Res, ResMut, StandardMaterial, Startup,
    Transform, Update, Vec3,
};
use bevy::DefaultPlugins;
use bevy_fmod::{AudioListener, AudioSource, AudioSourcePlayer, FmodPlugin};

use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FmodPlugin {
                audio_banks_directory: "./demo_project/Build/Desktop",
            },
        ))
        .add_plugins(LookTransformPlugin)
        .add_plugins(FpsCameraPlugin::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (spawn_sound, stop_sound, toggle_sound, play_sound))
        .run();
}

fn spawn_sound(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F) {
        commands.spawn((
            AudioSource::from("event:/return"),
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(-1.0, 0.0, 1.0).with_scale(Vec3::splat(0.2)),
                ..default()
            },
        ));
    }
}

fn stop_sound(query: Query<&AudioSourcePlayer>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::O) {
        for audio_player in query.iter() {
            audio_player.stop();
        }
    }
}

fn play_sound(query: Query<&AudioSourcePlayer>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::P) {
        for audio_player in query.iter() {
            audio_player.play();
        }
    }
}

fn toggle_sound(query: Query<&AudioSourcePlayer>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::T) {
        for audio_player in query.iter() {
            audio_player.toggle();
        }
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
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
    // camera
    commands
        .spawn(Camera3dBundle::default())
        .insert(AudioListener::default())
        .insert(FpsCameraBundle::new(
            FpsCameraController::default(),
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
}

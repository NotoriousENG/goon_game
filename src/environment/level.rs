use crate::{constants::TRACK_LENGTH, environment::skybox::SkyboxPlugin}; // probably a better way to do this, in level for right now since nothing else needs to know about it yet
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_plugin(SkyboxPlugin);
    }
}

const BOARDWALK_LENGTH: f32 = 42.0;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 25000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                (-60.0 as f32).to_radians(),
                (-30.0 as f32).to_radians(),
                0.0,
            ),
            ..default()
        },
        ..default()
    });
    for i in 0..((TRACK_LENGTH + 200.0) / BOARDWALK_LENGTH) as i32 {
        let boardwalk_name = format!("boardwalk_{}", i);
        commands.spawn((
            // spawn a boardwalk for each boardwalk length
            SceneBundle {
                scene: asset_server.load("models/boardwalk/boardwalk.gltf#Scene0"),
                transform: Transform::from_translation(Vec3::new(
                    0.0,
                    0.0,
                    -i as f32 * BOARDWALK_LENGTH,
                )),
                ..default()
            },
            Name::new(boardwalk_name),
        ));
    }
}

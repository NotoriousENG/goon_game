use crate::environment::skybox::SkyboxPlugin; // probably a better way to do this, in level for right now since nothing else needs to know about it yet
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_plugin(SkyboxPlugin);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            SceneBundle {
                scene: asset_server.load("models/level/level.gltf#Scene0"),
                ..default()
            },
            Name::new("Level"),
        ))
        .with_children(|level| {
            level.spawn((
                DirectionalLightBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(
                        EulerRot::XYZ,
                        62.0,
                        0.0,
                        0.0,
                    )),
                    ..default()
                },
                Name::new("Sun"),
            ));
        });
}

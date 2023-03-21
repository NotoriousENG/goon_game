//! A simple 3D scene with light shining over a cube sitting on a plane.
//! https://bevyengine.org/examples/3d/3d-scene/
//!

mod clamp;
mod constants;
mod environment;
mod lanes;
mod obstacles;
mod particles;
mod player;

use crate::environment::level::LevelPlugin;
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_rapier3d::prelude::*;
use obstacles::ObstaclePlugin;
use particles::ParticlePlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(EditorPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(RapierDebugRenderPlugin::default()) // disable hdr to use
        .add_plugin(ObstaclePlugin)
        .add_plugin(ParticlePlugin)
        .run();
}

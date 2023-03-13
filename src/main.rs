//! A simple 3D scene with light shining over a cube sitting on a plane.
//! https://bevyengine.org/examples/3d/3d-scene/
//!

mod camera;
mod clamp;
mod environment;
mod player;

use crate::camera::CameraPlugin;
use crate::environment::level::LevelPlugin;
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}

//! A simple 3D scene with light shining over a cube sitting on a plane.
//! https://bevyengine.org/examples/3d/3d-scene/
//!

mod camera;
mod level;

use crate::camera::CameraPlugin;
use crate::level::LevelPlugin;
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(LevelPlugin)
        .run();
}

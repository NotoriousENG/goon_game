use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    constants::{LANE_FACTOR, TRACK_LENGTH},
    lanes::{Lane, LaneEntity},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, Sensor};
use rand::prelude::*;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

#[derive(Clone, Debug)]
pub struct ObstacleResource {
    pub obstacle_type: ObstacleType,
    pub scene_handle: Handle<Scene>,
    pub collision_h_xyz: Vec3,
    pub collision_offset: Vec3,
}

#[derive(Component)]
pub struct Obstacle {
    pub obstacle_type: ObstacleType,
}

#[derive(Clone, Debug, Copy)]
pub enum ObstacleType {
    Low,
    High,
    Full,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();

    // add obstacle resources
    let filepaths = get_filepaths_of_type(Path::new("assets/models/obstacles"), "gltf");
    let mut obstacle_resources: Vec<ObstacleResource> = Vec::new();

    for filepath in filepaths {
        // ensure the filepath is local to assets/ and in unix format
        let filepath = PathBuf::from(filepath.to_str().unwrap().replace("assets/", ""));
        // get filepath string (add #Scene0 to end)
        let fp_str: String = String::from(filepath.to_str().unwrap()) + "#Scene0";

        let scene_handle = asset_server.load(fp_str);
        let mut collision_h_xyz = Vec3::new(0.0, 0.0, 0.0);
        let mut obstacle_type = ObstacleType::Low;
        let mut collision_offset = Vec3::new(0.0, 0.0, 0.0);

        // figured out by spawning a blender cube and scaling it to the size of the low obstacle, looking at transform in editor
        if filepath.to_str().unwrap().contains("/low/") {
            obstacle_type = ObstacleType::Low;
            collision_h_xyz = Vec3::new(1.0, 1.0, 1.0);
            collision_offset = Vec3::new(0.0, 1.0, 0.0);
        } else if filepath.to_str().unwrap().contains("/high/") {
            obstacle_type = ObstacleType::High;
            collision_h_xyz = Vec3::new(6.96, 1.336, 4.304);
            collision_offset = Vec3::new(0.0, 6.572, 0.0);
        } else if filepath.to_str().unwrap().contains("/full/") {
            collision_h_xyz = Vec3::new(1.66, 3.78, 1.66);
            obstacle_type = ObstacleType::Full;
            collision_offset = Vec3::new(0.0, 3.78, 0.0);
        }
        obstacle_resources.push(ObstacleResource {
            obstacle_type: obstacle_type,
            scene_handle,
            collision_h_xyz,
            collision_offset,
        });
    }

    // spawn an obstacle every 50 meters from origin to 600
    let step = 50;
    for i in 1..(TRACK_LENGTH as i32 / step) {
        // get a random obstacle resource
        let obstacle_resource = obstacle_resources
            .get(rng.gen_range(0..obstacle_resources.len()))
            .unwrap();
        // if the obstacle is low obstacle, get a random count of obstacles 1-3 to spawn, 1-2 if full , 1 if high
        let obstacle_count = match obstacle_resource.obstacle_type {
            ObstacleType::Low => rng.gen_range(1..4),
            ObstacleType::High => 1,
            ObstacleType::Full => rng.gen_range(1..3),
        };

        // get all possibilities of lanes as an array
        let mut lanes: Vec<Lane> = vec![Lane::Left, Lane::Middle, Lane::Right];

        // iterate on the obstacle count
        for _ in 0..obstacle_count {
            let lane_index = rng.gen_range(0..lanes.len());
            // get a random lane
            let lane = lanes.get(lane_index).unwrap();

            // xpos is *lane as i32 as f32 * LANE_FACTOR but 0 if it is a high obstacle
            let x_pos = match obstacle_resource.obstacle_type {
                ObstacleType::Low => *lane as i32 as f32 * LANE_FACTOR,
                ObstacleType::High => 0.0,
                ObstacleType::Full => *lane as i32 as f32 * LANE_FACTOR,
            };

            let obstacle_name = format!(
                "obstacle_{}_{:?}_{:?}",
                i * step,
                obstacle_resource.obstacle_type,
                *lane
            );
            // spawn the obstacle
            commands
                .spawn((
                    SceneBundle {
                        scene: obstacle_resource.scene_handle.clone(),
                        transform: Transform::from_translation(Vec3::new(
                            x_pos,
                            0.0,
                            -1.0 * step as f32 * i as f32,
                        )),
                        ..default()
                    },
                    Obstacle {
                        obstacle_type: obstacle_resource.obstacle_type,
                    },
                    LaneEntity { lane: *lane },
                    Name::new(obstacle_name),
                ))
                .with_children(|obstacle| {
                    obstacle.spawn((
                        TransformBundle::from(Transform::from_translation(
                            obstacle_resource.collision_offset,
                        )),
                        Collider::cuboid(
                            obstacle_resource.collision_h_xyz.x,
                            obstacle_resource.collision_h_xyz.y,
                            obstacle_resource.collision_h_xyz.z,
                        ),
                        Sensor,
                        Name::new("obstacle collider"),
                    ));
                });

            // remove the lane from the possibilities
            lanes.remove(lane_index);
        }
    }
}

fn get_filepaths_of_type(dir: &Path, ext: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                paths.extend(get_filepaths_of_type(&path, ext));
            } else if path.extension().unwrap() == ext {
                // replace \\ with / for windows
                let sanitized_path = PathBuf::from(path.to_str().unwrap().replace("\\", "/"));
                paths.push(sanitized_path);
            }
        }
    }
    paths
}

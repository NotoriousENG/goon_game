use std::time::Duration;

use crate::{
    constants::{LANE_FACTOR, TRACK_LENGTH},
    lanes::LaneEntity,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system((setup_player_once_loaded).after(setup))
            .add_system(keyboard_animation_control)
            .add_system(move_player)
            .add_system(move_player_root)
            .add_system(handle_collision_events);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCollider;

#[derive(Component)]
pub struct PlayerRoot;

#[derive(Resource)]
struct PlayerAnimations(Vec<Handle<AnimationClip>>);
const CAMERA_HEIGHT: f32 = 10.0;
const CAM_Z_DISTANCE: f32 = 10.0;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            SceneBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..default()
            },
            PlayerRoot,
            Name::new("player root"),
        ))
        .with_children(|root| {
            root.spawn((
                SceneBundle {
                    scene: asset_server.load("models/player/goblin_animated.gltf#Scene0"),
                    transform: Transform::from_rotation(Quat::from_rotation_y(
                        std::f32::consts::PI,
                    )),
                    ..default()
                },
                Name::new("player"),
                Player,
                LaneEntity::default(),
            ))
            .with_children(|player| {
                player.spawn((
                    TransformBundle::from(Transform::from_translation(Vec3::new(0.0, 4.3, 0.0))),
                    Collider::cuboid(1.0, 3.52, 1.0),
                    Sensor,
                    Name::new("player collider"),
                    PlayerCollider,
                    RigidBody::Dynamic,
                    GravityScale(0.0), // we just do this in the animation system
                    ActiveEvents::COLLISION_EVENTS,
                ));
            });
            root.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0.0, CAMERA_HEIGHT, CAM_Z_DISTANCE)
                    .looking_at(Vec3::Y * CAMERA_HEIGHT / 1.5, Vec3::Y),
                ..default()
            });
        });

    commands.insert_resource(PlayerAnimations(vec![
        // for whatever reason, the animations are in reverse order so list them backwards here
        asset_server.load("models/player/goblin_animated.gltf#Animation0"), // 0. Idle
        asset_server.load("models/player/goblin_animated.gltf#Animation1"), // 1. Jump
        asset_server.load("models/player/goblin_animated.gltf#Animation2"), // 2. Run
        asset_server.load("models/player/goblin_animated.gltf#Animation3"), // 3. Slide
    ]));
}

// Once the player is loaded, start the animation
fn setup_player_once_loaded(
    animations: Res<PlayerAnimations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {
            player.play(animations.0[2].clone_weak()).repeat();
            *done = true;
        }
    }
}

fn move_player_root(mut player_root: Query<&mut Transform, With<PlayerRoot>>, time: Res<Time>) {
    for mut player_root_transform in player_root.iter_mut() {
        player_root_transform.translation.z -= time.delta_seconds() * 20.0;
        if player_root_transform.translation.z < -TRACK_LENGTH {
            player_root_transform.translation.z = 0.0;
        }
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Transform, &mut LaneEntity), With<Player>>,
) {
    for (mut player_transform, mut lane_entity) in player.iter_mut() {
        // store x direction input
        let x = keyboard_input.just_pressed(KeyCode::D) as i32 as f32
            - keyboard_input.just_pressed(KeyCode::A) as i32 as f32;

        // move the player in the direction of the input vector
        lane_entity.change_lane(x as i32);

        // set x position based on lane
        player_transform.translation.x = lane_entity.lane as i32 as f32 * LANE_FACTOR;
    }
}

fn keyboard_animation_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut animation_player: Query<&mut AnimationPlayer>,
    animation_handles: Res<PlayerAnimations>,
    animation_assets: ResMut<Assets<AnimationClip>>,
    mut current_animation: Local<usize>,
    mut player_collision: Query<&mut Transform, With<PlayerCollider>>,
) {
    if let Ok(mut player) = animation_player.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::W) {
            // jump once then return to run animation
            *current_animation = 1;
            player.play_with_transition(
                animation_handles.0[*current_animation].clone_weak(),
                Duration::from_millis(250),
            );
            player_collision.iter_mut().for_each(|mut transform| {
                transform.scale.y = 0.5;
                transform.translation.y = 6.3;
            });
        }
        if keyboard_input.just_pressed(KeyCode::S) {
            // slide once then return to run animation
            *current_animation = 3;
            player.play_with_transition(
                animation_handles.0[*current_animation].clone_weak(),
                Duration::from_millis(250),
            );
            player_collision.iter_mut().for_each(|mut transform| {
                transform.translation.y = 2.15;
                transform.scale.y = 0.5;
            });
        }

        if let Some(clip) = animation_assets.get(&animation_handles.0[*current_animation]) {
            // return to run animation if not already playing
            if player.elapsed() >= clip.duration() - 0.250 {
                *current_animation = 2;
                player
                    .play_with_transition(
                        animation_handles.0[2].clone_weak(),
                        Duration::from_millis(250),
                    )
                    .repeat();
                player_collision.iter_mut().for_each(|mut transform| {
                    transform.translation.y = 4.3;
                    transform.scale.y = 1.0;
                });
            }
        }
    }
}

fn handle_collision_events(
    query_player_collider: Query<Entity, With<PlayerCollider>>,
    mut query_player_root: Query<&mut Transform, With<PlayerRoot>>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    for contact_event in contact_events.iter() {
        for player_entity in query_player_collider.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &player_entity || h2 == &player_entity {
                    for mut player_root_transform in query_player_root.iter_mut() {
                        player_root_transform.translation.z = 0.0; // actually do something later
                    }
                }
            }
        }
    }
}

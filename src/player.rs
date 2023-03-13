use bevy::prelude::*;

use crate::clamp::Clamp;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(setup_player_once_loaded.after(setup))
            .add_system(move_player);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Clone, Copy)]
pub enum Lane {
    Left = -1,
    Middle = 0,
    Right = 1,
}

#[derive(Component)]
pub struct LaneEntity {
    pub lane: Lane,
}

impl LaneEntity {
    pub fn change_lane(&mut self, direction: i32) {
        // set the lane to the clamp of the current lane + the input
        let next_lane = Clamp::clamp(
            self.lane as i32 + direction,
            Lane::Left as i32,
            Lane::Right as i32,
        );
        self.lane = match next_lane {
            0 => Lane::Middle,
            1 => Lane::Right,
            -1 => Lane::Left,
            _ => panic!("Invalid lane value"),
        };
    }
}

impl Default for LaneEntity {
    fn default() -> Self {
        Self { lane: Lane::Middle }
    }
}

#[derive(Resource)]
struct PlayerAnimations(Vec<Handle<AnimationClip>>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/player/goblin_animated.gltf#Scene0"),
            ..default()
        },
        Name::new("player"),
        Player,
        LaneEntity::default(),
    ));

    commands.insert_resource(PlayerAnimations(vec![
        // for whatever reason, the animations are in reverse order so list them backwards here
        asset_server.load("models/player/goblin_animated.gltf#Animation1"), // 1. Idle
        asset_server.load("models/player/goblin_animated.gltf#Animation0"), // 0. Run
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
            player.play(animations.0[0].clone_weak()).repeat();
            *done = true;
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
        player_transform.translation.x = lane_entity.lane as i32 as f32 * 4.0;
    }
}

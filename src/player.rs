use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(setup_player_once_loaded.after(setup));
    }
}

#[derive(Component)]
pub struct Player;

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

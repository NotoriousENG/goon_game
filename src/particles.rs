use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HanabiPlugin)
            .add_startup_system(setup_fx)
            .add_system(spawn_effect.after(setup_fx));
    }
}

// store the fire effect handle
#[derive(Resource)]
pub struct ParticleEffectHandles {
    pub fire: Handle<EffectAsset>,
}

#[derive(Component)]
pub struct Fire;

fn setup_fx(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("textures/particles/fire1.png");

    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(4.0, 4.0, 0.0, 1.0)); // multiply rgb by 4 for hdr
    color_gradient.add_key(0.3, Vec4::new(4.0, 1.0, 0.0, 1.0));
    color_gradient.add_key(0.7, Vec4::new(4.0, 0.0, 0.0, 1.0));
    color_gradient.add_key(1.0, Vec4::splat(0.0));

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.0, Vec2::splat(1.0));
    size_gradient1.add_key(0.5, Vec2::splat(1.5));
    size_gradient1.add_key(1.0, Vec2::splat(0.0));

    let effect = effects.add(
        EffectAsset {
            name: "Gradient".to_string(),
            capacity: 32768,
            spawner: Spawner::rate(200.0.into()),
            ..Default::default()
        }
        .init(InitPositionSphereModifier {
            center: Vec3::ZERO,
            radius: 0.2,
            dimension: ShapeDimension::Volume,
        })
        .init(InitVelocitySphereModifier {
            center: Vec3::ZERO,
            speed: 0.0.into(),
        })
        .init(InitLifetimeModifier {
            lifetime: 2.0.into(),
        })
        .update(AccelModifier::constant(Vec3::new(0.0, 4.0, 0.0)))
        .render(ParticleTextureModifier {
            texture: texture_handle.clone(),
        })
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient,
        })
        // render as billboard
        .render(BillboardModifier::default())
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient1,
        }),
    );

    // store the effect handle
    commands.insert_resource(ParticleEffectHandles { fire: effect });
}

fn spawn_effect(
    mut commands: Commands,
    effects: Res<ParticleEffectHandles>,
    mut done: Local<bool>,
) {
    if !*done {
        // get the effect asset from
        // spawn example
        commands.spawn((
            Name::new("Gradient"),
            ParticleEffectBundle {
                effect: ParticleEffect::new(effects.fire.clone()),
                transform: Transform::IDENTITY,
                ..Default::default()
            },
            Fire,
        ));
        *done = true;
    }
}

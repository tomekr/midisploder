use crate::consts::*;
use bevy::prelude::*;
use rand::Rng;

// Chordite component
struct Chordite {
    velocity: Vec3,
}

/// In this system we're making use of a Timer, which is the best way to do
/// repeated actions every x seconds in Bevy. We're using the newtype pattern to
/// provide encapsulation, allowing us to distinguish our SpawnTimer from other
/// possible timers.
struct SpawnTimer(Timer);

/// Spawns chordites
fn spawn_chordites(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    window: Res<WindowDescriptor>,
) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let rand_xpos = rng.gen_range(-window.width / 2.0, window.width / 2.0);

    let transform = Transform::from_translation(Vec3::new(rand_xpos, 400.0, 1.0));
    println!("Generating chordite with rand_xpos: {}", rand_xpos);
    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            sprite: Sprite::new(Vec2::new(CHORDITE_SIZE, CHORDITE_SIZE)),
            transform,
            ..Default::default()
        })
        .with(Chordite {
            velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
        });
}

/// Moves the chordites down
fn move_chordites(time: Res<Time>, mut query: Query<(&mut Transform, &Chordite)>) {
    for (mut transform, _arrow) in query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * 200.;
    }
}

pub struct ChorditesPlugin;
impl Plugin for ChorditesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // Initialize Resources
            //.init_resource::<ArrowMaterialResource>()
            .add_resource(SpawnTimer(Timer::from_seconds(0.5, true)))
            // Add systems
            .add_system(spawn_chordites.system())
            .add_system(move_chordites.system());
    }
}

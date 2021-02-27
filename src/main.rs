use bevy::{input::system::exit_on_esc_system, prelude::*};

mod chordites;
use chordites::ChorditesPlugin;
mod consts;

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {
            title: "MIDIsploder!".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0))) // Black background
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ChorditesPlugin)
        .add_system(exit_on_esc_system.system())
        .run();
}

fn setup(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
}

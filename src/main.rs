use bevy::prelude::{App, Camera2dBundle, Commands, FixedTime, Startup};
use bevy::DefaultPlugins;
use bevy_xpbd_2d::prelude::PhysicsPlugins;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .insert_resource(FixedTime::new_from_secs(1. / 60.))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

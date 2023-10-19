use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_xpbd_2d::prelude::PhysicsPlugins;
use paddle::PaddlePlugin;

mod paddle;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins(PaddlePlugin)
        .insert_resource(FixedTime::new_from_secs(1. / 60.))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update() {}

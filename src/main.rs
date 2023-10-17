use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_xpbd_2d::prelude::PhysicsPlugins;
use paddle::setup_paddles;
use paddle::update_paddle;
use paddle::Paddle;

mod paddle;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .insert_resource(FixedTime::new_from_secs(1. / 60.))
        .add_systems(Startup, (setup, setup_paddles))
        .add_systems(Update, (update_paddle, update))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn update(query: Query<&Paddle>) {
    for paddle in &query {
        println!("{:#?}", paddle);
    }
}

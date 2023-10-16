use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_xpbd_2d::prelude::PhysicsPlugins;

mod paddle;

use paddle::Paddle;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .insert_resource(FixedTime::new_from_secs(1. / 60.))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_batch(vec![
        Paddle {
            width: 50,
            height: 200,
            position: Vec2::new(50., 50.),
            rotation: 0.,
            up: KeyCode::W,
            down: KeyCode::S,
        },
        Paddle {
            width: 50,
            height: 200,
            position: Vec2::new(-50., -50.),
            rotation: 0.,
            up: KeyCode::Up,
            down: KeyCode::Down,
        },
    ]);

    commands.spawn(Camera2dBundle::default());
}

fn update(input: Res<Input<KeyCode>>, time: Res<Time>, mut query: Query<&mut Paddle>) {
    for mut paddle in query.iter_mut() {
        paddle.handle_input(&input, time.delta_seconds());
        println!("{:#?}", paddle);
    }
}

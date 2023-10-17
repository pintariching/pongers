use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Paddle {
    pub width: f32,
    pub height: f32,
    pub position: Vec2,
    pub rotation: f32,
    pub up: KeyCode,
    pub down: KeyCode,
}

pub fn setup_paddles(mut commands: Commands) {
    commands.spawn((
        Paddle {
            width: 50.,
            height: 200.,
            position: Vec2::new(50., 50.),
            rotation: 0.,
            up: KeyCode::W,
            down: KeyCode::S,
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50., 200.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(50., 50., 0.)),
            ..default()
        },
    ));
}

pub fn update_paddle(input: Res<Input<KeyCode>>, time: Res<Time>, mut query: Query<&mut Paddle>) {
    for mut paddle in query.iter_mut() {
        handle_paddle_input(&mut paddle, &input, time.delta_seconds());
    }
}

fn handle_paddle_input(paddle: &mut Paddle, input: &Res<Input<KeyCode>>, delta_time: f32) {
    if input.pressed(paddle.up) {
        paddle.position.y += 100. * delta_time;
    }

    if input.pressed(paddle.down) {
        paddle.position.y += -100. * delta_time;
    }
}

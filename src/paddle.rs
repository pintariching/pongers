use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

const PADDLE_WIDTH: f32 = 20.;
const PADDLE_HEIGHT: f32 = 100.;

const PADDLE_LINEAR_ACCEL: f32 = 4000.;
const PADDLE_LINEAR_DAMPING: f32 = 10.;
const PADDLE_MAX_LINEAR_VELOCITY: f32 = 2000.;

const PADDLE_ANGULAR_ACCEL: f32 = 40.;
const PADDLE_ANGULAR_DAMPING: f32 = 10.;
const PADDLE_MAX_ANGULAR_VELOCITY: f32 = 100.;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_paddles)
            .add_systems(Update, update_paddle);
    }
}

#[derive(Bundle)]
struct PaddleBundle {
    paddle: Paddle,
    mesh: ColorMesh2dBundle,
    rigid_body: RigidBody,
    collider: Collider,
    position: Position,
    rotation: Rotation,
    restitution: Restitution,
    velocity: LinearVelocity,
    damping: LinearDamping,
    angular_velocity: AngularVelocity,
    angular_damping: AngularDamping,
}

#[derive(Component)]
struct Paddle {
    width: f32,
    height: f32,
    linear_acceleration: f32,
    angular_acceleration: f32,
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode,
    rotate_plus: KeyCode,
    rotate_minus: KeyCode,
}

fn setup_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(PaddleBundle {
        paddle: Paddle {
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
            linear_acceleration: PADDLE_LINEAR_ACCEL,
            angular_acceleration: PADDLE_ANGULAR_ACCEL,
            up: KeyCode::W,
            down: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            rotate_plus: KeyCode::Q,
            rotate_minus: KeyCode::E,
        },
        mesh: ColorMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(1., 1.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_scale(Vec3::new(PADDLE_WIDTH, PADDLE_HEIGHT, 1.)),
            ..default()
        },
        rigid_body: RigidBody::Dynamic,
        collider: Collider::cuboid(PADDLE_WIDTH, PADDLE_HEIGHT),
        position: Position(Vec2::new(-500., 0.)),
        rotation: Rotation::from_degrees(0.),
        restitution: Restitution::new(1.),
        velocity: LinearVelocity(Vec2::ZERO),
        damping: LinearDamping(PADDLE_LINEAR_DAMPING),
        angular_velocity: AngularVelocity(0.),
        angular_damping: AngularDamping(PADDLE_ANGULAR_DAMPING),
    });

    commands.spawn(PaddleBundle {
        paddle: Paddle {
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
            linear_acceleration: PADDLE_LINEAR_ACCEL,
            angular_acceleration: PADDLE_ANGULAR_ACCEL,
            up: KeyCode::Up,
            down: KeyCode::Down,
            left: KeyCode::Left,
            right: KeyCode::Right,
            rotate_plus: KeyCode::Numpad1,
            rotate_minus: KeyCode::Numpad2,
        },
        mesh: ColorMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(1., 1.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_scale(Vec3::new(PADDLE_WIDTH, PADDLE_HEIGHT, 1.)),
            ..default()
        },
        rigid_body: RigidBody::Dynamic,
        collider: Collider::cuboid(PADDLE_WIDTH, PADDLE_HEIGHT),
        position: Position(Vec2::new(500., 0.)),
        rotation: Rotation::from_degrees(0.),
        restitution: Restitution::new(1.),
        velocity: LinearVelocity(Vec2::ZERO),
        damping: LinearDamping(PADDLE_LINEAR_DAMPING),
        angular_velocity: AngularVelocity(0.),
        angular_damping: AngularDamping(PADDLE_ANGULAR_DAMPING),
    });
}

fn update_paddle(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Paddle, &mut LinearVelocity, &mut AngularVelocity)>,
) {
    for (paddle, mut velocity, mut ang_velocity) in query.iter_mut() {
        handle_linear_velocity(&paddle, &input, &mut velocity, time.delta_seconds());
        handle_angular_velocity(&paddle, &input, &mut ang_velocity, time.delta_seconds());
    }
}

fn handle_linear_velocity(
    paddle: &Paddle,
    input: &Res<Input<KeyCode>>,
    velocity: &mut LinearVelocity,
    delta_time: f32,
) {
    if input.pressed(paddle.up) {
        velocity.y += paddle.linear_acceleration * delta_time;
    }

    if input.pressed(paddle.down) {
        velocity.y -= paddle.linear_acceleration * delta_time;
    }

    if input.pressed(paddle.left) {
        velocity.x -= paddle.linear_acceleration * delta_time;
    }

    if input.pressed(paddle.right) {
        velocity.x += paddle.linear_acceleration * delta_time;
    }

    velocity.0 = velocity.clamp_length_max(PADDLE_MAX_LINEAR_VELOCITY);
}

fn handle_angular_velocity(
    paddle: &Paddle,
    input: &Res<Input<KeyCode>>,
    angular_velocity: &mut AngularVelocity,
    delta_time: f32,
) {
    if input.pressed(paddle.rotate_plus) {
        angular_velocity.0 += paddle.angular_acceleration * delta_time;
    }

    if input.pressed(paddle.rotate_minus) {
        angular_velocity.0 -= paddle.angular_acceleration * delta_time;
    }

    angular_velocity.0 = angular_velocity
        .0
        .clamp(-PADDLE_MAX_ANGULAR_VELOCITY, PADDLE_MAX_ANGULAR_VELOCITY);
}

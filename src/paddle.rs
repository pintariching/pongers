use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

const PADDLE_WIDTH: f32 = 20.;
const PADDLE_HEIGHT: f32 = 100.;
const PADDLE_SPEED: f32 = 200.;
const PADDLE_ROTATION_SPEED: f32 = 180.; // In degrees per second

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
}

#[derive(Component)]
struct Paddle {
    width: f32,
    height: f32,
    speed: f32,
    up: KeyCode,
    down: KeyCode,
    rotate_plus: KeyCode,
    rotate_minus: KeyCode,
    rotation_speed: f32,
}

fn setup_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let position = Vec2::new(-500., 0.);

    commands.spawn(PaddleBundle {
        paddle: Paddle {
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
            speed: PADDLE_SPEED,
            up: KeyCode::W,
            down: KeyCode::S,
            rotate_plus: KeyCode::Q,
            rotate_minus: KeyCode::E,
            rotation_speed: PADDLE_ROTATION_SPEED,
        },
        mesh: ColorMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(1., 1.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_scale(Vec3::new(PADDLE_WIDTH, PADDLE_HEIGHT, 1.)),
            ..default()
        },
        rigid_body: RigidBody::Kinematic,
        collider: Collider::cuboid(PADDLE_WIDTH, PADDLE_HEIGHT),
        position: Position(position),
        rotation: Rotation::from_degrees(0.),
        restitution: Restitution::new(1.),
    });
}

fn update_paddle(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Paddle, &mut Position, &mut Rotation)>,
) {
    for (paddle, mut position, mut rotation) in query.iter_mut() {
        handle_paddle_input(
            paddle,
            &mut position,
            &mut rotation,
            &input,
            time.delta_seconds(),
        );

        // let mut new_transform = Transform::from_translation(paddle.position.extend(0.));
        // new_transform.rotate(Quat::from_axis_angle(
        //     Vec3::new(0., 0., 1.),
        //     paddle.rotation,
        // ));

        //*mesh_transform = new_transform.with_scale(Vec3::new(paddle.width, paddle.height, 1.));

        // *mesh_handle = meshes
        //     .add(shape::Quad::new(Vec2::new(paddle.width, paddle.height)).into())
        //     .into();
    }
}

fn handle_paddle_input(
    paddle: &Paddle,
    position: &mut Position,
    rotation: &mut Rotation,
    input: &Res<Input<KeyCode>>,
    delta_time: f32,
) {
    if input.pressed(paddle.up) {
        position.y += paddle.speed * delta_time;
    }

    if input.pressed(paddle.down) {
        position.y -= paddle.speed * delta_time;
    }

    if input.pressed(paddle.rotate_plus) {
        let r = rotation.as_degrees();
        *rotation = Rotation::from_degrees(r + paddle.rotation_speed * delta_time);
    }

    if input.pressed(paddle.rotate_minus) {
        let r = rotation.as_degrees();
        *rotation = Rotation::from_degrees(r - paddle.rotation_speed * delta_time);
    }

    // if input.pressed(KeyCode::F) {
    //     paddle.height += 0.5 * delta_time;
    // }

    // if input.pressed(KeyCode::G) {
    //     paddle.height -= 0.5 * delta_time;
    // }
}

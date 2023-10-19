use bevy::prelude::*;

const PADDLE_WIDTH: f32 = 20.;
const PADDLE_HEIGHT: f32 = 100.;
const PADDLE_SPEED: f32 = 200.;
const PADDLE_ROTATION_SPEED: f32 = 3.; // Radians! - Roughly half a rotation per second.

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_paddles)
            .add_systems(Update, update_paddle);
    }
}

#[derive(Bundle)]
pub struct PaddleBundle {
    pub paddle: Paddle,
    pub mesh: ColorMesh2dBundle,
}

#[derive(Component)]
pub struct Paddle {
    pub width: f32,
    pub height: f32,
    pub position: Vec3,
    pub rotation: f32,
    pub speed: f32,
    pub up: KeyCode,
    pub down: KeyCode,
    pub rotate_plus: KeyCode,
    pub rotate_minus: KeyCode,
    pub rotation_speed: f32,
    pub scale: Vec3,
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
            position: Vec3::new(-500., 0., 0.),
            rotation: 0.,
            speed: PADDLE_SPEED,
            up: KeyCode::W,
            down: KeyCode::S,
            rotate_plus: KeyCode::Q,
            rotate_minus: KeyCode::E,
            rotation_speed: PADDLE_ROTATION_SPEED,
            scale: Vec3::ONE,
        },
        mesh: ColorMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(50., 50., 0.)),
            ..default()
        },
    });
}

fn update_paddle(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Paddle, &mut Transform)>,
) {
    for (mut paddle, mut transform) in query.iter_mut() {
        handle_paddle_input(&mut paddle, &input, time.delta_seconds());

        let mut new_transform = Transform::from_translation(paddle.position);
        new_transform.rotate(Quat::from_axis_angle(
            Vec3::new(0., 0., 1.),
            paddle.rotation,
        ));

        *transform = new_transform.with_scale(paddle.scale);

        // *mesh_handle = meshes
        //     .add(shape::Quad::new(Vec2::new(paddle.width, paddle.height)).into())
        //     .into();
    }
}

fn handle_paddle_input(paddle: &mut Paddle, input: &Res<Input<KeyCode>>, delta_time: f32) {
    if input.pressed(paddle.up) {
        paddle.position.y += paddle.speed * delta_time;
    }

    if input.pressed(paddle.down) {
        paddle.position.y -= paddle.speed * delta_time;
    }

    if input.pressed(paddle.rotate_plus) {
        paddle.rotation += paddle.rotation_speed * delta_time;
    }

    if input.pressed(paddle.rotate_minus) {
        paddle.rotation -= paddle.rotation_speed * delta_time;
    }

    if input.pressed(KeyCode::F) {
        paddle.scale.y += 0.5 * delta_time;
    }

    if input.pressed(KeyCode::G) {
        paddle.scale.y -= 0.5 * delta_time;
    }
}

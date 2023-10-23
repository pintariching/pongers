use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ball)
            .add_systems(Update, reset_ball);
    }
}

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    mesh: ColorMesh2dBundle,
    rigid_body: RigidBody,
    collider: Collider,
    position: Position,
    velocity: LinearVelocity,
    restitution: Restitution,
    gravity_scale: GravityScale,
    locked_axes: LockedAxes,
}

#[derive(Component)]
pub struct Ball {}

fn setup_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let position = Vec2::new(0., 0.);

    commands.spawn(BallBundle {
        ball: Ball {},
        mesh: ColorMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)).into(),
            ..default()
        },
        rigid_body: RigidBody::Dynamic,
        collider: Collider::ball(10.),
        position: Position(position),
        velocity: LinearVelocity(Vec2::new(-200., 10.)),
        restitution: Restitution::new(1.),
        gravity_scale: GravityScale(0.),
        locked_axes: LockedAxes::new().lock_rotation(),
    });
}

fn reset_ball(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&Ball, &mut Position, &mut LinearVelocity)>,
) {
    if input.pressed(KeyCode::Space) {
        let (_ball, mut position, mut velocity) = query.get_single_mut().unwrap();

        *position = Position(Vec2::new(0., 0.));
        *velocity = LinearVelocity(Vec2::new(-200., 10.));
    }
}

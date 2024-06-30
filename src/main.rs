use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume};

const BALL_SIZE: f32 = 5.0;

const PADDLE_SPEED: f32 = 1.0;
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 50.0;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Shape(Vec2);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    shape: Shape,
    velocity: Velocity,
    position: Position,
}

#[derive(Bundle)]
struct PaddleBundle {
    paddle: Paddle,
    shape: Shape,
    velocity: Velocity,
    position: Position,
}

impl BallBundle {
    fn new(x: f32, y:f32) -> Self {
        Self {
            ball: Ball,
            shape: Shape(Vec2::new(BALL_SIZE, BALL_SIZE)),
            velocity: Velocity(Vec2::new(x, y)),
            position: Position(Vec2::new(0.0, 0.0)),
        }
    }
}

impl PaddleBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            paddle: Paddle,
            shape: Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            velocity: Velocity(Vec2::new(0.0, 0.0)),
            position: Position(Vec2::new(x, y)),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_ball, spawn_paddle))
        .add_systems(Update, (
            move_ball,
            project_positions.after(move_ball),
            handle_collisions.after(move_ball),
        ))
        .run();
}
fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty()
        .insert(Camera2dBundle::default());
}
fn spawn_ball(mut commands: Commands,
              mut meshs: ResMut<Assets<Mesh>>,
              mut materials: ResMut<Assets<ColorMaterial>>) {

    let shape = Mesh::from(Circle::new(BALL_SIZE));
    let material = ColorMaterial::from(Color::RED);

    let mesh_handle = meshs.add(shape);
    let material_handle = materials.add(material);

    println!("Spawning ball");
    commands.spawn((
        BallBundle::new(1., 0.),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..Default::default()
    }));
}

fn spawn_paddle(mut commands: Commands,
                mut meshs: ResMut<Assets<Mesh>>,
                mut materials: ResMut<Assets<ColorMaterial>>) {

    let shape = Mesh::from(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT));
    let material = ColorMaterial::from(Color::GREEN);

    let mesh_handle = meshs.add(shape);
    let material_handle = materials.add(material);

    println!("Spawning paddle");
    commands.spawn((
        PaddleBundle::new(20., -25.),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            ..Default::default()
    }));
}

fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0;
    }
}

fn project_positions(
    mut positionables: Query<(&mut Transform, &Position)>
) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

fn collide_with_side(
    ball: BoundingCircle,
    wall: Aabb2d,
) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closet_point = wall.closest_point(ball.center());
    let offset = ball.center() - closet_point;

    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x > 0. {
            Collision::Right
        } else {
            Collision::Left
        }
    } else {
        if offset.y > 0. {
            Collision::Top
        } else {
            Collision::Bottom
        }
    };

    Some(side)
}

fn handle_collisions(
    mut ball: Query<(&mut Position, &mut Velocity, &Shape), With<Ball>>,
    other_things: Query<(&Position, &Shape), Without<Ball>>,
) {
    if let Ok((
    mut ball_velocity,
    ball_position,
    ball_shape)) = ball.get_single_mut() {
        for (other_position, other_shape) in &other_things {
            if let Some(collision) = collide_with_side(
                BoundingCircle::new(ball_position.0, ball_shape.0.x),
                Aabb2d::new(other_position.0, other_shape.0 / 2.),
            ) {
                match collision {
                    Collision::Left | Collision::Right => {
                        ball_velocity.0.x *= -1.;
                    }
                    Collision::Top | Collision::Bottom => {
                        ball_velocity.0.y *= -1.;
                    }
                }
            }
        }
    }
}
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

const BALL_SIZE: f32 = 5.0;

const PADDLE_SPEED: f32 = 1.0;
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 50.0;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Paddle;

#[derive(Bundle)]
struct BallBundle {
    position: Position,
    ball: Ball,
}

#[derive(Bundle)]
struct PaddleBundle {
    position: Position,
    paddle: Paddle,
}

impl BallBundle {
    fn new() -> Self {
        Self {
            position: Position(Vec2::new(0.0, 0.0)),
            ball: Ball,
        }
    }
}

impl PaddleBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            position: Position(Vec2::new(x, y)),
            paddle: Paddle,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_ball, spawn_paddle))
        .add_systems(Update, move_ball)
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
    commands.spawn((BallBundle::new(), MaterialMesh2dBundle {
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

fn move_ball(mut ball: Query<&mut Position, With<Ball>>) {
    if let Ok(mut position) = ball.get_single_mut() {
        position.0.x += 1.0;
    }
}

fn project_positions(
    mut positionables: Query<(&mut Transform, &Position)>
) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume};
use bevy::prelude::KeyCode::{ArrowDown, ArrowUp};

const BALL_SIZE: f32 = 5.0;

const PADDLE_SPEED: f32 = 5.0;
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 50.0;

const GUTTER_HEIGHT: f32 = 20.0;

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

#[derive(Component)]
struct Gutter;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ai;

#[derive(Component)]
struct PlayerScore;

#[derive(Component)]
struct AiScore;

enum Scorer {
    Player,
    Ai,
}

#[derive(Event)]
struct Scored(Scorer);

#[derive(Resource, Default)]
struct Score {
    player: u32,
    ai: u32,
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

#[derive(Bundle)]
struct GutterBundle {
    gutter: Gutter,
    shape: Shape,
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

impl GutterBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            gutter: Gutter,
            shape: Shape(Vec2::new(0.0, GUTTER_HEIGHT)),
            position: Position(Vec2::new(x, y)),
        }
    }
}

fn detect_scoring(
    mut ball: Query<&mut Position, With<Ball>>,
    window: Query<&Window>,
    mut events: EventWriter<Scored>,
) {
    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();

        if let Ok(ball) = ball.get_single_mut() {
            // Here we write the events using our EventWriter
            if ball.0.x > window_width / 2. {
                events.send(Scored(Scorer::Ai));
            } else if ball.0.x < -window_width / 2. {
                events.send(Scored(Scorer::Player));
            }
        }
    }
}

fn reset_ball(
    mut ball: Query<(&mut Position, &mut Velocity), With<Ball>>,
    mut events: EventReader<Scored>,
) {
    for event in events.read() {
        if let Ok((
                      mut position,
                      mut velocity
                  )) = ball.get_single_mut() {
            match event.0 {
                Scorer::Ai => {
                    position.0 = Vec2::new(0., 0.);
                    velocity.0 = Vec2::new(-1., 1.);
                }
                Scorer::Player => {
                    position.0 = Vec2::new(0., 0.);
                    velocity.0 = Vec2::new(1., 1.);
                }
            }
        }
    }
}


fn update_score(
    mut score: ResMut<Score>,
    mut events: EventReader<Scored>
) {
    for event in events.read() {
        match event.0 {
            Scorer::Ai => score.ai += 1,
            Scorer::Player => score.player += 1,
        }
    }

    println!("Score: {} - {}", score.player, score.ai);
}

#[derive(Component)]
struct PlayerScoreboard;

#[derive(Component)]
struct AiScoreboard;

fn update_scoreboard(
    mut player_score: Query<
        &mut Text,
        With<PlayerScoreboard>
    >,
    mut ai_score: Query<
        &mut Text,
        (With<AiScoreboard>, Without<PlayerScoreboard>)
    >,
    score: Res<Score>,
) {
    if score.is_changed() {
        if let Ok(mut player_score) = player_score.get_single_mut() {
            player_score.sections[0].value = score.player.to_string();
        }

        if let Ok(mut ai_score) = ai_score.get_single_mut() {
            ai_score.sections[0].value = score.ai.to_string();
        }
    }
}

fn spawn_scoreboard(
    mut commands: Commands,
) {
    commands.spawn((
        // Create a TextBundle that has a Text with a
        // single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts
            // into a `String`, such as `&str`
            "0",
            TextStyle {
                font_size: 72.0,
                color: Color::WHITE,
                ..default()
            },
        )
            .with_text_justify(JustifyText::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            }),
        PlayerScore
    ));

    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 72.0,
                color: Color::WHITE,
                ..default()
            },
        )
            .with_text_justify(JustifyText::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            }),
        AiScore
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .add_event::<Scored>()
        .add_systems(Startup, (
            spawn_camera,
            spawn_ball,
            spawn_paddles,
            spawn_gutter,
            spawn_scoreboard,
        ))
        .add_systems(Update, (
            move_ball,
            handle_player_input,
            move_ai,
            detect_scoring,
            reset_ball.after(detect_scoring),
            update_score.after(detect_scoring),
            move_paddles.after(handle_player_input),
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

fn spawn_paddles(mut commands: Commands,
                mut meshs: ResMut<Assets<Mesh>>,
                mut materials: ResMut<Assets<ColorMaterial>>,
                window: Query<&Window>
) {

    println!("Spawning paddle");

    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();
        let padding = 50.;
        let right_paddle_x = window_width / 2. - padding;
        let left_paddle_x = -window_width / 2. + padding;

        let mesh = Mesh::from(Rectangle::new(
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        ));

        let mesh_handle = meshs.add(mesh);

        commands.spawn((
            Player,
            PaddleBundle::new(right_paddle_x, 0.),
            MaterialMesh2dBundle {
                mesh: mesh_handle.clone().into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                ..Default::default()
            }
        ));

    commands.spawn((
        Ai,
        PaddleBundle::new(left_paddle_x, 0.),
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            ..Default::default()
    }));
    }
}

fn spawn_gutter(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    if let Ok(window) = window.get_single() {
        let window_width = window.resolution.width();
        let window_height = window.resolution.height();
        let padding = 50.;

        let top_gutter = GutterBundle::new(0., window_height / 2. - GUTTER_HEIGHT / 2.);
        let bottom_gutter = GutterBundle::new(0., -window_height / 2. + GUTTER_HEIGHT / 2.);

        let mesh = Mesh::from(Rectangle::from_size(
            top_gutter.shape.0
        ));
        let material = ColorMaterial::from(Color::BLACK);

        let mesh_handle = meshs.add(mesh);
        let material_handle = materials.add(material);

        commands.spawn((
            top_gutter,
            MaterialMesh2dBundle {
                mesh: mesh_handle.clone().into(),
                material: material_handle.clone(),
                ..Default::default()
            }
        ));

        commands.spawn((
            bottom_gutter,
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle.clone(),
                ..Default::default()
            }
        ));
    }
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

fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest = wall.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}

fn handle_collisions(
    mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>,
    other_things: Query<(&Position, &Shape), Without<Ball>>,
) {
    if let Ok((mut ball_velocity, ball_position, ball_shape)) = ball.get_single_mut() {
        for (position, shape) in &other_things {
            if let Some(collision) = collide_with_side(
                BoundingCircle::new(ball_position.0, ball_shape.0.x),
                Aabb2d::new(
                    position.0,
                    shape.0 / 2.
                )
            ) {
                match collision {
                    Collision::Left => {
                        ball_velocity.0.x *= -1.;
                    }
                    Collision::Right => {
                        ball_velocity.0.x *= -1.;
                    }
                    Collision::Top => {
                        ball_velocity.0.y *= -1.;
                    }
                    Collision::Bottom => {
                        ball_velocity.0.y *= -1.;
                    }
                }
            }
        }
    }
}

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = paddle.get_single_mut() {
        if keyboard_input.pressed(ArrowUp) {
            velocity.0.y = 1.;
        } else if keyboard_input.pressed(ArrowDown) {
            velocity.0.y = -1.;
        } else {
            velocity.0.y = 0.;
        }
    }
}

fn move_paddles(
    mut paddle: Query<(&mut Position, &Velocity), With<Paddle>>,
    window: Query<&Window>
) {
    if let Ok(window) = window.get_single() {
        let window_height = window.resolution.height();
        let max_y = window_height / 2. - GUTTER_HEIGHT - PADDLE_HEIGHT / 2.;

        for (mut position, velocity) in &mut paddle {
            let new_position = position.0 + velocity.0 * PADDLE_SPEED;
            if new_position.y.abs() < max_y {
                position.0 = new_position;
            }
        }
    }
}

fn move_ai(
    mut ai: Query<(&mut Velocity, &Position), With<Ai>>,
    ball: Query<&Position, With<Ball>>,
) {
    if let Ok((mut velocity, position)) = ai.get_single_mut() {
        if let Ok(ball_position) = ball.get_single() {
            let a_to_b = ball_position.0 - position.0;
            velocity.0.y = a_to_b.y.signum();
        }
    }
}
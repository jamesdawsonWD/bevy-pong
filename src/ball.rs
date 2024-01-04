use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::Audio;

use crate::{
    player::{Paddle, PlayerType, PADDLE_HEIGHT},
    ARENA_HEIGHT, ARENA_WIDTH, audio::{play_bounce, play_hit},
};

pub const BALL_SIZE: f32 = 10.0;
const BALL_START_POSITION_PLAYER_2: Vec3 = Vec3::new(-300.0, 0.0, 0.0);
const BALL_START_POSITION_PLAYER_1: Vec3 = Vec3::new(300.0, 0.0, 0.0);
const INITIAL_BALL_SPEED: f32 = 200.0;
const SPEED_INCREMENT: f32 = 50.0;

#[derive(Component)]
pub struct Ball {
    velocity: Vec3,
    fired: bool,
    owner: PlayerType,
}

#[derive(Resource)]
pub struct LastOwner {
    owner: PlayerType,
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LastOwner {
            owner: PlayerType::Player1,
        })
        .add_systems(Startup, spawn_ball)
        .add_systems(Update, move_ball);
    }
}
fn move_ball(
    mut ball_query: Query<(&mut Ball, &mut Transform), Without<Paddle>>,
    paddle_query: Query<(&Paddle, &Transform)>,
    asset_server: Res<AssetServer>, audio: Res<Audio>,
    lastOwner: Res<LastOwner>,
    time: Res<Time>,
) {
    let (mut ball, mut ball_transform) = ball_query.single_mut();

    if !ball.fired {
        for (paddle, paddle_transform) in paddle_query.iter() {
            match lastOwner.owner {
                PlayerType::Player1 => {
                    match paddle.player_type {
                        PlayerType::Player1 => {
                            // Increment velocity for Paddle 1
                            ball_transform.translation.x = paddle_transform.translation.x + 10.0;
                            ball_transform.translation.y = paddle_transform.translation.y + 2.0;
                        }
                        PlayerType::Player2 => {}
                    }
                }
                PlayerType::Player2 => {
                    match paddle.player_type {
                        PlayerType::Player1 => {}
                        PlayerType::Player2 => {
                            // Decrement velocity for Paddle 2
                            ball_transform.translation.x = paddle_transform.translation.x - 10.0;
                            ball_transform.translation.y = paddle_transform.translation.y - 2.0;
                        }
                    }
                }
            }
        }

        return;
    }

    ball_transform.translation += ball.velocity * time.delta_seconds();

    // Check for collision with the top and bottom walls
    if ball_transform.translation.y > ARENA_HEIGHT / 2.0 - BALL_SIZE / 2.0 {
        play_hit(&asset_server, &audio);
        ball_transform.translation.y = ARENA_HEIGHT / 2.0 - BALL_SIZE / 2.0;
        ball.velocity.y = -ball.velocity.y;
    } else if ball_transform.translation.y < -ARENA_HEIGHT / 2.0 + BALL_SIZE / 2.0 {
        play_hit(&asset_server, &audio);
        ball_transform.translation.y = -ARENA_HEIGHT / 2.0 + BALL_SIZE / 2.0;
        ball.velocity.y = -ball.velocity.y;
    }

    for (paddle, paddle_transform) in paddle_query.iter() {
        if check_paddle_collision(&ball_transform, paddle_transform) {
            ball.velocity.x = -ball.velocity.x;

            match paddle.player_type {
                PlayerType::Player1 => {
                    // Increment velocity for Paddle 1
                    ball.velocity += Vec3::new(SPEED_INCREMENT, 0.0, 0.0);

                    // Determine the paddle's movement direction
                    bounce_ball(&mut ball, paddle, paddle_transform);

                    play_bounce(&asset_server, &audio);
                }
                PlayerType::Player2 => {
                    // Decrement velocity for Paddle 2
                    ball.velocity -= Vec3::new(SPEED_INCREMENT, 0.0, 0.0);

                    bounce_ball(&mut ball, paddle, paddle_transform);

                    play_bounce(&asset_server, &audio);
                }
            }
            break; // Break after detecting any collision to avoid double bounces
        }
    }
}

pub fn fire_ball(ball: &mut Ball) {
    ball.fired = true;
}

fn bounce_ball(ball: &mut Ball, paddle: &Paddle, paddle_transform: &Transform) {
    let y_velocity_adjustment = 0.5 * paddle.speed; // Adjust this factor as needed

    if paddle.y_velocity > 0.0 {
        // Paddle moving up
        ball.velocity.y += y_velocity_adjustment;
    } else if paddle.y_velocity < 0.0 {
        // Paddle moving down
        ball.velocity.y -= y_velocity_adjustment;
    }
}

// Helper function to check collision with a paddle
fn check_paddle_collision(ball_transform: &Transform, paddle_transform: &Transform) -> bool {
    // Define the paddle size (adjust these values according to your paddle size)
    let paddle_size = Vec2::new(10.0, PADDLE_HEIGHT);

    let ball_pos = ball_transform.translation;
    let paddle_pos = paddle_transform.translation;

    // Check for overlap in x and y axes
    ball_pos.x + BALL_SIZE / 2.0 > paddle_pos.x - paddle_size.x / 2.0
        && ball_pos.x - BALL_SIZE / 2.0 < paddle_pos.x + paddle_size.x / 2.0
        && ball_pos.y + BALL_SIZE / 2.0 > paddle_pos.y - paddle_size.y / 2.0
        && ball_pos.y - BALL_SIZE / 2.0 < paddle_pos.y + paddle_size.y / 2.0
}

pub fn spawn_ball(mut commands: Commands, mut lastOwner: ResMut<LastOwner>) {
    match lastOwner.owner {
        PlayerType::Player1 => {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(-1000.0, -1000.0, 0.0),
                        ..default()
                    },
                    ..default()
                },
                Ball {
                    velocity: Vec3::new(INITIAL_BALL_SPEED, 0.0, 0.0),
                    fired: false,
                    owner: PlayerType::Player2,
                },
            ));

            lastOwner.owner = PlayerType::Player2;
        }
        PlayerType::Player2 => {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(-1000.0, -1000.0, 0.0),
                        ..default()
                    },
                    ..default()
                },
                Ball {
                    velocity: Vec3::new(INITIAL_BALL_SPEED, 0.0, 0.0),
                    fired: false,
                    owner: PlayerType::Player1,
                },
            ));
            lastOwner.owner = PlayerType::Player1;
        }
    }
}

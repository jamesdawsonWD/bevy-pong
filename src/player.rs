use bevy::prelude::*;

use crate::{
    ball::{fire_ball, Ball},
    ARENA_HEIGHT,
};

pub struct PlayerPlugin;

const PLAYER_SPEED: f32 = 300.0;
pub const PADDLE_HEIGHT: f32 = 60.0;

#[derive(Component)]
pub enum PlayerType {
    Player1,
    Player2,
}

#[derive(Component)]
pub struct Paddle {
    pub speed: f32,
    pub player_type: PlayerType,
    pub y_velocity: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_players)
            // .add_systems(Update, camera_follow.after(player_movement))
            .add_systems(Update, (player_movement));
    }
}

fn spawn_players(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10.0, PADDLE_HEIGHT)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(-300.0, 0.0, 0.0)),
            ..Default::default()
        },
        Paddle {
            speed: PLAYER_SPEED,
            player_type: PlayerType::Player1,
            y_velocity: 0.0,
        },
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10.0, PADDLE_HEIGHT)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(300.0, 0.0, 0.0)),
            ..Default::default()
        },
        Paddle {
            speed: PLAYER_SPEED,
            player_type: PlayerType::Player2,
            y_velocity: 0.0,
        },
    ));
    // commands.spawn((
    //     SpriteBundle {
    //         sprite: Sprite {
    //             custom_size: Some(Vec2::new(32.0, 32.0)),
    //             ..default()
    //         },
    //         texture: character_handle.clone(),
    //         transform: Transform {
    //             translation: Vec3 {
    //                 x: 2 as f32 * TILE_SIZE,
    //                 y: -3 as f32 * TILE_SIZE,
    //                 z: 2.0,
    //             },
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     Player {
    //         speed: 6.0,
    //         grounded: true,
    //         y_velocity: -1.0,
    //         jump_strength: 6.0,
    //     },
    // ));

    // commands.spawn((
    //     SpriteBundle {
    //         sprite: Sprite {
    //             custom_size: Some(Vec2::new(64.0, 64.0)),
    //             ..default()
    //         },
    //         texture: enemy_handle.clone(),
    //         transform: Transform {
    //             translation: Vec3 {
    //                 x: 14 as f32 * TILE_SIZE,
    //                 y: -8 as f32 * TILE_SIZE,
    //                 z: 2.0,
    //             },
    //             ..default()
    //         },
    //         ..default()
    //     },

    // ));
}

// fn wall_collision_check(
//     target_player_pos: Vec3,
//     wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
// ) -> bool {
//     for wall_transform in wall_query.iter() {
//         let collision = collide(
//             target_player_pos,
//             Vec2::splat(TILE_SIZE * 0.9),
//             wall_transform.translation,
//             Vec2::splat(TILE_SIZE),
//         );
//         if collision.is_some() {
//             return false;
//         }
//     }
//     true
// }
fn player_movement(
    mut players_query: Query<(&mut Paddle, &mut Transform)>,
    mut ball_query: Query<(&mut Ball, &mut Transform), Without<Paddle>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut paddle, mut transform) in players_query.iter_mut() {
        let mut y_delta = 0.0;

        let (mut ball, mut _ball_transform) = ball_query.single_mut();

        if keyboard.pressed(KeyCode::Space) {
            fire_ball(&mut *ball);
        }

        match paddle.player_type {
            PlayerType::Player1 => {
                if keyboard.pressed(KeyCode::W) {
                    y_delta += paddle.speed * time.delta_seconds();
                }
                if keyboard.pressed(KeyCode::S) {
                    y_delta -= paddle.speed * time.delta_seconds();
                }
            }
            PlayerType::Player2 => {
                if keyboard.pressed(KeyCode::Up) {
                    y_delta += paddle.speed * time.delta_seconds();
                }
                if keyboard.pressed(KeyCode::Down) {
                    y_delta -= paddle.speed * time.delta_seconds();
                }
            }
        }
        // Calculate the new y position
        let new_y = transform.translation.y + y_delta;

        // Clamp the y position to be within the arena bounds
        transform.translation.y = new_y.clamp(
            -ARENA_HEIGHT / 2.0 + PADDLE_HEIGHT / 2.0,
            ARENA_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
        );

        paddle.y_velocity = y_delta / time.delta_seconds();
    }
}

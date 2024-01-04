use crate::{
    audio::play_win,
    ball::{spawn_ball, Ball, LastOwner, BALL_SIZE},
    player::{Paddle, PlayerType},
    ARENA_HEIGHT, ARENA_WIDTH,
};
use bevy::a11y::accesskit::{NodeBuilder, Role};
use bevy::prelude::*;
use bevy_kira_audio::Audio;

const GOAL_WIDTH: f32 = 100.0;

#[derive(Resource)]

struct Score {
    player1_score: u32,
    player2_score: u32,
}

#[derive(Component)]
pub enum GoalFor {
    Player1,
    Player2,
}

#[derive(Component)]
pub struct Goal;

impl Default for Score {
    fn default() -> Self {
        Score {
            player1_score: 0,
            player2_score: 0,
        }
    }
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score::default())
            .add_systems(Startup, (setup_ui, spawn_goals))
            .add_systems(Update, (check_goal_collision));
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load a font
    // root node
    let font = asset_server.load("fonts/Minecraft.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::Left;
    let text_alignment = TextAlignment::Left;

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("00", text_style.clone()).with_alignment(text_alignment),
            transform: Transform::from_translation(Vec3::new(
                -40.0,
                ARENA_HEIGHT / 2.0 + 50.0,
                1.0,
            )),

            ..default()
        },
        PlayerType::Player1,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("00", text_style.clone()).with_alignment(text_alignment),
            // ensure the text is drawn on top of the box
            transform: Transform::from_translation(Vec3::new(40.0, ARENA_HEIGHT / 2.0 + 50.0, 1.0)),
            ..default()
        },
        PlayerType::Player2,
    ));
}

fn spawn_goals(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::Hsla {
                    hue: (0.0),
                    saturation: (0.0),
                    lightness: (0.0),
                    alpha: (0.0),
                },
                custom_size: Some(Vec2::new(GOAL_WIDTH, ARENA_HEIGHT)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-ARENA_WIDTH / 2.0 - GOAL_WIDTH * 2.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        },
        Goal,
        GoalFor::Player1,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::Hsla {
                    hue: (0.0),
                    saturation: (0.0),
                    lightness: (0.0),
                    alpha: (0.0),
                },
                custom_size: Some(Vec2::new(GOAL_WIDTH, ARENA_HEIGHT)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(ARENA_WIDTH / 2.0 + GOAL_WIDTH * 2.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        },
        Goal,
        GoalFor::Player2,
    ));
}
fn check_goal_collision(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &Transform), With<Ball>>,
    lastOwner: ResMut<LastOwner>,
    goal_query: Query<(&GoalFor, &Transform), With<Goal>>,
    mut score: ResMut<Score>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score_query: Query<(&PlayerType, &mut Text)>,
) {
    let mut ball_to_despawn = None;

    for (ball_entity, ball_transform) in ball_query.iter() {
        for (goal_for, goal_transform) in goal_query.iter() {
            if check_collision(ball_transform, goal_transform) {
                // Determine which player scored based on the goal hit
                let scoring_player = match goal_for {
                    GoalFor::Player1 => GoalFor::Player2,
                    GoalFor::Player2 => GoalFor::Player1,
                };

                // Update the score and the UI
                update_player_score(&mut score, scoring_player, &mut score_query);

                play_win(&asset_server, &audio);

                ball_to_despawn = Some(ball_entity);
                break;
            }
        }

        if ball_to_despawn.is_some() {
            break;
        }
    }

    if let Some(ball_entity) = ball_to_despawn {
        // Despawn current ball
        commands.entity(ball_entity).despawn();

        // Spawn new ball
        spawn_ball(commands, lastOwner);
    }
}

fn check_collision(ball_transform: &Transform, goal_transform: &Transform) -> bool {
    // Define the paddle size (adjust these values according to your paddle size)
    let goal_size = Vec2::new(100.0, ARENA_HEIGHT);

    let ball_pos = ball_transform.translation;
    let goal_pos = goal_transform.translation;

    // Check for overlap in x and y axes
    ball_pos.x + BALL_SIZE / 2.0 > goal_pos.x - goal_size.x / 2.0
        && ball_pos.x - BALL_SIZE / 2.0 < goal_pos.x + goal_size.x / 2.0
        && ball_pos.y + BALL_SIZE / 2.0 > goal_pos.y - goal_size.y / 2.0
        && ball_pos.y - BALL_SIZE / 2.0 < goal_pos.y + goal_size.y / 2.0
}

fn update_player_score(
    score: &mut Score,
    scoring_player: GoalFor,
    score_query: &mut Query<(&PlayerType, &mut Text)>,
) {
    match scoring_player {
        GoalFor::Player1 => {
            score.player1_score += 1;
        }
        GoalFor::Player2 => {
            score.player2_score += 1;
        }
    }

    for (player_type, mut text) in score_query.iter_mut() {
        match player_type {
            PlayerType::Player1 => {
                text.sections[0].value = format!("{:02}", score.player1_score);
            }
            PlayerType::Player2 => {
                text.sections[0].value = format!("{:02}", score.player2_score);
            }
        }
    }
}

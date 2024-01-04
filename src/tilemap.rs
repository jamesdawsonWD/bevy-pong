use bevy::prelude::*;

use crate::{ARENA_HEIGHT, ARENA_WIDTH};

const DASH_HEIGHT: f32 = 20.0;
const DASH_WIDTH: f32 = 5.0;
const DASH_SPACING: f32 = 20.0;

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (draw_dashed_line, draw_arena_outline));
    }
}

fn draw_arena_outline(mut commands: Commands) {
    let top_position = -ARENA_HEIGHT / 2.0;
    let bottom_position = ARENA_HEIGHT / 2.0;

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(ARENA_WIDTH * 3.0, 2.0)),
            ..default()
        },
        transform: Transform::from_xyz(-ARENA_WIDTH / 2.0, top_position, 0.0),
        ..default()
    });

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(ARENA_WIDTH * 3.0, 2.0)),
            ..default()
        },
        transform: Transform::from_xyz(-ARENA_WIDTH / 2.0, bottom_position, 0.0),
        ..default()
    });

}

fn draw_dashed_line(mut commands: Commands) {
    let mut position_y = -ARENA_HEIGHT / 2.0 + 20.0;

    while position_y < ARENA_HEIGHT / 2.0 {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(DASH_WIDTH, DASH_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, position_y, 0.0),
            ..default()
        });

        position_y += DASH_HEIGHT + DASH_SPACING;
    }
}

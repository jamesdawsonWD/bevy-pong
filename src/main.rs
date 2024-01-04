use bevy::prelude::*;

const ARENA_WIDTH: f32 = 800.0;
const ARENA_HEIGHT: f32 = 600.0;

mod ball;
mod player;
mod tilemap;
mod score;
mod audio;

use ball::BallPlugin;
use player::PlayerPlugin;
use tilemap::TilemapPlugin;
use score::ScorePlugin;
use audio::GameAudioPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Space puzzle game".into(),
                        resolution: (1000.0, 1000.0).into(),
                        resizable: false,

                        ..default()
                    }),
                    ..default()
                }),
        )
        // .add_systems(PreStartup, load_ascii)
        .add_systems(Startup, spawn_camera)
        .add_plugins((PlayerPlugin, TilemapPlugin, BallPlugin, ScorePlugin, GameAudioPlugin))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    commands.spawn(camera);
}

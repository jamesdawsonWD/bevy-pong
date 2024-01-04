use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{ARENA_HEIGHT, ARENA_WIDTH};

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AudioPlugin))
            .add_systems(Startup, start_background_audio);
    }
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("audio/background.ogg"))
        .looped();
}
pub fn play_bounce(asset_server: &Res<AssetServer>, audio: &Res<Audio>) {
    audio.play(asset_server.load("audio/bounce.ogg"));
}
pub fn play_win(asset_server: &Res<AssetServer>, audio: &Res<Audio>) {
    audio.play(asset_server.load("audio/win.ogg"));
}

pub fn play_hit(asset_server: &Res<AssetServer>, audio: &Res<Audio>) {
    audio.play(asset_server.load("audio/paddle_hit.ogg"));
}

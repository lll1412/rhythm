use crate::types::SongConfig;
use crate::AppState;
use bevy::prelude::*;
use crate::time::ControlledTime;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(play_song.system()));
    }
}

fn play_song(audio: Res<Audio>, time: Res<ControlledTime>, config: Res<SongConfig>) {
    let sec = time.seconds_since_startup();
    let sec_last = sec - time.delta_seconds_f64();

    if sec_last <= 3.0 && sec > 3.0 {
        audio.play(config.song_audio.clone());
    }
}

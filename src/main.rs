use bevy::{input::system::exit_on_esc_system, prelude::*};

use arrows::ArrowsPlugin;
use consts::*;
use types::SongConfig;

mod arrows;
mod consts;
mod types;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Rhythm".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(SongConfig::load_config())
        .add_state(AppState::Game)
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ArrowsPlugin)
        .run();
}

fn setup(mut cmd: Commands) {
    // 2d正交相机
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
    // 歌曲资源
    // let config = SongConfig::load_config();
    // cmd.insert_resource(config);
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Menu,
    Game,
}

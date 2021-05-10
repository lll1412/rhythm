use bevy::{input::system::exit_on_esc_system, prelude::*};

use arrows::ArrowsPlugin;
use audio::AudioPlugin;
use consts::{WINDOW_HEIGHT, WINDOW_WIDTH};
use score::ScoreResource;
use shaders::ShadersPlugin;
use types::SongConfig;
use ui::UIPlugin;

mod arrows;
mod audio;
mod consts;
mod score;
mod shaders;
mod types;
mod ui;

fn main() {
    App::build()
        // .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Rhythm".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<ScoreResource>()
        .init_resource::<SongConfig>() // 加载歌曲配置文件
        .add_state(AppState::Game)
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_plugin(ArrowsPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(ShadersPlugin)
        .run();
}

fn setup(mut cmd: Commands) {
    // 2d 正交相机
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d());
    // ui 相机
    cmd.spawn_bundle(UiCameraBundle::default());
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum AppState {
    // Menu,
    Game,
}

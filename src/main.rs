use bevy::{input::system::exit_on_esc_system, prelude::*};

use arrows::ArrowsPlugin;
use consts::{WINDOW_HEIGHT, WINDOW_WIDTH};
use types::SongConfig;
use ui::UIPlugin;

mod arrows;
mod consts;
mod types;
mod ui;

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
        .add_plugin(UIPlugin)
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
    Menu,
    Game,
}

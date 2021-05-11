#![warn(clippy::all)]

use bevy::{input::system::exit_on_esc_system, prelude::*};

use arrows::ArrowsPlugin;
use audio::AudioPlugin;
use consts::{WINDOW_HEIGHT, WINDOW_WIDTH};
use map_maker::MapMakerPlugin;
use menu::MenuPlugin;
use score::ScoreResource;
use shaders::ShadersPlugin;
use time::TimePlugin;
use ui::UIPlugin;

mod arrows;
mod audio;
mod consts;
mod map_maker;
mod menu;
mod score;
mod shaders;
mod time;
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
        .add_plugins(DefaultPlugins)
        .init_resource::<ScoreResource>()
        .add_state(AppState::Menu)
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_plugin(ArrowsPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(ShadersPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(TimePlugin)
        .add_plugin(MapMakerPlugin)
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
    MakeMap,
}

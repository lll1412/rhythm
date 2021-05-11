use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::consts::ARROW_SIZE;
use crate::time::ControlledTime;
use crate::types::{ArrowTimeToml, Directions, Speed};
use crate::AppState;

pub struct MapMakerPlugin;

impl Plugin for MapMakerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Presses>()
            .init_resource::<MapMakerAudio>()
            .add_system_set(
                SystemSet::on_enter(AppState::MakeMap)
                    .with_system(setup_map_maker.system())
                    .with_system(start_song.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::MakeMap)
                    .with_system(save_key_presses.system())
                    .with_system(save_to_file.system())
                    .with_system(toggle_map_maker_arrows.system()),
            );
    }
}

/// 按键录制
#[derive(Serialize, Deserialize, Default)]
struct Presses {
    arrows: Vec<ArrowTimeToml>,
}
struct MapMakeArrow(Directions);
/// 按键录制
fn save_key_presses(
    key_input: Res<Input<KeyCode>>,
    mut presses: ResMut<Presses>,
    time: Res<ControlledTime>,
) {
    let directions = Directions::directions();
    for direction in directions {
        if direction.key_just_pressed(&key_input) {
            presses.arrows.push(ArrowTimeToml {
                click_time: time.seconds_since_startup(),
                speed: Speed::Slow,
                direction,
            })
        }
    }
}
/// 保存到文件
fn save_to_file(
    key_input: Res<Input<KeyCode>>,
    presses: Res<Presses>,
    mut state: ResMut<State<AppState>>,
) {
    // ctrl + s
    if key_input.pressed(KeyCode::LControl) && key_input.just_pressed(KeyCode::F) {
        // 按键序列化为toml格式
        let toml_text = toml::to_string_pretty(&*presses).expect("couldn't convert to toml text");
        // 保存文件
        std::fs::write("assets/songs/map.toml", toml_text).expect("couldn't write to map.toml");
        // 返回menu
        state.set(AppState::Menu).unwrap();
    }
}
// 初始化
fn setup_map_maker(
    mut cmd: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let border_material = materials.add(asset_server.load("images/arrow_border.png").into());
    let directions = Directions::directions();
    for direction in directions {
        let y = direction.y();
        let mut transform = Transform::from_translation(Vec3::new(0.0, y, 1.0));
        transform.rotate(Quat::from_rotation_z(direction.rotation()));
        cmd.spawn_bundle(SpriteBundle {
            material: border_material.clone(),
            sprite: Sprite::new(Vec2::new(ARROW_SIZE, ARROW_SIZE)),
            transform,
            ..Default::default()
        })
        .insert(MapMakeArrow(direction));
    }
}

// 按键点击时才显示
fn toggle_map_maker_arrows(
    key_input: Res<Input<KeyCode>>,
    q: Query<(&mut Visible, &MapMakeArrow)>,
) {
    q.for_each_mut(|(mut visible, arrow)| {
        visible.is_visible = arrow.0.key_just_pressed(&key_input);
    });
}
//
struct MapMakerAudio(Handle<AudioSource>);

impl FromWorld for MapMakerAudio {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let song = asset_server.load("map_maker_song.mp3");
        Self(song)
    }
}

fn start_song(audio: Res<Audio>, map_maker_audio: Res<MapMakerAudio>) {
    audio.play(map_maker_audio.0.clone());
}

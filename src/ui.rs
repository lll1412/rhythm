use bevy::prelude::*;

use crate::score::ScoreResource;
use crate::AppState;
use crate::consts::DELAY_SONG;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_ui.system()))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(update_time_text.system())
                    .with_system(update_score_text.system()),
            );
    }
}

fn setup_ui(
    mut cmd: Commands,
    assert_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let font: Handle<Font> = assert_server.load("fonts/FiraSans-Bold.ttf");
    let material = materials.add(Color::NONE.into());
    // time text
    cmd.spawn_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                ..Default::default()
            },
            ..Default::default()
        },
        material: material.clone(),
        ..Default::default()
    })
    .with_children(|parent| {
        parent
            .spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Time: 0.0",
                    TextStyle {
                        font: font.clone(),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    TextAlignment::default(),
                ),
                ..Default::default()
            })
            .insert(TimeText);
    });
    // score text
    cmd.spawn_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(10.0),
                bottom: Val::Px(10.0),
                ..Default::default()
            },
            ..Default::default()
        },
        material,
        ..Default::default()
    })
    .with_children(|parent| {
        parent
            .spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Score: 0. Corrects: 0. Fails: 0.",
                    TextStyle {
                        font,
                        font_size: 40.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                    },
                    TextAlignment::default(),
                ),
                ..Default::default()
            })
            .insert(ScoreText);
    });
}
struct TimeText;
/// 更新时间文本
fn update_time_text(time: Res<Time>, mut time_text: Query<&mut Text, With<TimeText>>) {
    let sec = time.seconds_since_startup() - DELAY_SONG;
    if sec >= 0.0 {
        let mut time_text = time_text.single_mut().unwrap();
        time_text.sections[0].value = format!("Time: {:.2}", sec);
    }
}

struct ScoreText;
/// 更新分数文本
fn update_score_text(
    score_resource: Res<ScoreResource>,
    mut score_text: Query<&mut Text, With<ScoreText>>,
) {
    if score_resource.is_changed() {
        let mut score_text = score_text.single_mut().unwrap();
        score_text.sections[0].value = format!("{}", *score_resource);
    }
}

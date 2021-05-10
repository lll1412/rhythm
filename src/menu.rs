use bevy::prelude::*;

use crate::types::SongConfig;
use crate::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu.system()))
            .add_system_set(
                SystemSet::on_update(AppState::Menu).with_system(button_interaction.system()),
            )
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(despawn_menu.system()));
    }
}

struct ButtonMaterials {
    none: Handle<ColorMaterial>,
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
    font: Handle<Font>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        Self {
            none: materials.add(Color::NONE.into()),
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.35, 0.35, 0.35).into()),
            pressed: materials.add(Color::rgb(0.55, 0.75, 0.55).into()),
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        }
    }
}

enum MenuButton {
    MakeMap,
    PlaySong(String),
}

impl MenuButton {
    fn name(&self) -> String {
        match self {
            MenuButton::MakeMap => String::from("Make Map"),
            MenuButton::PlaySong(song) => format!("Play song: {}", song),
        }
    }
}

struct MenuUI;

/// 初始菜单
fn setup_menu(mut cmd: Commands, button_materials: Res<ButtonMaterials>) {
    let mut menu_buttons: Vec<MenuButton> = get_songs()
        .iter()
        .map(|name| MenuButton::PlaySong(name.clone()))
        .collect();
    menu_buttons.push(MenuButton::MakeMap);
    // node > button > text
    let node_bundle = NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexStart,
            ..Default::default()
        },
        material: button_materials.none.clone(),
        ..Default::default()
    };
    cmd.spawn_bundle(node_bundle)
        .insert(MenuUI)
        .with_children(|parent_node| {
            let button_bundle = ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(350.0), Val::Px(65.0)),
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: button_materials.normal.clone(),
                ..Default::default()
            };
            for menu_button in menu_buttons {
                parent_node
                    .spawn_bundle(button_bundle.clone())
                    .with_children(|parent_button| {
                        let text_bundle = TextBundle {
                            text: Text::with_section(
                                menu_button.name(),
                                TextStyle {
                                    font: button_materials.font.clone(),
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        };
                        parent_button.spawn_bundle(text_bundle);
                    })
                    .insert(menu_button);
            }
        });
}

/// 移除菜单和按钮
fn despawn_menu(mut cmd: Commands, menu: Query<Entity, With<MenuUI>>) {
    menu.for_each(|e| cmd.entity(e).despawn_recursive());
}

type InteractionButton = (Changed<Interaction>, With<Button>);

/// 按钮样式变换
fn button_interaction(
    mut cmd: Commands,
    mut state: ResMut<State<AppState>>,
    button_materials: Res<ButtonMaterials>,
    interaction_button: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &MenuButton),
        InteractionButton,
    >,
    asset_server: Res<AssetServer>,
) {
    interaction_button.for_each_mut(
        |(interaction, mut material, menu_button)| match interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                button_pressed_handle(&mut cmd, &mut state, menu_button, &asset_server);
            }
            Interaction::Hovered => *material = button_materials.hovered.clone(),
            Interaction::None => *material = button_materials.normal.clone(),
        },
    );
}

/// 读取歌曲文件
fn get_songs() -> Vec<String> {
    let paths = std::fs::read_dir("assets/songs").unwrap();
    paths
        .map(|path| path.unwrap().path())
        .filter(|path| "toml" == path.as_path().extension().unwrap())
        .map(|path| {
            path.as_path()
                .file_stem() // 去除后缀
                .unwrap()
                .to_str() // 转为
                .unwrap()
                .to_string()
        })
        .collect()
}

fn button_pressed_handle(
    cmd: &mut Commands,
    state: &mut State<AppState>,
    menu_button: &MenuButton,
    asset_server: &AssetServer,
) {
    match menu_button {
        MenuButton::MakeMap => state.set(AppState::MakeMap).unwrap(),
        MenuButton::PlaySong(song_config) => {
            let song_config =
                SongConfig::load_config(&*format!("{}.toml", song_config), &asset_server);
            cmd.insert_resource(song_config);
            state.set(AppState::Game).unwrap();
        }
    }
}

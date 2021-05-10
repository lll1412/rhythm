use bevy::prelude::*;

use crate::AppState;
use crate::consts::*;
use crate::score::ScoreResource;
use crate::types::*;

pub struct ArrowsPlugin;

impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ArrowMaterialResource>()
            .add_system_set(
                SystemSet::on_enter(AppState::Game).with_system(setup_target_arrows.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(spawn_arrows.system()) // 生成箭头
                    .with_system(move_arrows.system()) // 箭头移动
                    .with_system(despawn_arrows.system()), // 移除箭头
            );
    }
}

/// 箭头资源
struct ArrowMaterialResource {
    red_texture: Handle<ColorMaterial>,
    blue_texture: Handle<ColorMaterial>,
    green_texture: Handle<ColorMaterial>,
    border_texture: Handle<ColorMaterial>,
}

impl FromWorld for ArrowMaterialResource {
    fn from_world(world: &mut World) -> Self {
        // 这样让world在同一个地方多次mut引用
        let world = world.cell();
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        //  加载箭头图片
        let red_handle = asset_server.load("images/arrow_red.png");
        let blue_handle = asset_server.load("images/arrow_blue.png");
        let green_handle = asset_server.load("images/arrow_green.png");
        let border_handle = asset_server.load("images/arrow_border.png");
        ArrowMaterialResource {
            red_texture: materials.add(red_handle.into()),
            blue_texture: materials.add(blue_handle.into()),
            green_texture: materials.add(green_handle.into()),
            border_texture: materials.add(border_handle.into()),
        }
    }
}

/// 箭头组件
struct Arrow {
    speed: Speed,
    direction: Directions,
}

impl Arrow {
    fn from(arrow_time: &ArrowTime) -> Self {
        Self {
            speed: arrow_time.speed,
            direction: arrow_time.direction,
        }
    }
}

/// 生成箭头
fn spawn_arrows(
    mut cmd: Commands,
    mut song_config: ResMut<SongConfig>,
    materials: Res<ArrowMaterialResource>,
    time: Res<Time>,
) {
    // 从启动到现在的时间（减3 是因为歌曲在游戏开始3秒后播放）
    let sec = time.seconds_since_startup() - DELAY_SONG;
    // 两帧时间差
    let delta = time.delta_seconds_f64();
    // 上一帧时间
    let sec_last = sec - delta;

    let mut remove_counter = 0;
    for arrow in &song_config.arrows {
        // 如果生成时间介于上一帧和这一帧之间
        // println!("sec:{}, delta:{}, sec_last:{}, spawn_time:{}", sec, delta, sec_last, arrow.spawn_time);
        if arrow.spawn_time > sec_last && arrow.spawn_time <= sec {
            remove_counter += 1;

            // 根据速度获取材质
            let material = match arrow.speed {
                Speed::Slow => materials.red_texture.clone(),
                Speed::Medium => materials.blue_texture.clone(),
                Speed::Fast => materials.green_texture.clone(),
            };
            let sprite_bundle = spawn_arrow_sprite(material, &arrow.direction, SPAWN_POSITION);
            cmd.spawn_bundle(sprite_bundle).insert(Arrow::from(arrow));
        } else {
            break;
        }
    }
    // 移除已经生成过的
    for _ in 0..remove_counter {
        song_config.arrows.remove(0);
    }
}

/// 删除箭头
fn despawn_arrows(
    mut cmd: Commands,
    arrows: Query<(Entity, &Transform, &Arrow)>,
    key_input: Res<Input<KeyCode>>,
    mut score: ResMut<ScoreResource>,
) {
    arrows.for_each(|(entity, transform, arrow)| {
        let pos = transform.translation.x;
        // 检测箭头是否在目标箭头范围内被点击
        if (TARGET_POSITION - THRESHOLD..=TARGET_POSITION + THRESHOLD).contains(&pos)
            && arrow.direction.key_just_pressed(&key_input)
        {
            score.increase_correct(TARGET_POSITION - pos);
            cmd.entity(entity).despawn();
        }
        // 是否离开屏幕
        if pos >= 2.0 * TARGET_POSITION {
            cmd.entity(entity).despawn();
            score.increase_fails();
        }
    });
}
/// 移动箭头
fn move_arrows(time: Res<Time>, arrows: Query<(&mut Transform, &Arrow)>) {
    arrows.for_each_mut(|(mut transform, arrow)| {
        transform.translation.x += time.delta_seconds() * arrow.speed.value();

        let distance_after_target = transform.translation.x - (TARGET_POSITION + THRESHOLD);
        if distance_after_target >= 0.02 {
            // 下移
            transform.translation.y -= time.delta_seconds() * distance_after_target * 2.0;
            // 旋转
            transform.rotate(Quat::from_rotation_z(
                -distance_after_target * arrow.speed.multiplier() / 720.0,
            ));
            // 缩小
            let scale = (1.0 - distance_after_target / 300.0).max(0.2);
            transform.scale = Vec3::splat(scale);
        }
    });
}

/// 目标箭头
struct TargetArrow;
/// 初始化目标箭头
fn setup_target_arrows(mut cmd: Commands, materials: Res<ArrowMaterialResource>) {
    use Directions::*;
    let directions = [Up, Down, Left, Right];
    for direction in directions.iter() {
        let sprite_bundle = spawn_arrow_sprite(
            materials.border_texture.clone(),
            &direction,
            TARGET_POSITION,
        );
        cmd.spawn_bundle(sprite_bundle).insert(TargetArrow);
    }
}

/// 辅助函数
fn spawn_arrow_sprite(
    material: Handle<ColorMaterial>,
    direction: &Directions,
    start_position: f32,
) -> SpriteBundle {
    // 初始位置
    let mut transform = Transform::from_translation(Vec3::new(start_position, direction.y(), 1.0));
    // 旋转到正确方向
    transform.rotate(Quat::from_rotation_z(direction.rotation()));
    // 大小
    let sprite = Sprite::new(Vec2::new(ARROW_SIZE, ARROW_SIZE));
    SpriteBundle {
        sprite,
        material,
        transform,
        ..Default::default()
    }
}

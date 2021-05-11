use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::consts::*;

/// 箭头方向
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}

impl Directions {
    /// 当前方向键是否被按下
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        // input.just_pressed()
        let keys = match self {
            Directions::Up => [KeyCode::Up, KeyCode::W],
            Directions::Down => [KeyCode::Down, KeyCode::S],
            Directions::Left => [KeyCode::Left, KeyCode::A],
            Directions::Right => [KeyCode::Right, KeyCode::D],
        };
        keys.iter().any(|k| input.just_pressed(*k))
    }
    /// 返回箭头旋转角度（初始位置是右），比如向上旋转则是逆时针 PI/2
    pub fn rotation(&self) -> f32 {
        match self {
            Directions::Up => FRAC_PI_2,
            Directions::Down => -FRAC_PI_2,
            Directions::Left => PI,
            Directions::Right => 0.0,
        }
    }
    /// 返回箭头的纵坐标(纵向)
    pub fn y(&self) -> f32 {
        match self {
            Directions::Up => WINDOW_HEIGHT / 4.0,
            Directions::Down => WINDOW_HEIGHT / 12.0,
            Directions::Left => -WINDOW_HEIGHT / 12.0,
            Directions::Right => -WINDOW_HEIGHT / 4.0,
        }
    }
}

/// 箭头速度
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Speed {
    Slow,
    Medium,
    Fast,
}

impl Speed {
    /// 返回当前速度值
    pub fn value(&self) -> f32 {
        BASE_SPEED * self.multiplier()
    }
    /// 速度倍数
    pub fn multiplier(&self) -> f32 {
        match self {
            Speed::Slow => 1.0,
            Speed::Medium => 1.2,
            Speed::Fast => 1.5,
        }
    }
}

/// 每个箭头的基本属性
#[derive(Debug, Copy, Clone)]
pub struct ArrowTime {
    pub spawn_time: f64,
    pub speed: Speed,
    pub direction: Directions,
}

impl ArrowTime {
    /// click_time 是按钮点击时间
    pub fn new(arrow: &ArrowTimeToml) -> Self {
        let speed_value = arrow.speed.value();
        // 根据点击时间计算出生成箭头时间
        let spawn_time = arrow.click_time - (DISTANCE / speed_value) as f64;
        Self {
            spawn_time,
            speed: arrow.speed,
            direction: arrow.direction,
        }
    }
}

/// 一首歌的 箭头序列 配置
pub struct SongConfig {
    pub name: String,
    pub song_audio: Handle<AudioSource>,
    pub arrows: Vec<ArrowTime>,
}

impl SongConfig {
    pub fn load_config(path: &str, asset_server: &AssetServer) -> Self {
        // 读取文件
        let contents =
            std::fs::read_to_string(format!("assets/songs/{}", path)).expect("couldn't read file");
        // 解析文件
        let parsed: SongConfigToml = toml::from_str(&contents).expect("parse file error");
        // 处理解析文件
        let mut arrows: Vec<ArrowTime> = parsed
            .arrows
            .iter()
            .map(|arr| ArrowTime::new(arr))
            .collect();
        // 排序
        arrows.sort_by(|a, b| a.spawn_time.partial_cmp(&b.spawn_time).unwrap());
        // 加载音频文件
        let song_audio = asset_server.load(&*format!("songs/{}", parsed.filename));
        Self {
            name: parsed.name,
            song_audio,
            arrows,
        }
    }
}

#[derive(Debug, Deserialize)]
struct SongConfigToml {
    pub name: String,
    pub filename: String,
    pub arrows: Vec<ArrowTimeToml>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrowTimeToml {
    pub click_time: f64,
    pub speed: Speed,
    pub direction: Directions,
}

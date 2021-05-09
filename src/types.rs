use std::f32::consts::PI;

use bevy::prelude::*;

use crate::consts::*;

/// 箭头方向
#[derive(Debug, Copy, Clone, PartialEq)]
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
            Directions::Up => PI * 0.5,
            Directions::Down => -PI * 0.5,
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
#[derive(Debug, Copy, Clone)]
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
    fn multiplier(&self) -> f32 {
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
    pub fn new(click_time: f64, speed: Speed, direction: Directions) -> Self {
        let speed_value = speed.value();
        // 根据点击时间计算出生成箭头时间
        let spawn_time = click_time - (DISTANCE / speed_value) as f64;
        Self {
            spawn_time,
            speed,
            direction,
        }
    }
}

/// 一首歌的 箭头序列 配置
pub struct SongConfig {
    pub arrows: Vec<ArrowTime>,
}

impl SongConfig {
    pub fn load_config() -> Self {
        let arrows = vec![
            ArrowTime::new(1.0, Speed::Slow, Directions::Up),
            ArrowTime::new(2.0, Speed::Slow, Directions::Down),
            ArrowTime::new(3.0, Speed::Slow, Directions::Left),
            ArrowTime::new(4.0, Speed::Medium, Directions::Up),
            ArrowTime::new(5.0, Speed::Fast, Directions::Right),
        ];
        Self { arrows }
    }
}

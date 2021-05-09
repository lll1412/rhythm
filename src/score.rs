use std::fmt::Formatter;

use bevy::prelude::*;

use crate::consts::THRESHOLD;

#[derive(Default)]
pub struct ScoreResource {
    corrects: usize,
    fails: usize,

    score: usize,
}

impl ScoreResource {
    /// 增加分数
    pub fn increase_correct(&mut self, distance: f32) -> usize {
        self.corrects += 1;
        // 根据离目标远近获取分数倍率加成(0到1)
        let score_multiplier = (THRESHOLD - distance.abs()) / THRESHOLD;
        // 分数最低10,最高100
        let points = (score_multiplier * 100.0).min(100.0).max(10.0) as usize;
        self.score += points;
        points
    }
    /// 统计失败数
    pub fn increase_fails(&mut self) {
        self.fails += 1;
    }
    /// fields getter
    pub fn score(&self) -> usize {
        self.score
    }
    pub fn corrects(&self) -> usize {
        self.corrects
    }
    pub fn fails(&self) -> usize {
        self.fails
    }
}
impl std::fmt::Display for ScoreResource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Score: {}. Corrects: {}. Fails: {}.",
            self.score, self.corrects, self.fails
        )
    }
}

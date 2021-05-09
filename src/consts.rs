/// 游戏窗口宽度
pub const WINDOW_WIDTH: f32 = 800.0;
/// 游戏窗口高度
pub const WINDOW_HEIGHT: f32 = 600.0;

/// 箭头移动速度
pub const BASE_SPEED: f32 = 200.0;

/// 箭头大小
pub const ARROW_SIZE: f32 = 140.0;

/// 箭头生成时的 初始横坐标
pub const SPAWN_POSITION: f32 = -400.0;

/// 目标箭头 横坐标
pub const TARGET_POSITION: f32 = 200.0;

/// 匹配箭头时的误差阈值
pub const THRESHOLD: f32 = 20.0;

/// 箭头 出发点 到 目标位置 的距离
pub const DISTANCE: f32 = TARGET_POSITION - SPAWN_POSITION;

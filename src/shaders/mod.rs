use bevy::prelude::*;
use bevy::render::pipeline::{PipelineDescriptor, RenderPipeline};
use bevy::render::render_graph::{base, RenderGraph, RenderResourcesNode};
use bevy::render::renderer::RenderResources;
use bevy::render::shader::{ShaderStage, ShaderStages};
use bevy::window::WindowResized;

use background::*;
use target_arrows::*;

use crate::time::ControlledTime;
use crate::AppState;

mod background;
mod target_arrows;

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Game)
                .with_system(setup_background.system())
                .with_system(setup_render_graph.system())
                .with_system(setup_target_arrows.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(update_background_size.system())
                .with_system(update_time.system())
                .with_system(update_resolution.system())
                .with_system(correct_event.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Game).with_system(despawn_background.system()),
        );
    }
}

/// 传递这些变量到shader中使用
#[derive(RenderResources)]
pub struct ShaderInputs {
    // 时间
    time: f32,
    // 分辨率
    resolution: Vec2,
}
/// 更新时间
fn update_time(time: Res<ControlledTime>, nodes: Query<&mut ShaderInputs>) {
    let time = time.seconds_since_startup();
    nodes.for_each_mut(|mut node| node.time = time as f32);
}

/// 更新分辨率
fn update_resolution(mut resize_event: EventReader<WindowResized>, q: Query<&mut ShaderInputs>) {
    resize_event.iter().for_each(|e| {
        q.for_each_mut(|mut si| si.resolution = Vec2::new(e.width / e.height, 1.0));
    })
}

/// 配置
fn setup_render_graph(mut render_graph: ResMut<RenderGraph>) {
    render_graph.add_system_node("inputs", RenderResourcesNode::<ShaderInputs>::new(true));
    render_graph
        .add_node_edge("inputs", base::node::MAIN_PASS)
        .unwrap();
}
fn despawn_background(mut cmd: Commands, q: Query<Entity, With<Background>>) {
    q.for_each(|e| cmd.entity(e).despawn_recursive());
}

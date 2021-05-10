use bevy::prelude::*;
use bevy::render::pipeline::{PipelineDescriptor, RenderPipeline};
use bevy::render::render_graph::{base, RenderGraph, RenderResourcesNode};
use bevy::render::renderer::RenderResources;
use bevy::render::shader::{ShaderStage, ShaderStages};
use bevy::window::WindowResized;

use background::*;

use crate::AppState;

mod background;

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Game).with_system(setup_background.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game).with_system(update_background_size.system()),
        );
    }
}

/// bevy默认没有这两个资源，自己加一下
#[derive(RenderResources)]
pub struct ShaderInputs {
    time: f32,
    // 时间
    resolution: Vec2, // 分辨率
}
/// 更新时间
fn update_time(time: Res<Time>, nodes: Query<&mut ShaderInputs>) {
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
    render_graph.add_node_edge("inputs", base::node::MAIN_PASS).unwrap();
}

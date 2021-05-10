use bevy::prelude::*;
use bevy::render::pipeline::{PipelineDescriptor, RenderPipeline};
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

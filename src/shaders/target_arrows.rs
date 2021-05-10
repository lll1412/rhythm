use crate::consts::TARGET_POSITION;
use crate::types::Directions;

use super::*;
use crate::arrows::CorrectArrowEvent;

pub struct TargetArrowSparkle {
    direction: Directions,
}

#[derive(RenderResources)]
pub struct TimeSinceLastCorrect {
    last_time: f32,
    points: f32,
}

pub fn setup_target_arrows(
    mut cmd: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
    window: Res<WindowDescriptor>,
) {
    // 创建着色器管道
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("target_arrows.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("target_arrows.frag"),
        ))),
    }));

    // 将 TimeSinceLastCorrect 添加到渲染图中
    render_graph.add_system_node(
        "last_time",
        RenderResourcesNode::<TimeSinceLastCorrect>::new(true),
    );
    render_graph
        .add_node_edge("last_time", base::node::MAIN_PASS)
        .unwrap();
    use Directions::*;
    let directions = [Up, Down, Left, Right];
    directions.iter().for_each(|direction| {
        let z = match direction {
            Up => 0.3,
            Down => 0.4,
            Left => 0.5,
            Right => 0.6,
        };
        let mut transform =
            Transform::from_translation(Vec3::new(TARGET_POSITION, direction.y(), z));
        transform.scale = Vec3::new(300., 300., 1.);
        // 渲染管道
        let render_pipelines =
            RenderPipelines::from_pipelines(vec![RenderPipeline::new(pipeline_handle.clone())]);
        cmd.spawn_bundle(SpriteBundle {
            render_pipelines,
            transform,
            ..Default::default()
        })
        .insert(TargetArrowSparkle {
            direction: *direction,
        })
        .insert(TimeSinceLastCorrect {
            last_time: -1.0,
            points: 0.0,
        })
        .insert(ShaderInputs {
            time: 0.0,
            resolution: Vec2::new(window.width / window.height, 1.),
        });
    });
}

pub fn correct_event(
    mut correct_event: EventReader<CorrectArrowEvent>,
    q: Query<(&TargetArrowSparkle, &mut TimeSinceLastCorrect)>,
    time: Res<Time>,
) {
    correct_event.iter().for_each(|e| {
        q.for_each_mut(|(tas, mut lc)| {
            if e.direction == tas.direction {
                lc.last_time = time.seconds_since_startup() as f32;
                lc.points = e.points as f32 / 100.0;
            }
        })
    });
}

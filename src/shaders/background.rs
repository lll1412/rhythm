use super::*;

pub struct Background;

pub fn setup_background(
    mut cmd: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    window: Res<WindowDescriptor>,
) {
    // 创建着色器管道
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("background.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("background.frag"),
        ))),
    }));

    let render_pipelines =
        RenderPipelines::from_pipelines(vec![RenderPipeline::new(pipeline_handle)]);

    let transform = Transform::from_scale(Vec3::new(window.width, window.height, 1.0));
    cmd.spawn_bundle(SpriteBundle {
        render_pipelines,
        transform,
        ..Default::default()
    })
    .insert(Background)
    .insert(ShaderInputs {
        time: 0.0,
        resolution: Vec2::new(window.width / window.height, 1.0),
    });
}

pub fn update_background_size(
    mut resize_event: EventReader<WindowResized>,
    background_transform: Query<&mut Transform, With<Background>>,
) {
    resize_event.iter().for_each(|e| {
        background_transform.for_each_mut(|mut transform| {
            transform.scale = Vec3::new(e.width, e.height, 1.0);
        });
    });
}

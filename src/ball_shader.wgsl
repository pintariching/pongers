struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
};

struct Uniforms {
    left_paddle_position: vec2<f32>,
    left_paddle_width: f32,
    left_paddle_height: f32,
    right_paddle_position: vec2<f32>,
    right_paddle_width: f32,
    right_paddle_height: f32,
    ball_position: vec2<f32>,
    ball_radius: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if length(uniforms.ball_position - in.position.xy) > uniforms.ball_radius {
        discard
    }

    return vec4(1.);
}
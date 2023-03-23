struct Camera {
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec2<f32>,
}

struct Paddle {
    @location(2) position: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
}

@vertex
fn vs_main(vert: VertexInput, paddle: Paddle) -> VertexOutput {

    var out: VertexOutput;

    out.position = camera.view_proj * vec4<f32>(vert.position + paddle.position, 0., 1.);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(1.);
}
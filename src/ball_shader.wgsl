struct Camera {
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
    @location(1) radius: f32,
};

struct Ball {
    @location(2) center: vec2<f32>,
    @location(3) radius: f32,
}

@vertex
fn vs_main(vert: VertexInput, ball: Ball) -> VertexOutput {
    var out: VertexOutput;

    out.position = camera.view_proj * vec4<f32>(vert.position + ball.center, 0., 1.);
    out.coord = vert.position.xy;
    out.radius = ball.radius;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // let r: f32 = dot(in.coord, in.coord);

    // if r > in.radius {
    //     discard;
    // }

    return vec4<f32>(in.coord, 0., 1.);
}
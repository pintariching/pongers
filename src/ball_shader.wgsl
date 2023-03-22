struct Camera {
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
};

struct InstanceInput {
    @location(2) model_matrix_0: vec4<f32>,
    @location(3) model_matrix_1: vec4<f32>,
    @location(4) model_matrix_2: vec4<f32>,
    @location(5) model_matrix_3: vec4<f32>,
}

@vertex
fn vs_main(vert: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.position = camera.view_proj * vec4<f32>(vert.position, 1.);
    out.coord = vert.position.xy;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let r: f32 = dot(in.coord, in.coord);

    if r > 100. {
        discard;
    }

    return vec4(1.);
}

// fn circle_shape(position: vec3<f32>, radius: f32) -> f32 {
//     return step(radius, length(position));
// }

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {

    var vertices = array<vec2<f32>, 3>(
        vec2<f32>(0., 0.),
        vec2<f32>(0.5, 0.),
        vec2<f32>(0., 0.5),
    );

    var out: VertexOutput;
    out.coord = vertices[in.vertex_index];
    out.position = vec4<f32>(out.coord, 0.0, 1.0);

    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // let r: f32 = dot(in.coord, in.coord);

    // if r > .95 {
    //     discard;
    // }

    // let normalized = (in.coord + vec2<f32>(1., 1.)) / 2.;
    return vec4<f32>(in.coord.x, in.coord.y, 0., 1.0);
}
fn circle_shape(position: vec3<f32>, radius: f32) -> f32 {
    return step(radius, length(position));
}
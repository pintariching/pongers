struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
};

struct Paddle {
    a: vec2<f32>,
    b: vec2<f32>,
    c: vec2<f32>,
    d: vec2<f32>,
    is_active: i32,
}

struct Ball {
    position: vec2<f32>,
    radius: f32
}

@group(0) @binding(0)
var<storage> ball: Ball;

@group(1) @binding(0)
var<storage> paddles: array<Paddle, 8>;


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    for (var i = 0; i < 8; i++) {
        let p = paddles[i];

        if p.is_active == 0 {
            continue;
        }

        // Check if pixel is inside of the square
        let ab = p.b - p.a;
        let ap = in.position.xy - p.a;
        let bc = p.c - p.b;
        let bp = in.position.xy - p.b;
        let dot_ab_ap = dot(ab, ap);
        let dot_ab_ab = dot(ab, ab);
        let dot_bc_bp = dot(bc, bp);
        let dot_bc_bc = dot(bc, bc);
        if (dot_ab_ap >= 0. && dot_ab_ap <= dot_ab_ab) && (dot_bc_bp >= 0. && dot_bc_bp <= dot_bc_bc) {
            return vec4(1.);
        }
    }

    if length(ball.position - in.position.xy) < ball.radius {
        return vec4(1.);
    }

    discard;
}

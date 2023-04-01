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
    if draw_left_paddle(in.position.xy) {
        return vec4(1.);
    }

    if draw_right_paddle(in.position.xy) {
        return vec4(1.);
    }

    discard
}

fn draw_left_paddle(pos: vec2<f32>) -> bool {
    if pos.x > uniforms.left_paddle_position.x + uniforms.left_paddle_width / 2. {
        return false;
    }

    if pos.x < uniforms.left_paddle_position.x - uniforms.left_paddle_width / 2. {
        return false;
    }

    if pos.y > uniforms.left_paddle_position.y + uniforms.left_paddle_height / 2. {
        return false;
    }

    if pos.y < uniforms.left_paddle_position.y - uniforms.left_paddle_height / 2. {
        return false;
    }

    return true;
}

fn draw_right_paddle(pos: vec2<f32>) -> bool {
    if pos.x > uniforms.right_paddle_position.x + uniforms.right_paddle_width / 2. {
        return false;
    }

    if pos.x < uniforms.right_paddle_position.x - uniforms.right_paddle_width / 2. {
        return false;
    }

    if pos.y > uniforms.right_paddle_position.y + uniforms.right_paddle_height / 2. {
        return false;
    }

    if pos.y < uniforms.right_paddle_position.y - uniforms.right_paddle_height / 2. {
        return false;
    }

    return true;
}
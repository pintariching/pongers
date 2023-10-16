use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use wgpu::Color;

use crate::game_state::MAX_DEBUG_LINE_COUNT;

#[derive(Clone, Copy)]
pub struct DebugLine {
    pub a: Vec2,
    pub b: Vec2,
    pub color: Color,
}

impl DebugLine {
    pub fn new(a: Vec2, b: Vec2, color: Color) -> Self {
        Self { a, b, color }
    }

    pub fn to_points(&self) -> (DebugLinePoint, DebugLinePoint) {
        let a = DebugLinePoint {
            pos: self.a,
            color: [
                self.color.r as f32,
                self.color.g as f32,
                self.color.b as f32,
                self.color.a as f32,
            ],
        };
        let b = DebugLinePoint {
            pos: self.b,
            color: [
                self.color.r as f32,
                self.color.g as f32,
                self.color.b as f32,
                self.color.a as f32,
            ],
        };

        (a, b)
    }
}

impl Default for DebugLine {
    fn default() -> Self {
        Self {
            a: Default::default(),
            b: Default::default(),
            color: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct DebugLinePoint {
    pub pos: Vec2,
    pub color: [f32; 4],
}

impl Default for DebugLinePoint {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            color: Default::default(),
        }
    }
}

pub fn debug_lines_to_points(
    debug_lines: [DebugLine; MAX_DEBUG_LINE_COUNT],
) -> [DebugLinePoint; MAX_DEBUG_LINE_COUNT * 2] {
    let mut out = [DebugLinePoint::default(); MAX_DEBUG_LINE_COUNT * 2];

    let mut i = 0;
    for line in debug_lines {
        let points = line.to_points();
        out[i] = points.0;
        out[i + 1] = points.1;

        i += 2;
    }

    out
}

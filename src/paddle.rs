use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Paddle {
    pub width: u32,
    pub height: u32,
    pub position: Vec2,
    pub rotation: f32,
    pub up: KeyCode,
    pub down: KeyCode,
}

impl Paddle {
    pub fn handle_input(&mut self, input: &Res<Input<KeyCode>>, delta_time: f32) {
        if input.pressed(self.up) {
            self.position.y += 100. * delta_time;
        }

        if input.pressed(self.down) {
            self.position.y += -100. * delta_time;
        }
    }
}

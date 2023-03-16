use state::State;
use wgpu::SurfaceError;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod ball;
mod camera;
mod game_state;
mod paddle;
mod state;

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Pongers")
        .with_inner_size(PhysicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(window).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { window_id, event } if window_id == state.window().id() => {
            if !state.input(&event) {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => state.resize(physical_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(*new_inner_size)
                    }
                    _ => (),
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == state.window().id() => {
            state.update();
            match state.render() {
                Ok(_) => (),
                // Reconfigure the surface if lost
                Err(SurfaceError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit
                Err(SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            state.game_state.update();

            // RedrawRequested will only trigger once, unless we manually request it.
            state.window().request_redraw();
        }
        _ => (),
    })
}

extern crate winit;

use winit::window::WindowBuilder;
use winit::event::{Event, WindowEvent, ElementState, KeyboardInput};
use winit::event_loop::{EventLoop, ControlFlow};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Super Cursor Grab'n'Hide Simulator 9000")
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput(KeyboardInput {
                    state: ElementState::Released,
                    virtual_keycode: Some(key),
                    modifiers,
                    ..
                }) => {
                    use winit::event::VirtualKeyCode::*;
                    match key {
                        Escape => *control_flow = ControlFlow::Exit,
                        G => window.grab_cursor(!modifiers.shift).unwrap(),
                        H => window.hide_cursor(!modifiers.shift),
                        _ => (),
                    }
                }
                _ => (),
            }
        }
    });
}

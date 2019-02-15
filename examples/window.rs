extern crate winit;
use winit::window::WindowBuilder;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};

mod helpers;

fn main() {
    let event_loop = EventLoop::new();

    let _window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&event_loop)
        .unwrap();

    helpers::init_wayland(&_window);

    event_loop.run(move |event, _, control_flow| {
        println!("{:?}", event);

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::EventsCleared => {
                _window.request_redraw();
                println!("events cleared");
            }
            _ => *control_flow = ControlFlow::Wait,
        }
    });
}

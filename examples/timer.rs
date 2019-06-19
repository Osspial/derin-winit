extern crate winit;
use std::time::{Duration, Instant};
use winit::event::{Event, WindowEvent, StartCause};
use winit::event_loop::{EventLoop, ControlFlow};

fn main() {
    let event_loop = EventLoop::new();

    event_loop.run(move |event, _, control_flow| {
        println!("{:?}", event);

        match event {
            Event::NewEvents(StartCause::Init) =>
                *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::new(1, 0)),
            Event::NewEvents(StartCause::ResumeTimeReached{..}) => {
                *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::new(1, 0));
                println!("\nTimer\n");
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => ()
        }
    });
}

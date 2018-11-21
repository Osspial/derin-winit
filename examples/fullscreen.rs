extern crate winit;

use std::io::{self, Write};
use winit::{ControlFlow, Event, WindowEvent};

fn main() {
    let mut events_loop = winit::EventsLoop::new();

    // enumerating monitors
    let monitor = {
        for (num, monitor) in events_loop.get_available_monitors().enumerate() {
            println!("Monitor #{}: {:?}", num, monitor.get_name());
        }

        print!("Please write the number of the monitor to use: ");
        io::stdout().flush().unwrap();

        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();
        let num = num.trim().parse().ok().expect("Please enter a number");
        let monitor = events_loop.get_available_monitors().nth(num).expect("Please enter a valid ID");

        println!("Using {:?}", monitor.get_name());

        monitor
    };

    let window = winit::WindowBuilder::new()
        .with_title("Hello world!")
        .with_fullscreen(Some(monitor))
        .build(&events_loop)
        .unwrap();

    let mut is_fullscreen = true;
    let mut is_maximized = false;
    let mut decorations = true;
    let mut resizable = true;

    let monitor_2 = events_loop.get_available_monitors().nth(2).unwrap();

    events_loop.run_forever(|event| {
        // println!("{:?}", event);
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => return ControlFlow::Break,
                WindowEvent::Resized(size) => println!("resized {:?}", size.to_physical(window.get_hidpi_factor())),
                WindowEvent::KeyboardInput {
                    input:
                        winit::KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state,
                            ..
                        },
                    ..
                } => match (virtual_code, state) {
                    (winit::VirtualKeyCode::Escape, _) => return ControlFlow::Break,
                    (winit::VirtualKeyCode::F, winit::ElementState::Pressed) => {
                        is_fullscreen = !is_fullscreen;
                        println!("\nset fullscreen");
                        if !is_fullscreen {
                            window.set_fullscreen(None);
                        } else {
                            window.set_fullscreen(Some(monitor_2.clone()));
                        }
                    }
                    (winit::VirtualKeyCode::D, winit::ElementState::Pressed) => {
                        decorations = !decorations;
                        println!("\nset decorations");
                        window.set_decorations(decorations);
                    }
                    (winit::VirtualKeyCode::R, winit::ElementState::Pressed) => {
                        resizable = !resizable;
                        window.set_resizable(resizable);
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => {}
        }

        ControlFlow::Continue
    });
}

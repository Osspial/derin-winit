extern crate winit;
extern crate winapi;
use winit::window::WindowBuilder;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, event_loop, control_flow| {
        println!("{:?}", event);
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::MouseInput {
                    state: winit::event::ElementState::Released,
                    button: winit::event::MouseButton::Right,
                    ..
                },
                ..
            } => unsafe {
                use winit::platform::windows::WindowExtWindows;
                let hwnd = window.get_hwnd();
                event_loop.queue_function(move || {
                    use std::ptr;
                    use winapi::um::winuser::{self, *};

                    let menu = winuser::CreatePopupMenu();
                    winuser::InsertMenuA(menu, 0, MF_BYPOSITION | MF_STRING, 0, "Hello\0".as_ptr() as _);
                    winuser::TrackPopupMenuEx(
                        menu,
                        0,
                        512,
                        512,
                        hwnd as _,
                        ptr::null_mut()
                    );
                    winuser::DestroyMenu(menu);
                });
            }
            _ => *control_flow = ControlFlow::Wait,
        }
    });
}

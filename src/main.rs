use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit::dpi::LogicalSize;

use crate::render::Renderer;

mod render;
mod ecs;

fn main() {
    println!("HELLO HUMAN");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_inner_size(LogicalSize::new(640, 480)).build(&event_loop).unwrap();

    let renderer = Renderer::from_window(&window);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });

    println!("REMAIN INDOORS");
}

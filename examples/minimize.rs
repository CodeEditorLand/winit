extern crate winit;

use simple_logger::SimpleLogger;
use winit::{
	event::{Event, VirtualKeyCode, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};

fn main() {
	SimpleLogger::new().init().unwrap();

	let event_loop = EventLoop::new();

	let window = WindowBuilder::new()
		.with_title("A fantastic window!")
		.build(&event_loop)
		.unwrap();

	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Wait;

		match event {
			Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
				*control_flow = ControlFlow::Exit
			},

			// Keyboard input event to handle minimize via a hotkey
			Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, window_id } => {
				if window_id == window.id() {
					// Pressing the 'M' key will minimize the window
					if input.virtual_keycode == Some(VirtualKeyCode::M) {
						window.set_minimized(true);
					}
				}
			},
			_ => (),
		}
	});
}

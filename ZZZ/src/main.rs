//
//extern crate glfw;
//extern crate gl;
//extern crate nanovg;
//
//use std::comm::{channel, Receiver, Sender};
//use glfw::{Context, WindowEvent};
//
//struct App<'a> {
//	prev_t: f64,
//	w: u32,
//	h: u32,
//	glfw: glfw::Glfw,
//	main_window: glfw::Window,
//	events: Receiver<(f64, WindowEvent)>
//}
//impl<'a> App<'a> {
//	fn new() -> App<'a> {
//	    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
//		glfw.set_time(0.0);
//		let (window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::Windowed)
//	        .expect("Failed to create GLFW window.");
//
//		App {
//			prev_t: glfw.get_time(),
//			w: 640,
//			h: 480,
//			glfw: glfw,
//			main_window: window,
//			events: events
//		}
//	}
//
//
//	fn init(&mut self) {
//	    self.main_window.set_key_polling(true);
//	    self.main_window.make_current();
//	}
//
//	fn run(&mut self) {
////	    while !self.main_window.should_close() {
////	        self.glfw.poll_events(); // wait_events();
////	        for (_, event) in glfw::flush_messages(&self.events) {
////	            self.handle_window_event(&self.main_window, event);
////	        }
////	    }
//	}
//
//	fn handle_window_event(&mut self, window: &glfw::Window, event: glfw::WindowEvent) {
//	    match event {
//	        glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
//	            self.main_window.set_should_close(true)
//	        }
//	        _ => {}
//	    }
//	}
//
//}
//
//fn main() {
//	let mut app = App::new();
//	app.init();
//	//app.run();
//    loop {
//    	if app.main_window.should_close() { break; }
//        app.glfw.poll_events(); // wait_events();
//
//        for (_, event) in glfw::flush_messages(&app.events) {
//            app.handle_window_event(&app.main_window, event);
//        }
//    }
//
//}
//
fn main() {}

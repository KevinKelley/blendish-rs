#![feature(globs)]

extern crate graphics;
extern crate piston;
// extern crate sdl2_game_window;
extern crate glfw_game_window;

extern crate nanovg;
extern crate blendish;

// use Window = sdl2_game_window::GameWindowSDL2;
use Window = glfw_game_window::GameWindowGLFW;
use piston::{
    Game,
    GameIteratorSettings,
    GameWindowSettings,
    UpdateArgs,
    RenderArgs,
    KeyPressArgs,
    KeyReleaseArgs,
    MousePressArgs,
    MouseReleaseArgs,
    MouseMoveArgs,
    MouseRelativeMoveArgs,
    MouseScrollArgs,
};

use blendish::*;
use blendish::WidgetState;
//use blendish::theme::ThemedContext;

mod demodata;
mod demo;

pub struct App<'a> {
    blowup: bool,
    _screenshot: bool,
    _premult: bool,

    demodata: Option<demo::DemoData>,

    theme: ThemedContext<'a>
}

impl<'a> App<'a> {
    /// Creates a new application.
    pub fn new() -> App<'a> {
        App {
            blowup: false,
            _screenshot: false,
            _premult: false,

            demodata: None,

            theme: ThemedContext::new(nanovg::Ctx::create_gL3(nanovg::ANTIALIAS | nanovg::STENCIL_STROKES))
        }
    }
    fn nvg(&mut self) -> &mut nanovg::Ctx { self.theme.nvg() }
}



impl<'a> Game for App<'a> {
    /// Perform tasks for loading before showing anything.
    fn load(&mut self) {
        self.demodata = Some(demo::DemoData::load(self.theme.nvg(), "../../../../res"));
        //self.theme = Some(ThemedContext::new(&self.nvg));
    }

    fn update(&mut self, _args: &UpdateArgs) {
    }
    fn render(&mut self, args: &RenderArgs) {
        //let (mx, my) = window.get_cursor_pos();
        //let (winWidth, winHeight) = window.get_size();
        //let (fbWidth, fbHeight) = window.get_framebuffer_size();
        //// Calculate pixel ration for hi-dpi devices.
        //let pxRatio = fbWidth as f32 / winWidth as f32;

        let (w, h) = (args.width as f32, args.height as f32);
        let pxRatio = 1.0;
        let dt = args.ext_dt as f32;

        self.theme.nvg().begin_frame(w as i32, h as i32, pxRatio);

        match self.demodata {
            Some(ref data) => {
                demo::render_demo(self.theme.nvg(), 0.0,0.0,w,h, dt, self.blowup, data);
            },
            None => {
                println!("waah");
            }
        }
        //self.theme.draw_label(100.0,100.0, 200.0,40.0, -1, "Hello World!");
        //demo::render_demo(&vg, mx as f32,my as f32, winWidth as f32,winHeight as f32, t as f32, blowup, &data);
        //fps.render(&vg, 5.0, 5.0);

        self.theme.nvg().end_frame();
    }

    fn key_press(&mut self,  _args: &KeyPressArgs) {}
    fn key_release(&mut self, _args: &KeyReleaseArgs) {}

    fn mouse_press(&mut self, _args: &MousePressArgs) {}
    fn mouse_release(&mut self, _args: &MouseReleaseArgs) {}
    fn mouse_move(&mut self, _args: &MouseMoveArgs) {}
    /// Moved mouse relative, not bounded by cursor.
    fn mouse_relative_move(&mut self, _args: &MouseRelativeMoveArgs) {}
    fn mouse_scroll(&mut self, _args: &MouseScrollArgs) {}
}


fn main() {
    let mut window = Window::new(
        GameWindowSettings {
            title: "NanoMorph Demo".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
        }
    );

    let mut app = App::new();
    let game_iter_settings = GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    app.run(&mut window, &game_iter_settings);
}

// while !shouldClose {
//     double mx, my;
//     int winWidth, winHeight;
//     int fbWidth, fbHeight;
//     float pxRatio;
//
//     glfwGetCursorPos(window, &mx, &my);
//     glfwGetWindowSize(window, &winWidth, &winHeight);
//     glfwGetFramebufferSize(window, &fbWidth, &fbHeight);
//     // Calculate pixel ration for hi-dpi devices.
//     pxRatio = (float)fbWidth / (float)winWidth;
//
//     // Update and render
//     glViewport(0, 0, fbWidth, fbHeight);
//     glClearColor(0,0,0,1);
//     glClear(GL_COLOR_BUFFER_BIT|GL_DEPTH_BUFFER_BIT|GL_STENCIL_BUFFER_BIT);
//
//     nvgBeginFrame(vg, winWidth, winHeight, pxRatio);
//
//     draw(vg, winWidth, winHeight);
//
//     nvgEndFrame(vg);
//
//     glfwSwapBuffers(window);
//     glfwPollEvents();
// }

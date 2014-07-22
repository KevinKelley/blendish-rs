#![feature(globs)]

extern crate graphics;
extern crate piston;
// extern crate sdl2_game_window;
extern crate glfw_game_window;

extern crate nanovg;
extern crate blendish;


use piston::{
    Game, GameIteratorSettings, GameWindowSettings,
    UpdateArgs, RenderArgs,
    KeyPressArgs, KeyReleaseArgs,
    MousePressArgs, MouseReleaseArgs,
    MouseScrollArgs, MouseMoveArgs, MouseRelativeMoveArgs,
};


use blendish::*;
use nanovg::{Ctx, ANTIALIAS,STENCIL_STROKES, Color, };
pub use Window = glfw_game_window::GameWindowGLFW;

mod eyes;


pub struct App<'a> {
    demodata: Option<bool>,//demo::DemoData>,
    mouse: (i32,i32),
    elapsed_time: f64,
    theme: ThemedContext<'a>
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            demodata: None,
            mouse: (0,0),
            elapsed_time: 0.0,         // time sinze app start
            theme: ThemedContext::new(
                Ctx::create_gL3(ANTIALIAS|STENCIL_STROKES))
        }
    }
    fn nvg(&mut self) -> &mut Ctx { self.theme.nvg() }
}


fn rgba(r:u8, g:u8, b:u8, a:u8) -> Color { Color::rgba(r,g,b, a) }

fn draw_bg(vg: &mut Ctx, x:f32,y:f32,w:f32,h:f32) {
    let paint = vg.linear_gradient(x,y,x,y+15.0, rgba(255,255,255,8), rgba(0,0,0,16));
    vg.begin_path();
    vg.rect(x,y,w,h);
    vg.fill_paint(paint);
    vg.fill();
}
impl<'a> Game for App<'a> {
    fn load(&mut self) {
        self.demodata = Some(true); //demo::DemoData::load(self.nvg(), "../../res"));
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.elapsed_time += args.dt;
    }
    fn render(&mut self, args: &RenderArgs) {
        //let (mx, my) = window.get_cursor_pos();
        //let (winWidth, winHeight) = window.get_size();
        //let (fbWidth, fbHeight) = window.get_framebuffer_size();
        //// Calculate pixel ration for hi-dpi devices.
        //let pxRatio = fbWidth as f32 / winWidth as f32;

        let (w,  h) = (args.width as f32, args.height as f32);
        let pxRatio = 1.0;
        let dt      = args.ext_dt as f32;
        let (mx,my) = self.mouse;
        let _data   = self.demodata.expect("data not loaded!?");

        self.nvg().begin_frame(w as i32, h as i32, pxRatio);

        draw_bg(self.nvg(), 0.0,0.0, w,h);
        eyes::draw_eyes(self.nvg(),
        	60.0, 60.0, 200.0, 120.0,
        	mx as f32,my as f32,
        	dt
        );

        self.nvg().end_frame();
    }

    fn key_press(&mut self,  _args: &KeyPressArgs) {}
    fn key_release(&mut self, _args: &KeyReleaseArgs) {}

    fn mouse_press(&mut self, _args: &MousePressArgs) {}
    fn mouse_release(&mut self, _args: &MouseReleaseArgs) {}
    fn mouse_move(&mut self, args: &MouseMoveArgs) {
        self.mouse = (args.x as i32, args.y as i32);
    }
    /// Moved mouse relative, not bounded by cursor.
    fn mouse_relative_move(&mut self, _args: &MouseRelativeMoveArgs) {}
    fn mouse_scroll(&mut self, _args: &MouseScrollArgs) {}
}


fn main() {
    let mut window = Window::new(
        GameWindowSettings {
            title: "Eyes Have It".to_string(),
            size: [300, 240],
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

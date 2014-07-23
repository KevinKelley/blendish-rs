#![feature(globs)]
#![allow(unused_imports)]

extern crate nanovg;
extern crate blendish;
extern crate graphics;
extern crate piston;
// extern crate sdl2_game_window;
extern crate glfw_game_window;

pub use Window = glfw_game_window::GameWindowGLFW;

use piston::{
    Game, GameIteratorSettings, GameWindowSettings,
    UpdateArgs, RenderArgs,
    KeyPressArgs, KeyReleaseArgs,
    MousePressArgs, MouseReleaseArgs,
    MouseScrollArgs, MouseMoveArgs, MouseRelativeMoveArgs,
};
use nanovg::{Ctx, ANTIALIAS,STENCIL_STROKES, Font,Image };
use blendish::*;
use blendish::lowlevel_draw::LowLevelDraw;
use blendish::themed_draw::ThemedDraw;
use resources::Resources;

mod draw;
mod resources;

///////////////////////////////////////////////////////////////////////
// shorthand fns
//fn rgb(r:u8, g:u8, b:u8) -> Color { Color::rgb(r,g,b) }
fn rgba(r:u8, g:u8, b:u8, a:u8) -> Color { Color::rgba(r,g,b, a) }
// just testing
fn draw_bg(vg: &mut Ctx, x:f32,y:f32,w:f32,h:f32) {
    let paint = vg.linear_gradient(x,y,x,y+15.0, rgba(255,255,255,8), rgba(0,0,0,16));
    vg.begin_path();
    vg.rect(x,y,w,h);
    vg.fill_paint(paint);
    vg.fill();
}
///////////////////////////////////////////////////////////////////////


pub struct App<'a> {
    //resources: Resources,
    mouse: (i32,i32),           // current mouse pos
    elapsed_time: f64,          // seconds since app start
    themed: ThemedContext<'a>    // warp nvg ctx w/ themed-draw fns
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let nvg = Ctx::create_gl3(ANTIALIAS|STENCIL_STROKES);
        let resources = Resources::load(&nvg, "../../res");
        let font = resources.fontNormal;
        let icons = resources.iconsheet;    // move resources into the ThemedContext
        App {
            //resources: resources,
            mouse: (0,0),
            elapsed_time: 0.0,         // time since app start
            themed: ThemedContext::wrap(nvg, icons, font)
        }
    }
    fn nvg(&mut self) -> &mut Ctx { self.themed.nvg() }

    fn theme(&self) -> &Theme { self.themed.theme() }
}

impl<'a> Game for App<'a>
{
    fn load(&mut self) {
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.elapsed_time += args.dt;
    }

    #[allow(unused_variable)]
    fn render(&mut self, args: &RenderArgs) {
        //let (mx, my) = window.get_cursor_pos();
        //let (winWidth, winHeight) = window.get_size();
        //let (fbWidth, fbHeight) = window.get_framebuffer_size();
        //// Calculate pixel ration for hi-dpi devices.
        //let pxRatio = fbWidth as f32 / winWidth as f32;

        let (w,  h) = (args.width as f32, args.height as f32);
        let pxRatio = 1.0;
        let t       = self.elapsed_time as f32;
        let dt      = args.ext_dt as f32;
        let (mx,my) = self.mouse;
        let bg      = self.theme().backgroundColor;

//        let data = match self.resources.as_ref() { //.expect("data not loaded!?");
//            Some(data) => {
//                (
//                    data.fontNormal,
//                    data.fontBold,
//                    data.fontIcons,
//                    data.images
//                )
//            },
//            None => { (-1,-1,-1, [-1, ..12]) }
//        };

        self.nvg().begin_frame(w as i32, h as i32, pxRatio);

//        match data {
//            (_,_,icons,_) => {
                draw_bg(self.nvg(), 0.0,0.0, w,h);
                draw::draw(&mut self.themed, w,h, t);
//            }
            //_ => {
            //    println!("no resources!");
            //}
//        }

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
            title: "Blendish/NanoVG UI demo".to_string(),
            size: [800,600],
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



////////////////////////////////////////////////////////////////////////
// draw fn APIs for ref
//
// theme related (self is ThemedContext)
//fn draw_label(&mut self, xy,w,h:f32, iconid: i32, label: &str);
//fn draw_tool_button        (&mut self, x,y,w,h:f32, flags: CornerFlags, state: WidgetState, iconid: i32, label: &str);
//fn draw_radio_button       (&mut self, x,y,w,h:f32, flags: CornerFlags, state: WidgetState, iconid: i32, label: &str);
//fn draw_text_field         (&mut self, x,y,w,h:f32, flags: CornerFlags, state: WidgetState, iconid: i32, text: &str, cbegin: i32, cend: i32);
//fn draw_option_button      (&mut self, x,y,w,h:f32, state: WidgetState, label: &str);
//fn draw_choice_button      (&mut self, x,y,w,h:f32, flags: CornerFlags, state: WidgetState, iconid: i32, label: &str);
//fn draw_number_field       (&mut self, x,y,w,h:f32, flags: CornerFlags, state: WidgetState, label: &str, value: &str);
//fn draw_slider             (&mut self, x,y,w,h:f32, flags: CornerFlags, state: WidgetState, progress: f32, label: &str, value: &str);
//fn draw_scrollbar          (&mut self, x,y,w,h:f32, state: WidgetState, offset: f32, size: f32);
//fn draw_menu_background    (&mut self, x,y,w,h:f32, flags: CornerFlags);
//fn draw_menu_label         (&mut self, x,y,w,h:f32, iconid: i32, label: &str);
//fn draw_menu_item          (&mut self, x,y,w,h:f32, state: &mut WidgetState, iconid: i32, label: &str);
//fn draw_tooltip_background (&mut self, x,y,w,h:f32);

// context related (self is nanovg::Ctx)
//fn draw_rounded_box   (&mut self, x,y,w,h:f32, cr0,1,2,3: f32);
//fn draw_background    (&mut self, x,y,w,h:f32, bg: Color);
//fn draw_bevel         (&mut self, x,y,w,h:f32, bg: Color);
//fn draw_bevel_inset   (&mut self, x,y,w,h:f32, cr2:cr3: f32, bg: Color);
//fn draw_drop_shadow   (&mut self, x,y,w,h:f32, r: f32, feather: f32, alpha: f32);
//fn draw_inner_box     (&mut self, x,y,w,h:f32, cr0,1,2,3: f32, shade_top: Color, shade_down: Color);
//fn draw_outline_box   (&mut self, x,y,w,h:f32, cr0,1,2,3: f32, color: Color);
//fn draw_check         (&mut self, ox: f32, oy: f32, color: Color);
//fn draw_arrow         (&mut self, x: f32, y: f32, s: f32, color: Color);
//fn draw_up_down_arrow (&mut self, x: f32, y: f32, s: f32, color: Color);
////fn draw_icon             (&mut self, x: f32, y: f32, iconid: i32);
////fn draw_icon_label_value (&mut self, x,y,w,h:f32, iconid: i32, color: Color, align: TextAlignment, fontsize: f32, label: &str, value: Option<&str>);
////fn draw_icon_label_caret (&mut self, x,y,w,h:f32, iconid: i32, color: Color, fontsize: f32, label: &str, caretcolor: Color, cbegin: i32, cend: i32);

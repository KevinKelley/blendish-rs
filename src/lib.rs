#![crate_type = "lib"]

#![feature(globs)]
#![allow(non_snake_case_functions)]
//#![warn(missing_doc)]
#![allow(unused_imports)]
#![allow(unused_variable)]
#![allow(dead_code)]


extern crate libc;
extern crate nanovg;


pub use nanovg::Color;
pub use nanovg::Winding;
pub use CCW = nanovg::CCW;
pub use nanovg::{Image, Font};
pub use constants::*;
pub use theme::ThemedContext;
pub use theme::*;

pub use ICONID = ffi::BND_ICONID;
pub use TextAlignment = nanovg::Align;

mod ffi;
pub mod constants;
pub mod theme;

pub mod lowlevel_draw;
pub mod themed_draw;



pub fn min(a:f32, b:f32) -> f32 { if a<b { a } else { b } }
pub fn max(a:f32, b:f32) -> f32 { if a>b { a } else { b } }
pub fn clamp(v: f32, mn: f32, mx: f32) -> f32 { max(mn, min(mx, v) ) } //if v>mx {mx} else { if v<mn {mn} else {v} }

pub fn rgba_f(r:f32, g:f32, b:f32, a:f32) -> Color { Color::rgba_f(r, g, b, a) }
pub fn black() -> Color { Color::rgba(0,0,0,1) }


// pub fn label_width(ctx: &nanovg::Ctx, iconid: i32, label: &str, font: &Font) -> f32
// pub fn transparent(color: Color) -> Color
// pub fn offset_color(color: Color, delta: i32) -> Color
// pub fn select_corners(radiuses: &mut [f32, ..4], r: f32, flags: CornerFlags)
// pub fn inner_colors(shade_top: &mut Color, shade_down: &mut Color, theme: &WidgetTheme, state: WidgetState, flipActive: bool
// pub fn text_color(theme: &WidgetTheme, state: WidgetState) -> Color
// pub fn scroll_handle_rect(x: &mut f32, y: &mut f32, w: &mut f32, h: &mut f32, offset: f32, size: f32


/// how text on a control is aligned
//#[repr(u32)]
//pub enum TextAlignment
//{
//    LEFT   = ffi::BND_LEFT,
//    CENTER = ffi::BND_CENTER
//}

#[repr(u32)]
#[deriving(Clone, Eq, Hash, PartialEq, Show)]
pub enum WidgetState {
    /// not interacting
    DEFAULT  = ffi::BND_DEFAULT,
    /// the mouse is hovering over the control
    HOVER    = ffi::BND_HOVER,
    /// the widget is activated (pressed) or in an active state (toggled)
    ACTIVE   = ffi::BND_ACTIVE,
}


/// flags indicating which corners are sharp (for grouping widgets)
bitflags!(
    flags CornerFlags: u32 {
        // all corners are round
        static CORNER_NONE         = ffi::BND_CORNER_NONE,
        // sharp top left corner
        static CORNER_TOP_LEFT     = ffi::BND_CORNER_TOP_LEFT,
        // sharp top right corner
        static CORNER_TOP_RIGHT    = ffi::BND_CORNER_TOP_RIGHT,
        // sharp bottom right corner
        static CORNER_DOWN_RIGHT   = ffi::BND_CORNER_DOWN_RIGHT,
        // sharp bottom left corner
        static CORNER_DOWN_LEFT    = ffi::BND_CORNER_DOWN_LEFT,
        // all corners are sharp;
        // you can invert a set of flags using ^= BND_CORNER_ALL
        static CORNER_ALL          = ffi::BND_CORNER_ALL,
        // top border is sharp
        static CORNER_TOP          = ffi::BND_CORNER_TOP,
        // bottom border is sharp
        static CORNER_DOWN         = ffi::BND_CORNER_DOWN,
        // left border is sharp
        static CORNER_LEFT         = ffi::BND_CORNER_LEFT,
        // right border is sharp
        static CORNER_RIGHT        = ffi::BND_CORNER_RIGHT
    }
)


////////////////////////////////////////////////////////////////////////////////

// Estimator Functions
// -------------------
// Use these functions to estimate sizes for widgets with your NVGcontext.

// returns the ideal width for a label with given icon and text
pub fn label_width(ctx: &nanovg::Ctx, iconid: i32, label: &str, font: &Font) -> f32
{
    let mut w = (PAD_LEFT + PAD_RIGHT) as f32;
    if iconid >= 0 {
        w += ICON_SHEET_RES as f32;
    }
    if label.len() > 0 {
        ctx.font_face_id( font);
        ctx.font_size(LABEL_FONT_SIZE);
        w += ctx.text_advance(1.0, 1.0, label);
    }
    return w;
}


// Low Level Functions
// -------------------
// these are part of the implementation detail and can be used to theme
// new kinds of controls in a similar fashion.

// make color transparent using the default alpha value
pub fn transparent(color: Color) -> Color
{
    return rgba_f(
        color.r(),
        color.g(),
        color.b(),
        color.a() * TRANSPARENT_ALPHA
    );
}


// offset a color by a given integer delta in the range -100 to 100
pub fn offset_color(color: Color, delta: i32) -> Color
{
    if delta != 0 {
	    let offset = (delta as f32) / 255.0;
        return rgba_f(
            clamp(color.r()+offset, 0.0, 1.0),
            clamp(color.g()+offset, 0.0, 1.0),
            clamp(color.b()+offset, 0.0, 1.0),
            color.a())
    }
    return color;
}


// assigns radius r to the four entries of array radiuses depending on whether
// the corner is marked as sharp or not; see BNDcornerFlags for possible
// flag values.
pub fn select_corners(radiuses: &mut [f32, ..4], r: f32, flags: CornerFlags)
{
    radiuses[0] = if flags.contains(CORNER_TOP_LEFT  ) {0.0} else {r};
    radiuses[1] = if flags.contains(CORNER_TOP_RIGHT ) {0.0} else {r};
    radiuses[2] = if flags.contains(CORNER_DOWN_RIGHT) {0.0} else {r};
    radiuses[3] = if flags.contains(CORNER_DOWN_LEFT ) {0.0} else {r};
}

// computes the upper and lower gradient colors for the inner box from a widget
// theme and the widgets state. If flipActive is set and the state is
// ACTIVE, the upper and lower colors will be swapped.
pub fn inner_colors(shade_top: &mut Color, shade_down: &mut Color,
    theme: &WidgetTheme, state: WidgetState, flipActive: bool)
{
    match state {
	    //default:
	    DEFAULT => {
	        *shade_top = offset_color(theme.innerColor, theme.shadeTop);
	        *shade_down = offset_color(theme.innerColor, theme.shadeDown);
	    },
	    HOVER => {
	        let color = offset_color(theme.innerColor, HOVER_SHADE);
	        *shade_top = offset_color(color, theme.shadeTop);
	        *shade_down = offset_color(color, theme.shadeDown);
	    },
	    ACTIVE => {
	        *shade_top = offset_color(theme.innerSelectedColor,
	            if flipActive {theme.shadeDown} else {theme.shadeTop});
	        *shade_down = offset_color(theme.innerSelectedColor,
	            if flipActive {theme.shadeTop} else {theme.shadeDown});
	    }
    }
}

// computes the text color for a widget label from a widget theme and the
// widgets state.
pub fn text_color(theme: &WidgetTheme, state: WidgetState) -> Color
{
    return if state == ACTIVE {theme.textSelectedColor} else {theme.textColor};
}


// computes the bounds of the scrollbar handle from the scrollbar size
// and the handles offset and size.
// offset is in the range 0..1 and defines the position of the scroll handle
// size is in the range 0..1 and defines the size of the scroll handle
pub fn scroll_handle_rect(x: &mut f32, y: &mut f32, w: &mut f32, h: &mut f32,
    offset: f32, size: f32
) {
    let size = clamp(size, 0.0, 1.0);
    let offset = clamp(offset, 0.0, 1.0);
    if (*h) > (*w) {
        let hs = max(size*(*h), (*w)+1.0);
        *y = (*y) + ((*h)-hs)*offset;
        *h = hs;
    } else {
        let ws = max(size*(*w), (*h)-1.0);
        *x = (*x) + ((*w)-ws)*offset;
        *w = ws;
    }
}


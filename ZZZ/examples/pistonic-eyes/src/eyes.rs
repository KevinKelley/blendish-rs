use nanovg::{Ctx, Color};
use std::num::pow;

fn min(a: f32, b: f32) -> f32 { if a < b { a } else { b } }
//fn max(a: f32, b: f32) -> f32 { if a > b { a } else { b } }
//fn abs(a: f32) -> f32 { if a >= 0.0 { a } else { -a } }
//fn clamp(a: f32, mn: f32, mx: f32) -> f32 { if a < mn { mn } else { if a > mx { mx } else { a } } }
//fn floor(x: f32) -> f32 { x.floor() }
fn sqrt(x: f32) -> f32 { x.sqrt() }
////fn pow(x: f32, e: uint) -> f32 { std::num::pow(x, e) }
//fn cos(x: f32) -> f32 { x.cos() }
fn sin(x: f32) -> f32 { x.sin() }

fn rgba(r:u8, g:u8, b:u8, a:u8) -> Color { Color::rgba(r,g,b,a) }
//fn hsla(h:f32, s:f32, l:f32, a:u8) -> Color { Color::hsla(h,s,l,a) }


pub fn draw_eyes(vg: &Ctx, x: f32,
            y: f32, w: f32,
            h: f32, mx: f32,
            my: f32, t: f32)
{
	let ex = w *0.23;
	let ey = h * 0.5;
	let lx = x + ex;
	let ly = y + ey;
	let rx = x + w - ex;
	let ry = y + ey;
	let br = min(ex, ey) * 0.5;
	let blink: f32 = 1.0 - pow(sin(t*0.5),200)*0.8;

	let bg = vg.linear_gradient(x,y+h*0.5,x+w*0.1,y+h, rgba(0,0,0,32), rgba(0,0,0,16));
	vg.begin_path();
	vg.ellipse(lx+3.0,ly+16.0, ex,ey);
	vg.ellipse(rx+3.0,ry+16.0, ex,ey);
	vg.fill_paint(bg);
	vg.fill();

	let shadow = vg.linear_gradient(x,y+h*0.25,x+w*0.1,y+h, rgba(220,220,220,255), rgba(128,128,128,255));
	vg.begin_path();
	vg.ellipse(lx,ly, ex,ey);
	vg.ellipse(rx,ry, ex,ey);
	vg.fill_paint(shadow);
	vg.fill();

	let mut dx = (mx - rx) / (ex * 10.0);
	let mut dy = (my - ry) / (ey * 10.0);
	let mut d = sqrt(dx*dx+dy*dy);
	if d > 1.0 {
		dx /= d; dy /= d;
	}
	dx *= ex*0.4;
	dy *= ey*0.5;
	vg.begin_path();
	vg.ellipse(lx+dx,ly+dy+ey*0.25*(1.0-blink), br,br*blink);
	vg.fill_color(rgba(32,32,32,255));
	vg.fill();

	dx = (mx - rx) / (ex * 10.0);
	dy = (my - ry) / (ey * 10.0);
	d = sqrt(dx*dx+dy*dy);
	if d > 1.0 {
		dx /= d; dy /= d;
	}
	dx *= ex*0.4;
	dy *= ey*0.5;
	vg.begin_path();
	vg.ellipse(rx+dx,ry+dy+ey*0.25*(1.0-blink), br,br*blink);
	vg.fill_color(rgba(32,32,32,255));
	vg.fill();

	let lgloss = vg.radial_gradient(lx-ex*0.25,ly-ey*0.5, ex*0.1,ex*0.75, rgba(255,255,255,128), rgba(255,255,255,0));
	vg.begin_path();
	vg.ellipse(lx,ly, ex,ey);
	vg.fill_paint(lgloss);
	vg.fill();

	let rgloss = vg.radial_gradient(rx-ex*0.25,ry-ey*0.5, ex*0.1,ex*0.75, rgba(255,255,255,128), rgba(255,255,255,0));
	vg.begin_path();
	vg.ellipse(rx,ry, ex,ey);
	vg.fill_paint(rgloss);
	vg.fill();
}



use blendish::*;
use blendish::constants::*;
//use blendish::ThemedContext;
use blendish::themed_draw::ThemedDraw;
use blendish::lowlevel_draw::LowLevelDraw;


fn cos(x: f32) -> f32 { x.cos() }
fn sin(x: f32) -> f32 { x.sin() }
fn fmodf(numer: f32, denom: f32) -> f32 {
	let tquot = (numer/denom).trunc();
	numer - tquot * denom
}

fn get_time() -> f32 { 0.0 }
fn icon_id(x:u8, y:u8) -> i32 { ICONID(x,y) as i32 }


pub fn draw(ctx: &mut ThemedContext, w:f32, h:f32, bg: Color)
{
	let wgtheight:f32 = WIDGET_HEIGHT as f32;
	let toolwidth:f32 = TOOL_WIDTH	  as f32;
	let empty_str = "";
	let empty = empty_str.as_slice();

    ctx.nvg().draw_background(0.0, 0.0, w, h, bg);

    let mut x = 10.0;
    let mut y = 10.0;


    ctx.draw_tool_button(x,y, 120.0, wgtheight,CORNER_NONE,DEFAULT,
        icon_id(6, 3), "Default");
    y += 25.0;
    ctx.draw_tool_button(x,y, 120.0, wgtheight,CORNER_NONE,HOVER,
        icon_id(6, 3), "Hovered");
    y += 25.0;
    ctx.draw_tool_button(x,y, 120.0, wgtheight,CORNER_NONE,ACTIVE,
        icon_id(6, 3), "Active");

    y += 40.0;
    ctx.draw_radio_button(x,y, 80.0, wgtheight,CORNER_NONE,DEFAULT,
        -1, "Default");
    y += 25.0;
    ctx.draw_radio_button(x,y, 80.0, wgtheight,CORNER_NONE,HOVER,
        -1, "Hovered");
    y += 25.0;
    ctx.draw_radio_button(x,y, 80.0, wgtheight,CORNER_NONE,ACTIVE,
        -1, "Active");

    y += 25.0;
    ctx.draw_label(x,y, 120.0, wgtheight,-1, "Label:");
    y += wgtheight;
    ctx.draw_choice_button(x,y, 80.0, wgtheight,CORNER_NONE,DEFAULT,
        -1, "Default");
    y += 25.0;
    ctx.draw_choice_button(x,y, 80.0, wgtheight,CORNER_NONE,HOVER,
        -1, "Hovered");
    y += 25.0;
    ctx.draw_choice_button(x,y, 80.0, wgtheight,CORNER_NONE,ACTIVE,
        -1, "Active");

    y += 25.0;
    let mut ry:f32 = y;
    let mut rx:f32 = x;

    y = 10.0;
    x += 130.0;
    ctx.draw_option_button(x,y, 120.0, wgtheight,DEFAULT, "Default");
    y += 25.0;
    ctx.draw_option_button(x,y, 120.0, wgtheight,HOVER, "Hovered");
    y += 25.0;
    ctx.draw_option_button(x,y, 120.0, wgtheight,ACTIVE, "Active");

    y += 40.0;
    ctx.draw_number_field(x,y, 120.0, wgtheight,CORNER_DOWN,DEFAULT,
        "Top", "100");
    y += wgtheight - 2.0;
    ctx.draw_number_field(x,y, 120.0, wgtheight,CORNER_ALL,DEFAULT,
        "Center", "100");
    y += wgtheight - 2.0;
    ctx.draw_number_field(x,y, 120.0, wgtheight,CORNER_TOP,DEFAULT,
        "Bottom", "100");

    let     mx = x-30.0;
    let mut my = y-12.0;
    let     mw = 120.0;
    ctx.draw_menu_background(mx, my, mw, 120.0, CORNER_TOP);
    ctx.draw_menu_label(mx, my, mw, wgtheight,-1, "Menu Title");
    my += wgtheight - 2.0;
    ctx.draw_menu_item(mx, my, mw, wgtheight, &mut DEFAULT,
        icon_id(17, 3), "Default");
    my += wgtheight - 2.0;
    ctx.draw_menu_item(mx, my, mw, wgtheight, &mut HOVER,
        icon_id(18, 3), "Hovered");
    my += wgtheight - 2.0;
    ctx.draw_menu_item(mx, my, mw, wgtheight, &mut ACTIVE,
        icon_id(19, 3), "Active");

    y = 10.0;
    x += 130.0;
    let ox = x;
    ctx.draw_number_field(x,y, 120.0, wgtheight,CORNER_NONE,DEFAULT,
        "Default", "100");
    y += 25.0;
    ctx.draw_number_field(x,y, 120.0, wgtheight,CORNER_NONE,HOVER,
        "Hovered", "100");
    y += 25.0;
    ctx.draw_number_field(x,y, 120.0, wgtheight,CORNER_NONE,ACTIVE,
        "Active", "100");

    y += 40.0;
    ctx.draw_radio_button(x,y, 60.0, wgtheight,CORNER_RIGHT,DEFAULT,
        -1, "One");
    x += 60.0 - 1.0;
    ctx.draw_radio_button(x,y, 60.0, wgtheight,CORNER_ALL,DEFAULT,
        -1, "Two");
    x += 60.0 - 1.0;
    ctx.draw_radio_button(x,y, 60.0, wgtheight,CORNER_ALL,DEFAULT,
        -1, "Three");
    x += 60.0 - 1.0;
    ctx.draw_radio_button(x,y, 60.0, wgtheight,CORNER_LEFT, ACTIVE,
        -1, "Butts");

    x = ox;
    y += 40.0;
    let progress_value:f32 = fmodf(get_time()/10.0, 1.0);
    let progress_label = format!("{}%", (progress_value*100.0+0.5) as int);

    ctx.draw_slider(x,y, 240.0, wgtheight,CORNER_NONE,DEFAULT,
        progress_value, "Default", progress_label.as_slice());
    y += 25.0;
    ctx.draw_slider(x,y, 240.0, wgtheight,CORNER_NONE,HOVER,
        progress_value, "Hovered", progress_label.as_slice());
    y += 25.0;
    ctx.draw_slider(x,y, 240.0, wgtheight,CORNER_NONE,ACTIVE,
        progress_value, "Active", progress_label.as_slice());

    let rw:f32 = x+240.0-rx;
    let s_offset:f32 = sin(get_time()/2.0)*0.5+0.5;
    let s_size  :f32 = cos(get_time()/3.11)*0.5+0.5;

    let scrollbar_h = SCROLLBAR_HEIGHT as f32;
    ctx.draw_scrollbar(rx, ry, rw, scrollbar_h,DEFAULT, s_offset, s_size);
    ry += 20.0;
    ctx.draw_scrollbar(rx, ry, rw, scrollbar_h,HOVER, s_offset, s_size);
    ry += 20.0;
    ctx.draw_scrollbar(rx, ry, rw, scrollbar_h,ACTIVE, s_offset, s_size);

    let edit_text = "The quick brown fox".to_string();
    let textlen =edit_text.len() as i32;
    let t = (get_time()*2.0) as i32;
    let idx1 = ((t/textlen)%textlen) as i32;
    let idx2 = (idx1 + (t%(textlen-idx1))) as i32;

    ry += 25.0;
    let text = edit_text.as_slice();
    ctx.draw_text_field(rx, ry, 240.0, wgtheight,CORNER_NONE,DEFAULT,
        -1, text, idx1, idx2);
    ry += 25.0;
    ctx.draw_text_field(rx, ry, 240.0, wgtheight,CORNER_NONE,HOVER,
        -1, text, idx1, idx2);
    ry += 25.0;
    ctx.draw_text_field(rx, ry, 240.0, wgtheight,CORNER_NONE,ACTIVE,
        -1, text, idx1, idx2);

    rx += rw + 20.0;
    ry = 10.0;
    let scrollbar_w = SCROLLBAR_WIDTH as f32;
    ctx.draw_scrollbar(rx, ry, scrollbar_w, 240.0, DEFAULT, s_offset, s_size);
    rx += 20.0;
    ctx.draw_scrollbar(rx, ry, scrollbar_w, 240.0, HOVER, s_offset, s_size);
    rx += 20.0;
    ctx.draw_scrollbar(rx, ry, scrollbar_w, 240.0, ACTIVE, s_offset, s_size);

    x = ox;
    y += 40.0;
    ctx.draw_tool_button(x,y, toolwidth, wgtheight,CORNER_RIGHT,
        DEFAULT, icon_id(0, 10), empty);
    x += toolwidth - 1.0;
    ctx.draw_tool_button(x,y, toolwidth, wgtheight,CORNER_ALL,
        DEFAULT, icon_id(1, 10), empty);
    x += toolwidth - 1.0;
    ctx.draw_tool_button(x,y, toolwidth, wgtheight,CORNER_ALL,
        DEFAULT, icon_id(2, 10), empty);
    x += toolwidth - 1.0;
    ctx.draw_tool_button(x,y, toolwidth, wgtheight,CORNER_ALL,
        DEFAULT, icon_id(3, 10), empty);
    x += toolwidth - 1.0;
    ctx.draw_tool_button(x,y, toolwidth, wgtheight,CORNER_ALL,
        DEFAULT, icon_id(4, 10), empty);
    x += toolwidth - 1.0;
    ctx.draw_tool_button(x,y, toolwidth, wgtheight,CORNER_LEFT,
        DEFAULT, icon_id(5, 10), empty);
    x += toolwidth - 1.0;
    x += 5.0;
    ctx.draw_radio_button(x,y, toolwidth, wgtheight,CORNER_RIGHT,
        DEFAULT, icon_id(0, 11), empty);
    x += toolwidth - 1.0;
    ctx.draw_radio_button(x,y, toolwidth, wgtheight,CORNER_ALL,
        DEFAULT, icon_id(1, 11), empty);
    x += toolwidth - 1.0;
    ctx.draw_radio_button(x,y, toolwidth, wgtheight,CORNER_ALL,
        DEFAULT, icon_id(2, 11), empty);
    x += toolwidth - 1.0;
    ctx.draw_radio_button(x,y, toolwidth, wgtheight,CORNER_ALL,
        DEFAULT, icon_id(3, 11), empty);
    x += toolwidth - 1.0;
    ctx.draw_radio_button(x,y, toolwidth, wgtheight,CORNER_ALL,
        ACTIVE, icon_id(4, 11), empty);
    x += toolwidth - 1.0;
    ctx.draw_radio_button(x,y, toolwidth, wgtheight,CORNER_LEFT,
        DEFAULT, icon_id(5, 11), "".as_slice());


    // some OUI stuff

    // some persistent variables for demonstration
//    static mut enum1: i32 = 0;
//    static mut progress1:f32 = 0.25;
//    static mut progress2:f32 = 0.75;
//    static mut option1: i32 = 1;
//    static mut option2: i32 = 0;
//    static mut option3: i32 = 0;

//    uiClear();
//
//    let root = panel();
//    // position root element
//    uiSetLayout(0, UI_LEFT|UI_TOP);
//    uiSetMargins(0, 600, 10, 0, 0);
//    uiSetSize(0, 250, 400);
//
//    let col = column(root);
//    uiSetMargins(col, 10, 10, 10, 10);
//    uiSetLayout(col, UI_TOP|UI_HFILL);
//
//    button(col, 1, icon_id(6, 3), "Item 1", demohandler);
//    button(col, 2, icon_id(6, 3), "Item 2", demohandler);
//
//    {
//        let h = hgroup(col);
//        radio(h, 3, icon_id(6, 3), "Item 3.0", &enum1);
//        radio(h, 4, icon_id(0, 10), None, &enum1);
//        radio(h, 5, icon_id(1, 10), None, &enum1);
//        radio(h, 6, icon_id(6, 3), "Item 3.3", &enum1);
//    }
//
//    {
//        let rows = row(col);
//        let coll = vgroup(rows);
//        label(coll, -1, "Items 4.0:");
//        coll = vgroup(coll);
//        button(coll, 7, icon_id(6, 3), "Item 4.0.0", demohandler);
//        button(coll, 8, icon_id(6, 3), "Item 4.0.1", demohandler);
//        let colr = vgroup(rows);
//        uiSetFrozen(colr, option1);
//        label(colr, -1, "Items 4.1:");
//        colr = vgroup(colr);
//        slider(colr, 9, "Item 4.1.0", &progress1);
//        slider(colr, 10, "Item 4.1.1", &progress2);
//    }
//
//    button(col, 11, icon_id(6, 3), "Item 5", None);
//
//    check(col, 12, "Frozen", &option1);
//    check(col, 13, "Item 7", &option2);
//    check(col, 14, "Item 8", &option3);
//
//    uiLayout();
//    drawUI(ctx, 0, 0, 0);
//    uiProcess();
}

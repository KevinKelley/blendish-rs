
use nanovg::{ Ctx, Color, BUTT,MITER };
use super::*;

//////////////////////////////////////////////////////
// NanoVG context extenders (self must be nanovg::Ctx)

pub trait LowLevelDraw
{
    // misc utility

    //fn transparent(color: Color) -> Color;
    //fn offset_color(color: Color, delta: i32) -> Color;
    //fn select_corners(radiuses: [f32, ..4], r: f32, flags: CornerFlags);
    //fn inner_colors(shade_top: &Color, shade_down: &Color, theme: &WidgetTheme, state: WidgetState, flipActive: bool);
    //fn text_color(theme: &WidgetTheme, state: WidgetState) -> Color;
    //fn scroll_handle_rect(x: *const f32, y: *const f32, w: *const f32, h: *const f32, offset: f32, size: f32);

    // context related

    fn draw_rounded_box(&mut self, x:f32,y:f32, w:f32,h:f32, cr0: f32, cr1: f32, cr2: f32, cr3: f32);
    fn draw_background(&mut self, x:f32,y:f32, w:f32,h:f32, bg: Color);
    fn draw_bevel(&mut self, x:f32,y:f32, w:f32,h:f32, bg: Color);
    fn draw_bevel_inset(&mut self, x:f32,y:f32, w:f32,h:f32, cr2: f32, cr3: f32, bg: Color);
    fn draw_drop_shadow(&mut self, x:f32,y:f32, w:f32,h:f32, r: f32, feather: f32, alpha: f32);
    fn draw_inner_box(&mut self, x:f32,y:f32, w:f32,h:f32, cr0: f32, cr1: f32, cr2: f32, cr3: f32, shade_top: Color, shade_down: Color);
    fn draw_outline_box(&mut self, x:f32,y:f32, w:f32,h:f32, cr0: f32, cr1: f32, cr2: f32, cr3: f32, color: Color);
    fn draw_check(&mut self, ox: f32, oy: f32, color: Color);
    fn draw_arrow(&mut self, x: f32, y: f32, s: f32, color: Color);
    fn draw_up_down_arrow(&mut self, x: f32, y: f32, s: f32, color: Color);
    fn draw_icon(&mut self, x: f32, y: f32, iconid: i32);
    fn draw_icon_label_value(&mut self, x:f32,y:f32, w:f32,h:f32, iconid: i32, color: Color, align: TextAlignment, fontsize: f32, label: &str, value: Option<&str>);
    fn draw_icon_label_caret(&mut self, x:f32,y:f32, w:f32,h:f32, iconid: i32, color: Color, fontsize: f32, label: &str, caretcolor: Color, cbegin: i32, cend: i32);
}
impl LowLevelDraw for Ctx {

    // Add a rounded box path at position (x, y) with size (w, h) and a separate
    // radius for each corner listed in clockwise order, so that cr0 = top left,
    // cr1 = top right, cr2 = bottom right, cr3 = bottom left;
    // this is a low level drawing function: the path must be stroked or filled
    // to become visible.
    fn draw_rounded_box(&mut self, x:f32,y:f32, w:f32,h:f32,
        cr0: f32, cr1: f32, cr2: f32, cr3: f32
    ) {
        let w = max(0.0, w);
        let h = max(0.0, h);
        let d = min(w, h);

        self.move_to(x, y+h*0.5);
        self.arc_to(x, y, x+w, y, min(cr0, d/2.0));
        self.arc_to(x+w, y, x+w, y+h, min(cr1, d/2.0));
        self.arc_to(x+w, y+h, x, y+h, min(cr2, d/2.0));
        self.arc_to(x, y+h, x, y, min(cr3, d/2.0));
        self.close_path();
    }

    // Draw a flat panel without any decorations at position (x, y) with size (w, h)
    // and fills it with backgroundColor
    fn draw_background(&mut self, x:f32,y:f32, w:f32,h:f32, bg: Color
    ) {
        self.begin_path();
        self.rect(x, y, w, h);
        self.fill_color(bg);
        self.fill();
    }

    // Draw a beveled border at position (x, y) with size (w, h) shaded with
    // lighter and darker versions of backgroundColor
    fn draw_bevel(&mut self, x:f32,y:f32, w:f32,h:f32, bg: Color
    ) {
        self.stroke_width(1.0);

        let x = x + 0.5;
        let y = y + 0.5;
        let w = w - 1.0;
        let h = h - 1.0;

        self.begin_path();
        self.move_to(x, y+h);
        self.line_to(x+w, y+h);
        self.line_to(x+w, y);
        self.stroke_color(transparent(
            offset_color(bg, -BEVEL_SHADE)));
        self.stroke();

        self.begin_path();
        self.move_to(x, y+h);
        self.line_to(x, y);
        self.line_to(x+w, y);
        self.stroke_color(transparent(
            offset_color(bg, BEVEL_SHADE)));
        self.stroke();
    }

    // Draw a lower inset for a rounded box at position (x, y) with size (w, h)
    // that gives the impression the surface has been pushed in.
    // cr2 and cr3 contain the radiuses of the bottom right and bottom left
    // corners of the rounded box.
    fn draw_bevel_inset(&mut self, x:f32,y:f32, w:f32,h:f32,
        cr2: f32, cr3: f32,
        bg: Color
    ) {
        let y = y - 0.5;
        let d = min(w, h);
        let cr2 = min(cr2, d/2.0);
        let cr3 = min(cr3, d/2.0);

        self.begin_path();
        self.move_to(x+w, y+h-cr2);
        self.arc_to(x+w, y+h, x, y+h, cr2);
        self.arc_to(x, y+h, x, y, cr3);

        let bevelColor = offset_color(bg,
            INSET_BEVEL_SHADE);

        self.stroke_width(1.0);
        self.stroke_paint(
            self.linear_gradient(
                x, y+h-max(cr2, cr3)-1.0,
                x, y+h-1.0,
            rgba_f(bevelColor.r(), bevelColor.g(), bevelColor.b(), 0.0),
            bevelColor));
        self.stroke();
    }

    // Draw a drop shadow around the rounded box at (x, y) with size (w, h) and
    // radius r, with feather as its maximum range in pixels.
    // No shadow will be painted inside the rounded box.
    fn draw_drop_shadow(&mut self, x:f32,y:f32, w:f32,h:f32,
        r: f32, feather: f32, alpha: f32
    ) {
        self.begin_path();

        let mut y = y;
        let mut h = h;
        y += feather;
        h -= feather;

        self.move_to(x-feather, y-feather);
        self.line_to(x, y-feather);
        self.line_to(x, y+h-feather);
        self.arc_to(x, y+h, x+r, y+h, r);
        self.arc_to(x+w, y+h, x+w, y+h-r, r);
        self.line_to(x+w, y-feather);
        self.line_to(x+w+feather, y-feather);
        self.line_to(x+w+feather, y+h+feather);
        self.line_to(x-feather, y+h+feather);
        self.close_path();

        self.fill_paint(self.box_gradient(
            x - feather*0.5, y - feather*0.5,
            w + feather, h+feather,
            r+feather*0.5,
            feather,
            rgba_f(0.0, 0.0, 0.0, alpha*alpha),
            rgba_f(0.0, 0.0, 0.0, 0.0)));
        self.fill();
    }

    // Draw the inner part of a widget box, with a gradient from shade_top to
    // shade_down. If h>w, the gradient will be horizontal instead of
    // vertical.
    fn draw_inner_box(&mut self, x:f32,y:f32, w:f32,h:f32,
        cr0: f32, cr1: f32, cr2: f32, cr3: f32,
        shade_top: Color, shade_down: Color
    ) {
        self.begin_path();
        self.draw_rounded_box(x+1.0, y+1.0, w-2.0, h-3.0,
            max(0.0, cr0-1.0), max(0.0, cr1-1.0), max(0.0, cr2-1.0), max(0.0, cr3-1.0));
        self.fill_paint(
            if (h-2.0)>w  {self.linear_gradient(x, y, x+w, y, shade_top, shade_down)}
            else        {self.linear_gradient(x, y, x, y+h, shade_top, shade_down)});
        self.fill();
    }

    // Draw the outline part of a widget box with the given color
    fn draw_outline_box(&mut self, x:f32,y:f32, w:f32,h:f32,
        cr0: f32, cr1: f32, cr2: f32, cr3: f32, color: Color
    ) {
        self.begin_path();
        self.draw_rounded_box(x+0.5, y+0.5, w-1.0, h-2.0, cr0, cr1, cr2, cr3);
        self.stroke_color(color);
        self.stroke_width(1.0);
        self.stroke();
    }

    // Draw an icon with (x, y) as its upper left coordinate; the iconid selects
    // the icon from the sheet; use the ICONID macro to build icon IDs.
    fn draw_icon(&mut self, x: f32, y: f32, iconid: i32
    ) {
    //    let icons = self.theme().icon_image;
    //    if (icons < 0) {return}  // no icons loaded
    //
    //    let ix = iconid & 0xff;
    //    let iy = (iconid>>8) & 0xff;
    //    let u = (ICON_SHEET_OFFSET_X + ix*ICON_SHEET_GRID) as f32;
    //    let v = (ICON_SHEET_OFFSET_Y + iy*ICON_SHEET_GRID) as f32;
    //
    //    self.begin_path();
    //    self.rect(x, y, ICON_SHEET_RES, ICON_SHEET_RES);
    //    self.fill_paint(
    //        self.image_pattern(x-u, y-v,
    //        ICON_SHEET_WIDTH as f32,
    //        ICON_SHEET_HEIGHT as f32,
    //        0.0, icons, 0.0, 1.0));
    //    self.fill();
    }

    // Draw an optional icon specified by <iconid> and an optional label with
    // given alignment (BNDtextAlignment), fontsize and color within a widget box.
    // if iconid is >= 0, an icon will be drawn and the labels remaining space
    // will be adjusted.
    // if label is not NULL, it will be drawn with the specified alignment, fontsize
    // and color.
    // if value is not NULL, label and value will be drawn with a ":" separator
    // inbetween.
    fn draw_icon_label_value(&mut self, x:f32,y:f32, w:f32,h:f32,
        iconid: i32, color: Color, align: TextAlignment, fontsize: f32, label: &str,
        value: Option<&str>
    ) {
    //    let pleft = PAD_LEFT;
    //    if label {
    //        if iconid >= 0 {
    //            draw_icon(self, x+4.0, y+2.0, iconid);
    //            pleft += ICON_SHEET_RES;
    //        }
    //
    //        if bnd_font < 0 {return};
    //        self.font_face_id(bnd_font);
    //        self.font_size(fontsize);
    //        self.begin_path();
    //        self.fill_color(color);
    //        if value {
    //            let label_width = self.text_bounds(1.0, 1.0, label);
    //            let sep_width = self.text_bounds(1.0, 1.0,
    //                theme::LABEL_SEPARATOR);
    //
    //            self.text_align(nanovg::LEFT|nanovg::BASELINE);
    //            x += pleft as f32;
    //            if (align == CENTER) {
    //                let width = label_width + sep_width
    //                    + self.text_bounds(1.0, 1.0, value);
    //                x += ((w-(PAD_RIGHT-pleft) as f32)-width)*0.5;
    //            }
    //            y += h-TEXT_PAD_DOWN as f32;
    //            self.text(x, y, label);
    //            x += label_width;
    //            self.text(x, y, theme::LABEL_SEPARATOR);
    //            x += sep_width;
    //            self.text(x, y, value);
    //        } else {
    //            self.text_align(
    //                if align==LEFT  {nanovg::LEFT  |nanovg::BASELINE}
    //                else              {nanovg::CENTER|nanovg::BASELINE});
    //            self.text_box(x+pleft as f32, y+h-TEXT_PAD_DOWN as f32,
    //                w-PAD_RIGHT as f32-pleft as f32, label);
    //        }
    //    } else if (iconid >= 0) {
    //        draw_icon(self, x+2.0, y+2.0, iconid);
    //    }
    }

    // Draw an optional icon specified by <iconid>, an optional label and
    // a caret with given fontsize and color within a widget box.
    // if iconid is >= 0, an icon will be drawn and the labels remaining space
    // will be adjusted.
    // if label is not NULL, it will be drawn with the specified alignment, fontsize
    // and color.
    // cbegin must be >= 0 and <= strlen(text) and denotes the beginning of the caret
    // cend must be >= cbegin and <= strlen(text) and denotes the end of the caret
    // if cend < cbegin, then no caret will be drawn
    fn draw_icon_label_caret(&mut self, x:f32,y:f32, w:f32,h:f32,
        iconid: i32, color: Color, fontsize: f32, label: &str,
        caretcolor: Color, cbegin: i32, cend: i32
    ) {
    //    let bounds: [f32, ..4];
    //    let pleft = theme::TEXT_RADIUS;
    //    if (!label) {return};
    //    if (iconid >= 0) {
    //        draw_icon(self, x+4.0, y+2.0, iconid);
    //        pleft += ICON_SHEET_RES as f32;
    //    }
    //
    //    if (bnd_font < 0) {return};
    //
    //    x+=pleft;
    //    y+=h-TEXT_PAD_DOWN as f32;
    //
    //    self.font_face_id(bnd_font);
    //    self.font_size(fontsize);
    //    self.text_align(nanovg::LEFT|nanovg::BASELINE);
    //
    //    if (cend >= cbegin) {
    //        //const char *cb;const char *ce;
    //        let /*static*/ glyphs: [NVGglyphPosition, ..theme::MAX_GLYPHS];
    //        let nglyphs = self.text_glyph_positions(
    //            x, y, label, label+cend+1, glyphs, theme::MAX_GLYPHS);
    //        let c0=glyphs[0].x;
    //        let c1=glyphs[nglyphs-1].x;
    //        let cb = label+cbegin;
    //        let ce = label+cend;
    //        // TODO: this is slow
    //        for i in range(0, nglyphs) {
    //            if (glyphs[i].str == cb) {
    //                c0 = glyphs[i].x;
    //            }
    //            if (glyphs[i].str == ce) {
    //                c1 = glyphs[i].x;
    //            }
    //        }
    //
    //        self.text_bounds(x, y, label, bounds);
    //        self.begin_path();
    //        if (cbegin == cend) {
    //            self.fill_color(nanovg::rgb_f(0.337, 0.502, 0.761));
    //            self.rect(c0-1.0, bounds[1], 2.0, bounds[3]-bounds[1]);
    //        } else {
    //            self.fill_color(caretcolor);
    //            self.rect(c0-1.0, bounds[1], c1-c0+1.0, bounds[3]-bounds[1]);
    //        }
    //        self.fill();
    //    }
    //
    //    self.begin_path();
    //    self.fill_color(color);
    //    self.text_box(x, y, w-theme::TEXT_RADIUS-pleft, label);
    }

    // Draw a checkmark for an option box with the given upper left coordinates
    // (ox, oy) with the specified color.
    fn draw_check(&mut self, ox: f32, oy: f32, color: Color
    ) {
        self.begin_path();
        self.stroke_width(2.0);
        self.stroke_color(color);
        self.line_cap(BUTT);
        self.line_join(MITER);
        self.move_to(ox+4.0, oy+5.0);
        self.line_to(ox+7.0, oy+8.0);
        self.line_to(ox+14.0, oy+1.0);
        self.stroke();
    }


    // Draw a horizontal arrow for a number field with its center at (x, y) and
    // size s; if s is negative, the arrow points to the left.
    fn draw_arrow(&mut self, x: f32, y: f32, s: f32, color: Color
    ) {
        self.begin_path();
        self.move_to(x, y);
        self.line_to(x-s, y+s);
        self.line_to(x-s, y-s);
        self.close_path();
        self.fill_color(color);
        self.fill();
    }

    // Draw an up/down arrow for a choice box with its center at (x, y) and size s
    fn draw_up_down_arrow(&mut self, x: f32, y: f32, s: f32, color: Color
    ) {
        self.begin_path();
        let w = 1.1*s;
        self.move_to(x, y-1.0);
        self.line_to(x+0.5*w, y-s-1.0);
        self.line_to(x+w, y-1.0);
        self.close_path();
        self.move_to(x, y-1.0);
        self.line_to(x+0.5*w, y+s+1.0);
        self.line_to(x+w, y-1.0);
        self.close_path();
        self.fill_color(color);
        self.fill();
    }
}

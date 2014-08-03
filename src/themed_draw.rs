
use nanovg::{Color, LEFT,CENTER };
use super::constants::*;
use super::*;
use super::lowlevel_draw::LowLevelDraw;


pub trait ThemedDraw
{
    fn draw_background(&mut self, x:f32,y:f32, w:f32,h:f32);
    fn draw_bevel(&mut self, x:f32,y:f32, w:f32,h:f32);
    fn draw_label(&mut self, x:f32,y:f32, w:f32,h:f32, iconid: u32, label: &str);
    fn draw_tool_button(&mut self, x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState, iconid: u32, label: &str);
    fn draw_radio_button(&mut self, x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState, iconid: u32, label: &str);
    fn draw_text_field(&mut self, x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState, iconid: u32, text: &str, cbegin: uint, cend: uint);
    fn draw_option_button(&mut self, x:f32,y:f32, w:f32,h:f32, state: WidgetState, label: &str);
    fn draw_choice_button(&mut self, x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState, iconid: u32, label: &str);
    fn draw_number_field(&mut self, x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState, label: &str, value: &str);
    fn draw_slider(&mut self, x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState, progress: f32, label: &str, value: &str);
    fn draw_scrollbar(&mut self, x:f32,y:f32, w:f32,h:f32, state: WidgetState, offset: f32, size: f32);
    fn draw_menu_background(&mut self, x:f32,y:f32, w:f32,h:f32, flags: CornerFlags);
    fn draw_menu_label(&mut self, x:f32,y:f32, w:f32,h:f32, iconid: u32, label: &str);
    fn draw_menu_item(&mut self, x:f32,y:f32, w:f32,h:f32, state: &mut WidgetState, iconid: u32, label: &str);
    fn draw_tooltip_background(&mut self, x:f32,y:f32, w:f32,h:f32);
}

impl<'a> ThemedDraw for ThemedContext<'a>
{
    ////////////////////////////////////////////////////////////////////////////////
    // High Level Functions
    // --------------------
    // Use these functions to draw themed widgets with your NVGcontext.

    // Draw a flat panel without any decorations at position (x, y) with size (w, h)
    // and fills it with theme's backgroundColor
    fn draw_background(&mut self, x:f32,y:f32, w:f32,h:f32)
    {
        let bg = self.theme().backgroundColor;
        self.nvg().draw_background(x,y, w,h, bg);
    }

    // Draw a beveled border at position (x, y) with size (w, h) shaded with
    // lighter and darker versions of backgroundColor
    fn draw_bevel(&mut self, x:f32,y:f32, w:f32,h:f32)
    {
        let bg = self.theme().backgroundColor;
        self.nvg().draw_bevel(x,y, w,h, bg);
    }

    // Draw a label with its lower left origin at (x, y) and size of (w, h).
    // if iconid >= 0, an icon will be added to the widget
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_label(&mut self,
        x:f32,y:f32, w:f32,h:f32, iconid: u32, label: &str
    ) {
        let color = self.theme().regularTheme.textColor;
        let icons = *self.icon_image();
        let font  = *self.font();
        self.nvg().draw_icon_label_value(x, y, w, h, &icons, iconid,
            color, LEFT,
            &font, LABEL_FONT_SIZE, label, None);
    }

    // Draw a tool button  with its lower left origin at (x, y) and size of (w, h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // if iconid >= 0, an icon will be added to the widget
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_tool_button(&mut self,
        x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState,
        iconid: u32, label: &str
    ) {
        let mut cr: [f32, ..4] = [0.0, ..4]; //float cr[4];
        let mut shade_top: Color = black();
        let mut shade_down: Color = black();

        select_corners(&mut cr, TOOL_RADIUS, flags);
        let color = self.theme().backgroundColor;
        self.nvg().draw_bevel_inset(x, y, w, h, cr[2], cr[3], color);
        inner_colors(&mut shade_top, &mut shade_down, &self.theme().toolTheme, state, true);
        self.nvg().draw_inner_box(x, y, w, h, cr[0], cr[1], cr[2], cr[3], shade_top, shade_down);
        let color = self.theme().toolTheme.outlineColor;
        self.nvg().draw_outline_box(x, y, w, h, cr[0], cr[1], cr[2], cr[3],
            transparent(color));
        let color = text_color(&self.theme().toolTheme, state);
        let icons = *self.icon_image();
        let font  = *self.font();
        self.nvg().draw_icon_label_value(x, y, w, h, &icons, iconid,
            color, CENTER,
            &font, LABEL_FONT_SIZE, label, None);
    }

    // Draw a radio button with its lower left origin at (x, y) and size of (w, h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // if iconid >= 0, an icon will be added to the widget
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_radio_button(&mut self,
        x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState,
        iconid: u32, label: &str
    ) {
        let mut cr: [f32, ..4] = [0.0, ..4]; //float cr[4];
        let mut shade_top: Color = black();
        let mut shade_down: Color = black();

        select_corners(&mut cr, OPTION_RADIUS, flags);
        let bg = self.theme().backgroundColor;
        self.nvg().draw_bevel_inset(x, y, w, h, cr[2], cr[3], bg);
        inner_colors(&mut shade_top, &mut shade_down, &self.theme().radioTheme, state, true);
        self.nvg().draw_inner_box(x, y, w, h, cr[0], cr[1], cr[2], cr[3], shade_top, shade_down);
        let outline = self.theme().radioTheme.outlineColor;
        self.nvg().draw_outline_box(x, y, w, h, cr[0], cr[1], cr[2], cr[3],
            transparent(outline));
        let color = text_color(&self.theme().radioTheme, state);
        let icons = *self.icon_image();
        let font  = *self.font();
        self.nvg().draw_icon_label_value(x, y, w, h, &icons, iconid,
            color, CENTER,
            &font, LABEL_FONT_SIZE, label, None);
    }

    // Draw a text field with its lower left origin at (x, y) and size of (w, h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // if iconid >= 0, an icon will be added to the widget
    // if text is not NULL, text will be printed to the widget
    // cbegin must be >= 0 and <= strlen(text) and denotes the beginning of the caret
    // cend must be >= cbegin and <= strlen(text) and denotes the end of the caret
    // if cend < cbegin, then no caret will be drawn
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_text_field(&mut self,
        x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState,
        iconid: u32, text: &str, cbegin: uint, cend: uint
    ) {
        let mut cr: [f32, ..4] = [0.0, ..4]; //float cr[4];
        let mut shade_top: Color = black();
        let mut shade_down: Color = black();

        select_corners(&mut cr, TEXT_RADIUS, flags);
        let bg = self.theme().backgroundColor;
        self.nvg().draw_bevel_inset(x, y, w, h, cr[2], cr[3], bg);
        inner_colors(&mut shade_top, &mut shade_down, &self.theme().textFieldTheme, state, false);
        self.nvg().draw_inner_box(x, y, w, h, cr[0], cr[1], cr[2], cr[3], shade_top, shade_down);
        let outline = self.theme().textFieldTheme.outlineColor;
        self.nvg().draw_outline_box(x, y, w, h, cr[0], cr[1], cr[2], cr[3],
            transparent(outline));
        let mut cend = cend;
        if state != ACTIVE {
            cend = -1;
        }
        let itemcolor = self.theme().textFieldTheme.itemColor;
        let textcolor = text_color(&self.theme().textFieldTheme, state);
        let icons = *self.icon_image();
        let font  = *self.font();
        self.nvg().draw_icon_label_caret(x, y, w, h,
            &icons, iconid,
            textcolor, &font, LABEL_FONT_SIZE, text,
            itemcolor, cbegin, cend);
    }

    // Draw an option button with its lower left origin at (x, y) and size of (w, h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_option_button(&mut self,
        x:f32,y:f32, w:f32,h:f32, state: WidgetState,
        label: &str
    ) {
        let mut shade_top: Color = black();
        let mut shade_down: Color = black();

        let ox = x;
        let oy = y+h-OPTION_HEIGHT-3.0;

        let bg = self.theme().backgroundColor;
        self.nvg().draw_bevel_inset(ox, oy,
            OPTION_WIDTH, OPTION_HEIGHT,
            OPTION_RADIUS, OPTION_RADIUS,
            bg);
        inner_colors(&mut shade_top, &mut shade_down, &self.theme().optionTheme, state, true);
        self.nvg().draw_inner_box(ox, oy,
            OPTION_WIDTH, OPTION_HEIGHT,
            OPTION_RADIUS, OPTION_RADIUS, OPTION_RADIUS, OPTION_RADIUS,
            shade_top, shade_down);
        let color = self.theme().optionTheme.outlineColor;
        self.nvg().draw_outline_box(ox, oy,
            OPTION_WIDTH, OPTION_HEIGHT,
            OPTION_RADIUS, OPTION_RADIUS, OPTION_RADIUS, OPTION_RADIUS,
            transparent(color));
        if state == ACTIVE {
            let color = self.theme().optionTheme.itemColor;
            self.nvg().draw_check(ox, oy, transparent(color));
        }
        let color = text_color(&self.theme().optionTheme, state);
        let icons = *self.icon_image();
        let font  = *self.font();
        self.nvg().draw_icon_label_value(x+12.0, y, w-12.0, h, &icons, -1,
            color, LEFT,
            &font, LABEL_FONT_SIZE, label, None);
    }

    // Draw a choice button with its lower left origin at (x, y) and size of (w, h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // if iconid >= 0, an icon will be added to the widget
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_choice_button(&mut self,
        x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState,
        iconid: u32, label: &str
    ) {
        let mut cr: [f32, ..4] = [0.0, ..4]; //float cr[4];
        let mut shade_top: Color = black();
        let mut shade_down: Color = black();

        select_corners(&mut cr, OPTION_RADIUS, flags);
        let bg = self.theme().backgroundColor;
        self.nvg().draw_bevel_inset(x, y, w, h, cr[2], cr[3], bg);
        inner_colors(&mut shade_top, &mut shade_down, &self.theme().choiceTheme, state, true);
        self.nvg().draw_inner_box(x, y, w, h, cr[0], cr[1], cr[2], cr[3], shade_top, shade_down);
        let color = self.theme().choiceTheme.outlineColor;
        self.nvg().draw_outline_box(x, y, w, h, cr[0], cr[1], cr[2], cr[3],
            transparent(color));
        let color = text_color(&self.theme().choiceTheme, state);
        let icons = *self.icon_image();
        let font  = *self.font();
        self.nvg().draw_icon_label_value(x, y, w, h, &icons, iconid,
            color, LEFT,
            &font, LABEL_FONT_SIZE, label, None);
        let color = self.theme().choiceTheme.itemColor;
        self.nvg().draw_up_down_arrow(x+w-10.0, y+10.0, 5.0,
            transparent(color));
    }

    // Draw a number field with its lower left origin at (x, y) and size of (w, h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // if label is not NULL, a label will be added to the widget
    // if value is not NULL, a value will be added to the widget, along with
    // a ":" separator
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_number_field(&mut self,
        x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState,
        label: &str, value: &str
    ) {
        let mut cr: [f32, ..4] = [0.0, ..4]; //float cr[4];
        let mut shade_top: Color = black();
        let mut shade_down: Color = black();

        select_corners(&mut cr, NUMBER_RADIUS, flags);
        let bg = self.theme().backgroundColor;
        self.nvg().draw_bevel_inset(x, y, w, h, cr[2], cr[3], bg);
        inner_colors(&mut shade_top, &mut shade_down, &self.theme().numberFieldTheme, state, false);
        self.nvg().draw_inner_box(x, y, w, h, cr[0], cr[1], cr[2], cr[3], shade_top, shade_down);
        let color = self.theme().numberFieldTheme.outlineColor;
        self.nvg().draw_outline_box(x, y, w, h, cr[0], cr[1], cr[2], cr[3],
            transparent(color));
        let color = text_color(&self.theme().numberFieldTheme, state);
        let icons = *self.icon_image();
        let font  = *self.font();
        self.nvg().draw_icon_label_value(x, y, w, h, &icons, -1,
            color, CENTER, &font, LABEL_FONT_SIZE, label, Some(value));
        let color = self.theme().numberFieldTheme.itemColor;
        self.nvg().draw_arrow(x+8.0, y+10.0, -NUMBER_ARROW_SIZE,
            transparent(color));
        self.nvg().draw_arrow(x+w-8.0, y+10.0, NUMBER_ARROW_SIZE,
            transparent(color));
    }

    // Draw slider control with its lower left origin at (x, y) and size of (w, h),
    // where flags is one or multiple flags from BNDcornerFlags and state denotes
    // the widgets current UI state.
    // progress must be in the range 0..1 and controls the size of the slider bar
    // if label is not NULL, a label will be added to the widget
    // if value is not NULL, a value will be added to the widget, along with
    // a ":" separator
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_slider(&mut self,
        x:f32,y:f32, w:f32,h:f32, flags: CornerFlags, state: WidgetState,
        progress: f32, label: &str, value: &str
    ) {
        let mut cr: [f32, ..4] = [0.0, ..4]; //float cr[4];
        let mut shade_top: Color = black();
        let mut shade_down: Color = black();

        let bg = self.theme().backgroundColor;
        select_corners(&mut cr, NUMBER_RADIUS, flags);
        self.nvg().draw_bevel_inset(x, y, w, h, cr[2], cr[3], bg);
        inner_colors(&mut shade_top, &mut shade_down, &self.theme().sliderTheme, state, false);
        self.nvg().draw_inner_box(x, y, w, h, cr[0], cr[1], cr[2], cr[3], shade_top, shade_down);

        let top = self.theme().sliderTheme.shadeTop;
        let down = self.theme().sliderTheme.shadeDown;
        if state == ACTIVE {
            shade_top = offset_color(
                self.theme().sliderTheme.itemColor, top);
            shade_down = offset_color(
                self.theme().sliderTheme.itemColor, down);
        } else {
            shade_top = offset_color(
                self.theme().sliderTheme.itemColor, down);
            shade_down = offset_color(
                self.theme().sliderTheme.itemColor, top);
        }
        self.nvg().scissor(x, y, 8.0+(w-8.0)*clamp(progress, 0.0, 1.0), h);
        self.nvg().draw_inner_box(x, y, w, h, cr[0], cr[1], cr[2], cr[3], shade_top, shade_down);
        self.nvg().reset_scissor();

        let outline = self.theme().sliderTheme.outlineColor;
        self.nvg().draw_outline_box(x, y, w, h, cr[0], cr[1], cr[2], cr[3],
            transparent(outline));
        let color = text_color(&self.theme().sliderTheme, state);
        let icons = *self.icon_image();
        let font  = *self.font();
        self.nvg().draw_icon_label_value(x, y, w, h, &icons, -1,
            color, CENTER, &font, LABEL_FONT_SIZE, label, Some(value));
    }

    // Draw scrollbar with its lower left origin at (x, y) and size of (w, h),
    // where state denotes the widgets current UI state.
    // offset is in the range 0..1 and controls the position of the scroll handle
    // size is in the range 0..1 and controls the size of the scroll handle
    // horizontal widget looks best when height is SCROLLBAR_HEIGHT,
    // vertical looks best when width is SCROLLBAR_WIDTH
    fn draw_scrollbar(&mut self,
        x:f32,y:f32, w:f32,h:f32, state: WidgetState,
        offset: f32, size: f32
    ) {
        let bg = self.theme().backgroundColor;
        self.nvg().draw_bevel_inset(x, y, w, h,
            SCROLLBAR_RADIUS, SCROLLBAR_RADIUS,
            bg);
        let top = self.theme().scrollBarTheme.shadeTop;
        let down = self.theme().scrollBarTheme.shadeDown;
        let inner = self.theme().scrollBarTheme.innerColor;
        let outline = self.theme().scrollBarTheme.outlineColor;
        self.nvg().draw_inner_box(x, y, w, h,
            SCROLLBAR_RADIUS, SCROLLBAR_RADIUS,
            SCROLLBAR_RADIUS, SCROLLBAR_RADIUS,
            offset_color(inner, 3*down),
            offset_color(inner, 3*top));
        self.nvg().draw_outline_box(x, y, w, h,
            SCROLLBAR_RADIUS, SCROLLBAR_RADIUS,
            SCROLLBAR_RADIUS, SCROLLBAR_RADIUS,
            transparent(outline));

        let itemcolor = self.theme().scrollBarTheme.itemColor;
        let itemColor = offset_color(
            itemcolor,
            if state == ACTIVE {SCROLLBAR_ACTIVE_SHADE} else {0});

        let mut x = x; let mut y = y;
        let mut w = w; let mut h = h;
        scroll_handle_rect(&mut x, &mut y, &mut w, &mut h, offset, size);

        let top = self.theme().scrollBarTheme.shadeTop;
        let down = self.theme().scrollBarTheme.shadeDown;
        let outline = self.theme().scrollBarTheme.outlineColor;
        self.nvg().draw_inner_box(x, y, w, h,
            SCROLLBAR_RADIUS, SCROLLBAR_RADIUS,
            SCROLLBAR_RADIUS, SCROLLBAR_RADIUS,
            offset_color(itemColor, 3*top),
            offset_color(itemColor, 3*down));
        self.nvg().draw_outline_box(x, y, w, h,
            SCROLLBAR_RADIUS, SCROLLBAR_RADIUS,
            SCROLLBAR_RADIUS, SCROLLBAR_RADIUS,
            transparent(outline));
    }

    // Draw a menu background with its lower left origin at (x, y) and size of (w, h),
    // where flags is one or multiple flags from BNDcornerFlags.
    fn draw_menu_background(&mut self,
        x:f32,y:f32, w:f32,h:f32, flags: CornerFlags
    ) {
        let mut cr: [f32, ..4] = [0.0, ..4]; //float cr[4];
        let mut shade_top: Color = black();
        let mut shade_down: Color = black();

        select_corners(&mut cr, MENU_RADIUS, flags);
        inner_colors(&mut shade_top, &mut shade_down, &self.theme().menuTheme,
            DEFAULT, false);
        self.nvg().draw_inner_box(x, y, w, h+1.0, cr[0], cr[1], cr[2], cr[3], shade_top, shade_down);
        let outline = self.theme().menuTheme.outlineColor;
        self.nvg().draw_outline_box(x, y, w, h+1.0, cr[0], cr[1], cr[2], cr[3],
            transparent(outline));
        self.nvg().draw_drop_shadow(x, y, w, h, MENU_RADIUS,
            SHADOW_FEATHER, SHADOW_ALPHA);
    }

    // Draw a menu label with its lower left origin at (x, y) and size of (w, h).
    // if iconid >= 0, an icon will be added to the widget
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_menu_label(&mut self,
        x:f32,y:f32, w:f32,h:f32, iconid: u32, label: &str
    ) {
        let color = self.theme().menuTheme.textColor;
        let icons = *self.icon_image();
        let font  = *self.font();
        self.nvg().draw_icon_label_value(x, y, w, h, &icons, iconid,
            color, LEFT,
            &font, LABEL_FONT_SIZE, label, None);
    }

    // Draw a menu item with its lower left origin at (x, y) and size of (w, h),
    // where state denotes the widgets current UI state.
    // if iconid >= 0, an icon will be added to the widget
    // if label is not NULL, a label will be added to the widget
    // widget looks best when height is WIDGET_HEIGHT
    fn draw_menu_item(&mut self,
        x:f32,y:f32, w:f32,h:f32, state: &mut WidgetState,
        iconid: u32, label: &str
    ) {
        let top = self.theme().menuItemTheme.shadeTop;
        let down = self.theme().menuItemTheme.shadeDown;
        if *state != DEFAULT {
            let color = self.theme().menuItemTheme.innerSelectedColor;
            self.nvg().draw_inner_box(x, y, w, h, 0.0, 0.0, 0.0, 0.0,
                offset_color(color, top),
                offset_color(color, down));
            *state = ACTIVE;
        }
        let color = text_color(&self.theme().menuItemTheme, *state);
        let icons = *self.icon_image();
        let font  = *self.font();
        self.nvg().draw_icon_label_value(x, y, w, h, &icons, iconid,
            color, LEFT, &font, LABEL_FONT_SIZE, label, None);
    }

    // Draw a tooltip background with its lower left origin at (x, y) and size of (w, h)
    fn draw_tooltip_background(&mut self, x:f32,y:f32, w:f32,h:f32
    ) {
        let mut shade_top = black();
        let mut shade_down = black();

        inner_colors(&mut shade_top, &mut shade_down, &self.theme().tooltipTheme,
            DEFAULT, false);
        self.nvg().draw_inner_box(x, y, w, h+1.0,
            MENU_RADIUS, MENU_RADIUS, MENU_RADIUS, MENU_RADIUS,
            shade_top, shade_down);
        let ocolor = self.theme().tooltipTheme.outlineColor;
        self.nvg().draw_outline_box(x, y, w, h+1.0,
            MENU_RADIUS, MENU_RADIUS, MENU_RADIUS, MENU_RADIUS,
            transparent(ocolor));
        self.nvg().draw_drop_shadow(x, y, w, h, MENU_RADIUS,
            SHADOW_FEATHER, SHADOW_ALPHA);
    }
}

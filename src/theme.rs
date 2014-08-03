use std::rc::Rc;
use nanovg::{Ctx, Color, Font, Image};


/// describes the theme used to draw a single widget or widget box;
/// these values correspond to the same values that can be retrieved from
/// the Theme panel in the Blender preferences
#[repr(C)]
pub struct WidgetTheme
{
    /// color of widget box outline
    pub outlineColor: Color,
    /// color of widget item (meaning changes depending on widget class)
    pub itemColor: Color,
    /// fill color of widget box
    pub innerColor: Color,
    /// fill color of widget box when active
    pub innerSelectedColor: Color,
    /// color of text label
    pub textColor: Color,
    /// color of text label when active
    pub textSelectedColor: Color,
    /// percentage-delta modifier for upper part of gradient (-100 to 100)
    pub shadeTop: i32,
    /// percentage-delta modifier for lower part of gradient (-100 to 100)
    pub shadeDown: i32,
}

/// describes the theme used to draw widgets
#[repr(C)]
pub struct Theme
{
    /// the background color of panels and windows
    pub backgroundColor: Color,

    /// theme for labels
    pub regularTheme: WidgetTheme,
    /// theme for tool buttons
    pub toolTheme: WidgetTheme,
    /// theme for radio buttons
    pub radioTheme: WidgetTheme,
    /// theme for text fields
    pub textFieldTheme: WidgetTheme,
    /// theme for option buttons (checkboxes)
    pub optionTheme: WidgetTheme,
    /// theme for choice buttons (comboboxes)
    /// Blender calls them "menu buttons"
    pub choiceTheme: WidgetTheme,
    /// theme for number fields
    pub numberFieldTheme: WidgetTheme,
    /// theme for slider controls
    pub sliderTheme: WidgetTheme,
    /// theme for scrollbars
    pub scrollBarTheme: WidgetTheme,
    /// theme for tooltips
    pub tooltipTheme: WidgetTheme,
    /// theme for menu backgrounds
    pub menuTheme: WidgetTheme,
    /// theme for menu items
    pub menuItemTheme: WidgetTheme,
}


pub trait Themed<'a> {
    fn theme(&'a self) -> &'a Theme;
    fn icon_images_handle(&'a self) -> Rc<Image>;
    fn font_handle(&'a self) -> Rc<Font>;
}

////////////////////////////////////////////////////////////////////////////////
/// extends a nanovg context with theming

pub struct ThemedContext<'a>
{
    theme: Theme,

    // nanovg rendering context (owned)
    nvg: Ctx,

    // handle, icon image spritesheet
    icon_image: Rc<Image>,

    // handle
    font: Rc<Font>,

}

impl<'a> ThemedContext<'a> {
    pub fn wrap(nvg: Ctx, icon_sheet: Image, font: Font) -> ThemedContext<'a> {
        ThemedContext {
            nvg: nvg,
            theme: initial_theme(),
            icon_image: Rc::new(icon_sheet),
            font: Rc::new(font)
        }
    }

    pub fn nvg(&mut self) -> &mut Ctx { &mut self.nvg }

    pub fn theme(&self) -> &Theme { &self.theme }

    /// give out refcounted clones -- when all have expired, and ours too,
    /// the resource will be 'drop'ed.
    pub fn font(&self) -> Rc<Font> { self.font.clone() }

    /// give out refcounted clones to client code
    pub fn icon_image(&self) -> Rc<Image> { self.icon_image.clone() }


    pub fn set_theme(&mut self, theme: Theme) { self.theme = theme }
    /// take ownership, wrap in ref-counted box.
    pub fn set_icon_image(&mut self, icons: Image) { self.icon_image = Rc::new(icons) }
    /// take ownership, wrap in ref-counted box.
    pub fn set_font(&mut self, font: Font) { self.font = Rc::new(font) }
}

impl<'a> Themed<'a> for ThemedContext<'a>
{
    fn theme(&'a self) -> &'a Theme { self.theme() }

    fn icon_images_handle<'b>(&self) -> Rc<Image> { self.icon_image() }

    fn font_handle<'c>(&self) -> Rc<Font> { self.font() }
}


////////////////////////////////////////////////////////////////////////////////

fn rgba_f(r:f32, g:f32, b:f32, a:f32) -> Color { Color::rgba_f(r, g, b, a) }


// the initial theme mimics Blender
pub fn initial_theme() -> Theme {
    // default text color
    let text_color_normal: Color = rgba_f( 0.0, 0.0, 0.0, 1.0);
    // default highlighted text color
    let text_color_sel: Color = rgba_f( 1.0, 1.0, 1.0, 1.0);

    Theme
    {
        backgroundColor: rgba_f( 0.447, 0.447, 0.447, 1.0 ),

        regularTheme: WidgetTheme {
            outlineColor: rgba_f( 0.098, 0.098, 0.098, 1.0 ),
            itemColor: rgba_f( 0.098, 0.098, 0.098, 1.0 ),
            innerColor: rgba_f( 0.6, 0.6, 0.6, 1.0 ),
            innerSelectedColor: rgba_f( 0.392, 0.392, 0.392, 1.0 ),
            textColor: text_color_normal,
            textSelectedColor: text_color_sel,
            shadeTop: 0,
            shadeDown: 0,
        },
        toolTheme: WidgetTheme {
            outlineColor: rgba_f( 0.098, 0.098, 0.098, 1.0 ),
            itemColor: rgba_f( 0.098, 0.098, 0.098, 1.0 ),
            innerColor: rgba_f( 0.6, 0.6, 0.6, 1.0 ),
            innerSelectedColor: rgba_f( 0.392, 0.392, 0.392, 1.0 ),
            textColor: text_color_normal,
            textSelectedColor: text_color_sel,
            shadeTop: 15,
            shadeDown: -15,
        },
        radioTheme: WidgetTheme {
            outlineColor: rgba_f( 0.0, 0.0, 0.0, 1.0 ),
            itemColor: rgba_f( 1.0, 1.0, 1.0, 1.0 ),
            innerColor: rgba_f( 0.275, 0.275, 0.275, 1.0 ),
            innerSelectedColor: rgba_f( 0.337, 0.502, 0.761, 1.0 ),
            textColor: text_color_sel,
            textSelectedColor: text_color_normal,
            shadeTop: 15,
            shadeDown: -15,
        },
        textFieldTheme: WidgetTheme {
            outlineColor: rgba_f( 0.098, 0.098, 0.098, 1.0 ),
            itemColor: rgba_f( 0.353, 0.353, 0.353, 1.0 ),
            innerColor: rgba_f( 0.6, 0.6, 0.6, 1.0 ),
            innerSelectedColor: rgba_f( 0.6, 0.6, 0.6, 1.0 ),
            textColor: text_color_normal,
            textSelectedColor: text_color_sel,
            shadeTop: 0,
            shadeDown: 25,
        },
        optionTheme: WidgetTheme {
            outlineColor: rgba_f( 0.0, 0.0, 0.0, 1.0 ),
            itemColor: rgba_f( 1.0, 1.0, 1.0, 1.0 ),
            innerColor: rgba_f( 0.275, 0.275, 0.275, 1.0 ),
            innerSelectedColor: rgba_f( 0.275, 0.275, 0.275, 1.0 ),
            textColor: text_color_normal,
            textSelectedColor: text_color_sel,
            shadeTop: 15,
            shadeDown: -15,
        },
        choiceTheme: WidgetTheme {
            outlineColor: rgba_f( 0.0, 0.0, 0.0, 1.0 ),
            itemColor: rgba_f( 1.0, 1.0, 1.0, 1.0 ),
            innerColor: rgba_f( 0.275, 0.275, 0.275, 1.0 ),
            innerSelectedColor: rgba_f( 0.275, 0.275, 0.275, 1.0 ),
            textColor: text_color_sel,
            textSelectedColor: rgba_f( 0.8, 0.8, 0.8, 1.0 ),
            shadeTop: 15,
            shadeDown: -15,
        },
        numberFieldTheme: WidgetTheme {
            outlineColor: rgba_f( 0.098, 0.098, 0.098, 1.0 ),
            itemColor: rgba_f( 0.353, 0.353, 0.353, 1.0 ),
            innerColor: rgba_f( 0.706, 0.706, 0.706, 1.0 ),
            innerSelectedColor: rgba_f( 0.6, 0.6, 0.6, 1.0 ),
            textColor: text_color_normal,
            textSelectedColor: text_color_sel,
            shadeTop: -20,
            shadeDown: 0,
        },
        sliderTheme: WidgetTheme {
            outlineColor: rgba_f( 0.098, 0.098, 0.098, 1.0 ),
            itemColor: rgba_f( 0.502, 0.502, 0.502, 1.0 ),
            innerColor: rgba_f( 0.706, 0.706, 0.706, 1.0 ),
            innerSelectedColor: rgba_f( 0.6, 0.6, 0.6, 1.0 ),
            textColor: text_color_normal,
            textSelectedColor: text_color_sel,
            shadeTop: -20,
            shadeDown: 0,
        },
        scrollBarTheme: WidgetTheme {
            outlineColor: rgba_f( 0.196, 0.196, 0.196, 1.0 ),
            itemColor: rgba_f( 0.502, 0.502, 0.502, 1.0 ),
            innerColor: rgba_f( 0.314, 0.314, 0.314, 0.706 ),
            innerSelectedColor: rgba_f( 0.392, 0.392, 0.392, 0.706 ),
            textColor: text_color_normal,
            textSelectedColor: text_color_sel,
            shadeTop: 5,
            shadeDown: -5,
        },
        tooltipTheme: WidgetTheme {
            outlineColor: rgba_f( 0.0, 0.0, 0.0, 1.0 ),
            itemColor: rgba_f( 0.392, 0.392, 0.392, 1.0 ),
            innerColor: rgba_f( 0.098, 0.098, 0.098, 0.902 ),
            innerSelectedColor: rgba_f( 0.176, 0.176, 0.176, 0.902 ),
            textColor: rgba_f( 0.627, 0.627, 0.627, 1.0 ),
            textSelectedColor: text_color_sel,
            shadeTop: 0,
            shadeDown: 0,
        },
        menuTheme: WidgetTheme {
            outlineColor: rgba_f( 0.0, 0.0, 0.0, 1.0 ),
            itemColor: rgba_f( 0.392, 0.392, 0.392, 1.0 ),
            innerColor: rgba_f( 0.098, 0.098, 0.098, 0.902 ),
            innerSelectedColor: rgba_f( 0.176, 0.176, 0.176, 0.902 ),
            textColor: rgba_f( 0.627, 0.627, 0.627, 1.0 ),
            textSelectedColor: text_color_sel,
            shadeTop: 0,
            shadeDown: 0,
        },
        menuItemTheme: WidgetTheme {
            outlineColor: rgba_f( 0.0, 0.0, 0.0, 1.0 ),
            itemColor: rgba_f( 0.675, 0.675, 0.675, 0.502 ),
            innerColor: rgba_f( 0.0, 0.0, 0.0, 0.0 ),
            innerSelectedColor: rgba_f( 0.337, 0.502, 0.761, 1.0 ),
            textColor: text_color_sel,
            textSelectedColor: text_color_normal,
            shadeTop: 38,
            shadeDown: 0,
        },
    }
}

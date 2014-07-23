/// use unicode characters for icons
static NO_ICON: 		   char = '\0';
static ICON_SEARCH:        char = '\U0001F50D';
static ICON_CIRCLED_CROSS: char = '\u2716';
static ICON_CHEVRON_RIGHT: char = '\uE75E';
static ICON_CHECK:         char = '\u2713';
static ICON_LOGIN:         char = '\uE740';
static ICON_TRASH:         char = '\uE729';

pub struct DemoData {
	//vg: &Ctx,
	fontNormal: i32,
	fontBold: i32,
	fontIcons: i32,
	images: [i32, ..12],
}

/// load and hold resources used in demo
impl DemoData
{
	pub fn load(vg: &Ctx, respath: &str) -> DemoData
	{
		let mut data = DemoData {
			//vg: vg,
			fontNormal: -1,
			fontBold:   -1,
			fontIcons:  -1,
			images: [-1, ..12]
		};

		for i in range(0, 12u) {
			let filename = format!("{}/images/image{}.jpg", respath, i+1);
			data.images[i] = vg.create_image(filename.as_slice());
			if data.images[i] == 0 {
				println!("Could not load {}.", filename);
			}
		}

		data.fontIcons = vg.create_font("icons", format!("{}/entypo.ttf", respath).as_slice());
		if data.fontIcons == -1 {
			println!("Could not add font 'icons'.");
		}
		data.fontNormal = vg.create_font("sans", format!("{}/Roboto-Regular.ttf", respath).as_slice());
		if data.fontNormal == -1 {
			println!("Could not add font 'sans'.");
		}
		data.fontBold = vg.create_font("sans-bold", format!("{}/Roboto-Bold.ttf", respath).as_slice());
		if data.fontBold == -1 {
			println!("Could not add font 'sans-bold'.");
		}

		return data;
	}
}

impl Drop for DemoData {
	fn drop(&mut self) {
		for i in range(0, 12u) {
			// need to borrow & hold nanovg context, or
			// need to be able to pass it to 'drop'
//			self.vg.delete_image(self.images[i]);
			self.images[i] = -1;
		}
	}
}

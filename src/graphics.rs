use eframe::epaint::Color32;

pub fn parse_colour(col: Option<String>, default_colour: Color32) -> Color32 {
	if let Some(colour_string) = col {
		if let Ok(mut col_raw) = i64::from_str_radix(&colour_string, 16) {
			let a = col_raw % 256;
			col_raw /= 256; // a < 256, so there is no need to subtract it before division as it can only create a decimal part which is dropped in integer division
			let b = col_raw % 256;
			col_raw /= 256;
			let g = col_raw % 256;
			col_raw /= 256;
			let r = col_raw;
			Color32::from_rgba_premultiplied(r as u8, g as u8, b as u8, a as u8)
		} else {
			default_colour
		}
	} else {
		default_colour
	}
}

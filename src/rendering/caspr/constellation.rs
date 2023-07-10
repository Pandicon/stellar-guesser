use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct BorderVertex {
	pub constellation: String,
	pub ra: f32,
	pub dec: f32,
}

impl BorderVertex {
	pub fn get_position(&self) -> (f32, f32) {
		(self.ra, self.dec)
	}
}

#[derive(Clone, Deserialize)]
pub struct ConstellationRaw {
	pub name_latin: String,
	pub abbreviation: String,
}

pub struct Constellation {
	pub possible_names: Vec<String>,
	pub vertices: Vec<(f32, f32)>,
}

impl Constellation {
	pub fn from_raw(raw: ConstellationRaw) -> (Self, String) {
		let abbreviation = raw.abbreviation.to_owned();
		(
			Self {
				possible_names: vec![raw.name_latin, abbreviation.to_owned()],
				vertices: Vec::new(),
			},
			abbreviation,
		)
	}
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConstellationData {
	pub name_latin: String,
	pub abbreviation: String,
}

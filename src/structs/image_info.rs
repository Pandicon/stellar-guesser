use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct DeepskyObjectImageInfo {
	pub object_designation: String,
	pub image: String,
	pub image_source: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct ImageInfo {
	pub path: String,
	pub source: Option<String>,
}

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct DeepskyObjectImageInfo {
    pub object_id: u64,
    pub image: String,
    pub image_source: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct ImageInfo {
    pub path: String,
    pub source: Option<String>,
}

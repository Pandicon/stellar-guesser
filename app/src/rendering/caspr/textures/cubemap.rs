#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Cubemap<T> {
    pub texture_size: u32,
    pub texture_data: Vec<Vec<T>>,

    pub changed: bool,
}

impl<T> Default for Cubemap<T> {
    fn default() -> Self {
        Self {
            texture_size: 0,
            texture_data: Vec::new(),

            changed: false,
        }
    }
}

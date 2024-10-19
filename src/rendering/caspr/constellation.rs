use serde::Deserialize;
use spherical_geometry::{Polygon, SphericalPoint};

#[derive(Clone, Deserialize)]
pub struct BorderVertex {
    pub constellation: String,
    pub ra: f32,
    pub dec: f32,
}

impl BorderVertex {
    pub fn get_position(&self) -> SphericalPoint {
        SphericalPoint::new(self.ra, self.dec)
    }
}

#[derive(Clone, Deserialize)]
pub struct ConstellationRaw {
    pub name_latin: String,
    pub abbreviation: String,
}

pub struct Constellation {
    pub abbreviation: String,
    /// \[abbreviation, latin name, ...\]
    pub possible_names: Vec<String>,
    pub polygons: Vec<Polygon>,
}

impl Constellation {
    pub fn from_raw(raw: ConstellationRaw) -> (Self, String) {
        let abbreviation = raw.abbreviation;
        (
            Self {
                abbreviation: abbreviation.clone(),
                possible_names: vec![abbreviation.clone(), raw.name_latin],
                polygons: Vec::new(),
            },
            abbreviation,
        )
    }
}

use angle::Angle;
use serde::Deserialize;
use spherical_geometry::{Polygon, SphericalPoint};

#[derive(Clone, Deserialize)]
pub struct BorderVertex {
    pub constellation: String,
    pub ra: angle::Deg<f32>,
    pub dec: angle::Deg<f32>,
}

impl BorderVertex {
    pub fn get_position(&self) -> SphericalPoint {
        SphericalPoint::new(self.ra.to_rad().value(), self.dec.to_rad().value())
    }
}

#[derive(Deserialize)]
pub struct ConstellationRaw {
    pub name_latin: String,
    pub abbreviation: String,
    pub polygons: String,
}

pub struct Constellation {
    pub abbreviation: String,
    /// \[abbreviation, latin name, ...\]
    pub possible_names: Vec<String>,
    pub polygons: Vec<Polygon>,
}

impl Constellation {
    pub fn from_raw(raw: ConstellationRaw) -> Result<(Self, String), Box<dyn std::error::Error>> {
		let polygons = match serde_json::from_str(&raw.polygons) {
                Ok(polygons) => {
                    log::debug!("Created the polygon for the {} constellation", raw.name_latin);
                    polygons
                }
                Err(err) => {
                    log::error!("Failed to create polygons for the {} constellation: {:?}", raw.name_latin, err);
                    return Err(Box::from(format!("Failed to create polygons for the {} constellation: {:?}", raw.name_latin, err)));
                }
            };
        let abbreviation = raw.abbreviation;
        Ok((
            Self {
                abbreviation: abbreviation.clone(),
                possible_names: vec![abbreviation.clone(), raw.name_latin],
                polygons,
            },
            abbreviation,
        ))
    }
}

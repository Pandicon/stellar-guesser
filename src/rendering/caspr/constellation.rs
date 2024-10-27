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

#[derive(Clone, Deserialize)]
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
        let mut polygons = Vec::new();
        for polygon_raw in raw.polygons.split('#') {
            let mut vertices = Vec::new();
            for ra_dec_str in polygon_raw.split('|') {
                let spl = ra_dec_str.split(';').collect::<Vec<&str>>();
                if spl.len() < 2 {
                    return Err(Box::from(format!("Missing ra and/or dec for a vertex in the {} constellation", raw.name_latin)));
                }
                let ra = angle::Deg(spl[0].parse::<f32>()?);
                let dec = angle::Deg(spl[1].parse::<f32>()?);
                vertices.push(SphericalPoint::new(ra.to_rad().value(), dec.to_rad().value()));
            }
            match Polygon::new(vertices, spherical_geometry::EdgeDirection::CounterClockwise) {
                Ok(polygon) => {
                    log::debug!("Created the polygon for the {} constellation", raw.name_latin);
                    polygons.push(polygon);
                }
                Err(err) => {
                    log::error!("Failed to create the polygon for the {} constellation: {:?}", raw.name_latin, err);
                    return Err(Box::from(format!("Failed to create the polygon for the {} constellation: {:?}", raw.name_latin, err)));
                }
            }
        }
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

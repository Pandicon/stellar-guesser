pub mod game_handler;
pub mod game_settings;
pub mod questions;
pub mod questions_filter;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(tag = "object_category", content = "object_type")]
pub enum ObjectType {
    Star(StarType),
    Deepsky(DeepskyType),
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum StarType {
    Single,
    Double,
    Multiple,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum DeepskyType {
    Nebula,
    PlanetaryNebula,
    OpenCluster,
    GlobularCluster,
    Galaxy,

    Unknown,
}

impl DeepskyType {
    pub fn to_option_string(&self) -> Option<String> {
        match self {
            &Self::Nebula => Some(String::from("Nebula")),
            &Self::PlanetaryNebula => Some(String::from("Planetary nebula")),
            &Self::OpenCluster => Some(String::from("Open cluster")),
            &Self::GlobularCluster => Some(String::from("Globular cluster")),
            &Self::Galaxy => Some(String::from("Galaxy")),
            &Self::Unknown => None,
        }
    }
}

#[derive(serde::Deserialize)]
pub struct QuestionObjectRaw {
    pub object_id: u64,

    #[serde(flatten)]
    pub object_type: ObjectType,

    pub dec: angle::Deg<f32>,
    pub ra: angle::Deg<f32>,
    pub proper_names: String,
    pub bayer_designation: Option<String>,
    pub flamsteed_designation: Option<String>,
    pub hipparcos_number: Option<u32>,
    pub hd_number: Option<u32>,
    pub messier_number: Option<u32>,
    pub caldwell_number: Option<u32>,
    pub ngc_number: Option<u32>,
    pub ic_number: Option<u32>,
    pub constellations_abbreviations: String,
    pub colour: Option<String>,
    pub mag: Option<f32>,
    pub distance: Option<f32>,
    pub bv: Option<f32>,
}

pub struct QuestionObject {
    pub object_id: u64,
    pub object_type: ObjectType,
    pub dec: angle::Deg<f32>,
    pub ra: angle::Deg<f32>,
    pub proper_names: Vec<String>,
    pub bayer_designation: Option<String>,
    pub flamsteed_designation: Option<String>,
    pub hipparcos_number: Option<u32>,
    pub hd_number: Option<u32>,
    pub messier_number: Option<u32>,
    pub caldwell_number: Option<u32>,
    pub ngc_number: Option<u32>,
    pub ic_number: Option<u32>,
    pub constellations_abbreviations: Vec<String>,
    pub colour: Option<String>,
    pub mag: Option<f32>,
    pub distance: Option<f32>,
    pub bv: Option<f32>,
}

impl QuestionObject {
    pub fn from_raw(raw: QuestionObjectRaw) -> Self {
        Self {
            object_id: raw.object_id,
            object_type: raw.object_type,
            dec: raw.dec,
            ra: raw.ra,
            proper_names: raw.proper_names.split(';').map(|s| s.to_owned()).collect(),
            bayer_designation: raw.bayer_designation,
            flamsteed_designation: raw.flamsteed_designation,
            hipparcos_number: raw.hipparcos_number,
            hd_number: raw.hd_number,
            messier_number: raw.messier_number,
            caldwell_number: raw.caldwell_number,
            ngc_number: raw.ngc_number,
            ic_number: raw.ic_number,
            constellations_abbreviations: raw.constellations_abbreviations.split('|').map(|s| s.to_owned()).collect(),
            colour: raw.colour,
            mag: raw.mag,
            distance: raw.distance,
            bv: raw.bv,
        }
    }
}

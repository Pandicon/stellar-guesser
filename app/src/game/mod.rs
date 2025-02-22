pub mod game_handler;
pub mod game_settings;
pub mod questions;
pub mod questions_filter;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(tag = "object_category", content = "object_type")]
pub enum ObjectType {
    Star(StarType),
    Deepsky(DeepskyType),
}

pub const ALLOWED_TYPES: &str = "STAR, STAR(SINGLE), STAR(DOUBLE), STAR(MULTIPLE), STAR(UNKNOWN), DEEPSKY, DEEPSKY(NEBULA), DEEPSKY(PLANETARY_NEBULA), DEEPSKY(OPEN_CLUSTER), DEEPSKY(GLOBULAR_CLUSTER), DEEPSKY(GALAXY), DEEPSKY(UNKNOWN)";
impl ObjectType {
    pub fn from_string(string: &str) -> Result<Self, String> {
        match string.to_uppercase().as_str() {
            "STAR" => Ok(Self::Star(StarType::Any)),
            "STAR(SINGLE)" => Ok(Self::Star(StarType::Single)),
            "STAR(DOUBLE)" => Ok(Self::Star(StarType::Double)),
            "STAR(MULTIPLE)" => Ok(Self::Star(StarType::Multiple)),
            "STAR(UNKNOWN)" => Ok(Self::Star(StarType::Unknown)),

            "DEEPSKY" => Ok(Self::Deepsky(DeepskyType::Any)),
            "DEEPSKY(NEBULA)" => Ok(Self::Deepsky(DeepskyType::Nebula)),
            "DEEPSKY(PLANETARY_NEBULA)" => Ok(Self::Deepsky(DeepskyType::PlanetaryNebula)),
            "DEEPSKY(OPEN_CLUSTER)" => Ok(Self::Deepsky(DeepskyType::OpenCluster)),
            "DEEPSKY(GLOBULAR_CLUSTER)" => Ok(Self::Deepsky(DeepskyType::GlobularCluster)),
            "DEEPSKY(GALAXY)" => Ok(Self::Deepsky(DeepskyType::Galaxy)),
            "DEEPSKY(UNKNOWN)" => Ok(Self::Deepsky(DeepskyType::Unknown)),

            _ => Err(format!("Unknown object type '{string}'. Allowed types are: {ALLOWED_TYPES}")),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum StarType {
    Single,
    Double,
    Multiple,

    Unknown,
    Any,
}

impl StarType {
    pub fn to_option_string(&self) -> Option<String> {
        match self {
            &Self::Single => Some(String::from("Star")),
            &Self::Double => Some(String::from("Double star")),
            &Self::Multiple => Some(String::from("Multiple star")),
            &Self::Unknown | &Self::Any => None,
        }
    }

    pub fn display_name(&self) -> String {
        self.to_option_string().unwrap_or(String::from("Star"))
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum DeepskyType {
    DarkNebula,
    DiffuseNebula,
    Nebula,
    PlanetaryNebula,
    OpenCluster,
    GlobularCluster,
    Galaxy,
    SupernovaRemnant,
    StarCloud,

    Unknown,
    Any,
}

impl DeepskyType {
    pub fn to_option_string(&self) -> Option<String> {
        match self {
            &Self::DarkNebula => Some(String::from("Dark nebula")),
            &Self::DiffuseNebula => Some(String::from("Diffuse nebula")),
            &Self::Nebula => Some(String::from("Nebula")),
            &Self::PlanetaryNebula => Some(String::from("Planetary nebula")),
            &Self::OpenCluster => Some(String::from("Open cluster")),
            &Self::GlobularCluster => Some(String::from("Globular cluster")),
            &Self::Galaxy => Some(String::from("Galaxy")),
            &Self::SupernovaRemnant => Some(String::from("Supernova remnant")),
            &Self::StarCloud => Some(String::from("Star cloud")),
            &Self::Unknown | &Self::Any => None,
        }
    }

    pub fn display_name(&self) -> String {
        self.to_option_string().unwrap_or(String::from("Unknown deepsky object"))
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
    pub proper_names_raw: Vec<String>,
    pub proper_names_full: Vec<String>,
    pub proper_names_all: Vec<String>,
    pub bayer_designation_raw: Option<String>,
    pub bayer_designation_full: Option<String>,
    pub flamsteed_designation_raw: Option<String>,
    pub flamsteed_designation_full: Option<String>,
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
    pub images: Vec<crate::structs::image_info::ImageInfo>,
}

impl QuestionObject {
    pub fn from_raw(raw: QuestionObjectRaw, images: Vec<crate::structs::image_info::ImageInfo>) -> Self {
        let proper_names_raw: Vec<String> = raw.proper_names.split(';').map(|s| s.to_owned()).filter(|s| !s.is_empty()).collect();
        let mut proper_names_full = Vec::new();
        let mut proper_names_all = Vec::new();
        for name in proper_names_raw.iter() {
            let names = crate::rendering::caspr::generate_name_combinations(name, crate::rendering::caspr::SpecificName::None);
            if let Some(full) = names.last() {
                proper_names_full.push(full.clone());
            }
            proper_names_all.extend(names);
        }

        let bayer_designation_raw = raw.bayer_designation;
        let bayer_designation_full = if let Some(bayer_raw) = &bayer_designation_raw {
            crate::rendering::caspr::generate_name_combinations(bayer_raw, crate::rendering::caspr::SpecificName::AllOptional)
                .last()
                .map(|full| full.to_owned())
        } else {
            None
        };

        let flamsteed_designation_raw = raw.flamsteed_designation;
        let flamsteed_designation_full = if let Some(flamsteed_raw) = &flamsteed_designation_raw {
            crate::rendering::caspr::generate_name_combinations(flamsteed_raw, crate::rendering::caspr::SpecificName::AllOptional)
                .last()
                .map(|full| full.to_owned())
        } else {
            None
        };
        Self {
            object_id: raw.object_id,
            object_type: raw.object_type,
            dec: raw.dec,
            ra: raw.ra,
            proper_names_raw,
            proper_names_full,
            proper_names_all,
            bayer_designation_raw,
            bayer_designation_full,
            flamsteed_designation_raw,
            flamsteed_designation_full,
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
            images,
        }
    }
}

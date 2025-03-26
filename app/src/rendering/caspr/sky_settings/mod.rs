use crate::rendering::caspr::stars;
use std::collections::HashMap;

const SEPARATOR: &str = "|";

pub struct SkySettings {
    pub stars_categories_active: HashMap<String, bool>,
    pub lines_categories_active: HashMap<String, bool>,
    pub deepskies_categories_active: HashMap<String, bool>,
    pub markers_categories_active: HashMap<String, bool>,
    pub star_names_categories_active: HashMap<String, bool>,
    pub mag_to_radius_id: usize,
    pub mag_to_radius_settings: [stars::MagnitudeToRadius; 2],
    pub deepsky_render_mag_decrease: f32,
    pub render_labels:bool,
}

impl SkySettings {
    pub fn from_raw(sky_settings: &SkySettingsRaw) -> Self {
        Self {
            stars_categories_active: string_to_partial_hash_map(&sky_settings.star_files_to_not_render),
            lines_categories_active: string_to_partial_hash_map(&sky_settings.line_files_to_not_render),
            deepskies_categories_active: string_to_partial_hash_map(&sky_settings.deepsky_files_to_not_render),
            markers_categories_active: string_to_partial_hash_map(&sky_settings.markers_files_to_not_render),
            star_names_categories_active: string_to_partial_hash_map(&sky_settings.star_names_files_to_not_use),
            mag_to_radius_id: sky_settings.mag_to_radius_id.min(crate::rendering::caspr::stars::MAGNITUDE_TO_RADIUS_OPTIONS - 1),
            mag_to_radius_settings: sky_settings.mag_to_radius_settings,
            deepsky_render_mag_decrease: sky_settings.deepsky_render_mag_decrease,
            render_labels:false,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct SkySettingsRaw {
    pub star_files_to_not_render: String,
    pub line_files_to_not_render: String,
    pub deepsky_files_to_not_render: String,
    pub markers_files_to_not_render: String,
    pub star_names_files_to_not_use: String,
    pub mag_to_radius_id: usize,
    pub mag_to_radius_settings: [stars::MagnitudeToRadius; 2],
    pub deepsky_render_mag_decrease: f32,
    pub render_labels:bool,
}

impl Default for SkySettingsRaw {
    fn default() -> Self {
        Self {
            star_files_to_not_render: String::new(),
            line_files_to_not_render: String::new(),
            deepsky_files_to_not_render: String::new(),
            markers_files_to_not_render: String::new(),
            star_names_files_to_not_use: String::new(),
            mag_to_radius_id: 1.min(crate::rendering::caspr::stars::MAGNITUDE_TO_RADIUS_OPTIONS - 1),
            mag_to_radius_settings: stars::MagnitudeToRadius::defaults(),
            deepsky_render_mag_decrease: 0.0,
            render_labels:false,
        }
    }
}

impl SkySettingsRaw {
    pub fn from_sky_settings(sky_settings: &SkySettings) -> Self {
        Self {
            star_files_to_not_render: hash_map_to_string(&sky_settings.stars_categories_active),
            line_files_to_not_render: hash_map_to_string(&sky_settings.lines_categories_active),
            deepsky_files_to_not_render: hash_map_to_string(&sky_settings.deepskies_categories_active),
            markers_files_to_not_render: hash_map_to_string(&sky_settings.markers_categories_active),
            star_names_files_to_not_use: hash_map_to_string(&sky_settings.star_names_categories_active),
            mag_to_radius_id: sky_settings.mag_to_radius_id,
            mag_to_radius_settings: sky_settings.mag_to_radius_settings,
            deepsky_render_mag_decrease: sky_settings.deepsky_render_mag_decrease,
            render_labels:sky_settings.render_labels,
        }
    }
}

fn hash_map_to_string(hash_map: &HashMap<String, bool>) -> String {
    hash_map
        .iter()
        .filter(|&(_file, active)| !*active)
        .map(|(file, _active)| file.clone())
        .collect::<Vec<String>>()
        .join(SEPARATOR)
}

fn string_to_partial_hash_map(string: &str) -> HashMap<String, bool> {
    let mut hash_map = HashMap::new();
    if !string.is_empty() {
        let spl = string.split(SEPARATOR);
        for s in spl {
            hash_map.insert(s.to_owned(), false);
        }
    }
    hash_map
}

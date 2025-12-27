use std::collections::HashMap;
use std::error::Error;

use angle::Angle;
use eframe::egui;

use crate::enums::LightPollution;
use crate::rendering::caspr;
use crate::{config, files_handling, game};

pub mod constellation;
pub mod deepsky;
pub mod lines;
pub mod markers;
pub mod star;
pub mod star_names;

pub struct Sky {
    pub stars: HashMap<String, Vec<star::Star>>,
    pub lines: HashMap<String, lines::SkyLines>,
    pub deepskies: HashMap<String, deepsky::Deepskies>,
    pub markers: HashMap<String, markers::Markers>,
    pub game_markers: markers::game_markers::GameMarkers,
    pub star_names: HashMap<String, Vec<star_names::StarName>>,
    pub constellations: HashMap<String, constellation::Constellation>,
    pub light_pollution_place: LightPollution,
    pub light_pollution_place_to_mag: HashMap<LightPollution, [Option<star::MagnitudeToRadius>; star::MAGNITUDE_TO_RADIUS_OPTIONS]>,
}

impl Sky {
    pub fn load(theme: &mut crate::rendering::themes::Theme, sky_rendering_settings: &mut caspr::sky_settings::SkySettings) -> Result<(Self, Vec<crate::game::QuestionObject>), Box<dyn Error>> {
        let list_file_path = format!("{}/./list.csv", crate::config::OBJECT_IMAGES_ADDON_FOLDER);
        let object_images = match crate::files_handling::read_file_relative(&list_file_path) {
            Ok(list_file) => {
                let mut objects_images = std::collections::HashMap::new();
                let mut reader = csv::ReaderBuilder::new().delimiter(b',').from_reader(list_file.get_contents());
                for object_image_data in reader.deserialize() {
                    let mut object_image_data: crate::structs::image_info::DeepskyObjectImageInfo = match object_image_data {
                        Ok(oid) => oid,
                        Err(err) => {
                            log::error!("Could not read object image data: {err}");
                            continue;
                        }
                    };
                    let image_path = format!("{}/./images/{}", crate::config::OBJECT_IMAGES_ADDON_FOLDER, object_image_data.image);
                    match crate::files_handling::read_file_relative(&image_path) {
                        Ok(data) => {
                            let url_result = {
                                #[cfg(not(target_arch = "wasm32"))]
                                {
                                    url::Url::from_file_path(data.get_path())
                                }

                                #[cfg(target_arch = "wasm32")]
                                {
                                    match data.get_path().to_str() {
                                        Some(p) => url::Url::parse(p).or_else(|_| Err(())),
                                        None => Err(()),
                                    }
                                }
                            };
                            if let Ok(path) = url_result {
                                object_image_data.image = path.as_str().to_owned();
                                let entry = objects_images.entry(object_image_data.object_id).or_insert(Vec::new());
                                entry.push(object_image_data);
                            }
                        }
                        Err(err) => {
                            log::error!("Could not read image at {image_path}: {err}")
                        }
                    }
                }
                Some(objects_images)
            }
            Err(err) => {
                log::error!("Could not find the image list file at {list_file_path}: {err}");
                None
            }
        };

        let content_folder = [
            ["sky objects", config::SKY_OBJECTS_FOLDER],
            ["lines", config::LINES_FOLDER],
            ["markers", config::MARKERS_FOLDER],
            ["star names", config::STAR_NAMES_FOLDER],
        ];

        let sky_data_lists = {
            let mut sky_data = Vec::new();

            for (i, d) in content_folder.iter().enumerate() {
                let id = d[0];
                let folder = d[1];
                sky_data.push((id, Vec::new()));
                match files_handling::read_dir_relative(folder) {
                    Ok(files) => {
                        let mut files_formatted = files.iter().filter_map(|f| f.get_name().as_ref().map(|file_name| (file_name.clone(), f.to_owned()))).collect();
                        sky_data[i].1.append(&mut files_formatted);
                    }
                    Err(err) => log::error!("Failed to read directory {folder:?}: {err}"),
                }
            }
            sky_data
        };

        let sky_data_files = {
            let mut other_sky_data = Vec::new();
            let file_path = config::CONSTELLATION_NAMES;
            match crate::files_handling::read_file_relative(file_path) {
                Ok(file_info) => other_sky_data.push((String::from("constellation names"), file_info)),
                Err(err) => log::error!("Failed to read file {file_path:?}: {err}"),
            }
            other_sky_data
        };

        let star_color = egui::epaint::Color32::WHITE;
        let mut catalog: HashMap<String, Vec<star::Star>> = HashMap::new();

        let mut lines: HashMap<String, lines::SkyLines> = HashMap::new();

        let mut deepskies: HashMap<String, deepsky::Deepskies> = HashMap::new();
        let objects_images = object_images.unwrap_or(std::collections::HashMap::new());

        let mut star_names: HashMap<String, Vec<star_names::StarName>> = HashMap::new();

        let mut markers: HashMap<String, markers::Markers> = HashMap::new();

        let mut question_objects = Vec::new();

        for (id, data) in sky_data_lists {
            if id == "lines" {
                for (file_name, file_info) in data {
                    let mut reader = csv::ReaderBuilder::new().delimiter(b',').from_reader(file_info.get_contents());
                    let mut line_colour = None;
                    let mut lines_vec = Vec::new();
                    for line_raw in reader.deserialize() {
                        let line_raw: lines::SkyLineRaw = line_raw?;
                        let (line, colour) = lines::SkyLine::from_raw(line_raw);
                        if line_colour.is_none() {
                            line_colour = colour;
                        }
                        lines_vec.push(line);
                    }
                    // Try to get the colour from the theme, then if the theme does not handle these lines, try to use the colour found in the lines declaration file. Only if that does not exist, use the default colour.
                    let line_colour = theme
                        .game_visuals
                        .lines_colours
                        .get(&file_name)
                        .cloned()
                        .unwrap_or(line_colour.unwrap_or(theme.game_visuals.default_colour));
                    lines.insert(
                        file_name.clone(),
                        lines::SkyLines {
                            colour: line_colour,
                            active: *sky_rendering_settings.lines_categories_active.get(&file_name).unwrap_or(&true),
                            lines: lines_vec,
                        },
                    );
                    if !sky_rendering_settings.lines_categories_active.contains_key(&file_name) {
                        sky_rendering_settings.lines_categories_active.insert(file_name.clone(), true);
                    }
                    if !theme.game_visuals.lines_colours.contains_key(&file_name) {
                        theme.game_visuals.lines_colours.insert(file_name.clone(), line_colour);
                    }
                }
            } else if id == "sky objects" {
                let override_star_colour = if theme.game_visuals.use_overriden_star_colour {
                    Some(theme.game_visuals.override_star_colour)
                } else {
                    None
                };
                for (file_name, file_info) in data {
                    let mut reader = csv::ReaderBuilder::new().delimiter(b',').from_reader(file_info.get_contents());
                    let mut deepskies_colour = None;
                    let mut deepskies_vec = Vec::new();
                    for object_raw in reader.deserialize() {
                        if let Err(err) = object_raw {
                            log::error!("Error deserializing a sky object from file {file_name}: {err}");
                            continue;
                        }
                        let object_raw: game::QuestionObjectRaw = object_raw?;
                        let names = object_raw.proper_names.clone();
                        let constellations = object_raw.constellations_abbreviations.clone();
                        let object_id = object_raw.object_id;
                        let object = game::QuestionObject::from_raw(
                            object_raw,
                            objects_images
                                .get(&object_id)
                                .cloned()
                                .unwrap_or_default()
                                .iter()
                                .map(|image_data| crate::structs::image_info::ImageInfo {
                                    path: image_data.image.clone(),
                                    source: image_data.image_source.clone(),
                                })
                                .collect(),
                        );
                        match &object.object_type {
                            crate::game::ObjectType::Star(_) => {
                                if object.mag.is_none() {
                                    log::error!("No magnitude found for star with object id {}", object.object_id);
                                    continue;
                                }
                                let star_raw = star::StarRaw {
                                    object_id: object.object_id,
                                    ra: object.ra,
                                    dec: object.dec,
                                    vmag: object.mag.unwrap(),
                                    colour: object.colour.clone(),
                                    name: if names.is_empty() { None } else { Some(names) },
                                    bv: object.bv,
                                    constellations,
                                };
                                let star = star::Star::from_raw(star_raw, star_color, override_star_colour);
                                let entry = catalog.entry(file_name.clone()).or_default();
                                entry.push(star);
                                if !sky_rendering_settings.stars_categories_active.contains_key(&file_name) {
                                    sky_rendering_settings.stars_categories_active.insert(file_name.clone(), true);
                                }
                            }
                            crate::game::ObjectType::Deepsky(inner) => {
                                let deepsky_raw = deepsky::DeepskyRaw {
                                    object_id: object.object_id,
                                    names: Some(names),
                                    messier: object.messier_number,
                                    caldwell: object.caldwell_number,
                                    ngc: object.ngc_number,
                                    ic: object.ic_number,
                                    object_type: inner.to_option_string(),
                                    constellation: constellations,
                                    ra: object.ra,
                                    dec: object.dec,
                                    mag: object.mag.map_or(String::new(), |v| v.to_string()),
                                    distance: object.distance.unwrap_or(-1.0),
                                    colour: object.colour.clone(),
                                };
                                let (deepsky, colour) = deepsky::Deepsky::from_raw(deepsky_raw, object.images.clone());
                                if deepskies_colour.is_none() {
                                    deepskies_colour = colour;
                                }
                                deepskies_vec.push(deepsky);
                            }
                        }
                        question_objects.push(object);
                    }
                    // Try to get the colour from the theme, then if the theme does not handle these lines, try to use the colour found in the lines declaration file. Only if that does not exist, use the default colour.
                    let deepskies_colour = theme
                        .game_visuals
                        .deepskies_colours
                        .get(&file_name)
                        .cloned()
                        .unwrap_or(deepskies_colour.unwrap_or(theme.game_visuals.default_colour));
                    deepskies.insert(
                        file_name.clone(),
                        deepsky::Deepskies {
                            colour: deepskies_colour,
                            active: *sky_rendering_settings.deepskies_categories_active.get(&file_name).unwrap_or(&true),
                            deepskies: deepskies_vec,
                        },
                    );
                    if !sky_rendering_settings.deepskies_categories_active.contains_key(&file_name) {
                        sky_rendering_settings.deepskies_categories_active.insert(file_name.clone(), true);
                    }
                    if !theme.game_visuals.deepskies_colours.contains_key(&file_name) {
                        theme.game_visuals.deepskies_colours.insert(file_name.clone(), deepskies_colour);
                    }
                }
            } else if id == "star names" {
                //TODO: Add linking between stars and their names
                for (file_name, file_contents) in data {
                    let mut reader = csv::ReaderBuilder::new().delimiter(b',').from_reader(file_contents.get_contents());
                    for star_name_raw in reader.deserialize() {
                        let star_name_raw: star_names::StarNameRaw = star_name_raw?;
                        let star_name = star_names::StarName::from_raw(star_name_raw);
                        match star_name {
                            Some(star_name) => {
                                let entry = star_names.entry(file_name.clone()).or_default();
                                entry.push(star_name);
                                if !sky_rendering_settings.star_names_categories_active.contains_key(&file_name) {
                                    sky_rendering_settings.star_names_categories_active.insert(file_name.clone(), true);
                                }
                            }
                            None => continue,
                        }
                    }
                }
            } else if id == "markers" {
                for (file_name, file_contents) in data {
                    let mut reader = csv::ReaderBuilder::new().delimiter(b',').from_reader(file_contents.get_contents());
                    let mut markers_colour = None;
                    let mut markers_vec = Vec::new();
                    for marker_raw in reader.deserialize() {
                        let marker_raw: markers::MarkerRaw = marker_raw?;
                        let (marker, colour) = markers::Marker::from_raw(marker_raw);
                        if markers_colour.is_none() {
                            markers_colour = colour;
                        }
                        markers_vec.push(marker);
                    }
                    // Try to get the colour from the theme, then if the theme does not handle these markers, try to use the colour found in the markers declaration file. Only if that does not exist, use the default colour.
                    let marker_colour = theme
                        .game_visuals
                        .markers_colours
                        .get(&file_name)
                        .cloned()
                        .unwrap_or(markers_colour.unwrap_or(theme.game_visuals.default_colour));
                    markers.insert(
                        file_name.clone(),
                        markers::Markers {
                            colour: marker_colour,
                            active: *sky_rendering_settings.markers_categories_active.get(&file_name).unwrap_or(&true),
                            markers: markers_vec,
                        },
                    );
                    if !sky_rendering_settings.markers_categories_active.contains_key(&file_name) {
                        sky_rendering_settings.markers_categories_active.insert(file_name.clone(), true);
                    }
                    if !theme.game_visuals.markers_colours.contains_key(&file_name) {
                        theme.game_visuals.markers_colours.insert(file_name.clone(), marker_colour);
                    }
                }
            }
        }

        let mut constellations = HashMap::new();
        for (id, file_contents) in sky_data_files {
            let mut reader = csv::ReaderBuilder::new().delimiter(b',').from_reader(file_contents.get_contents());
            if id == "constellation names" {
                for constellation_raw in reader.deserialize() {
                    let constellation_raw: constellation::ConstellationRaw = constellation_raw?;
                    let (constellation, abbreviation) = constellation::Constellation::from_raw(constellation_raw)?;
                    constellations.insert(abbreviation.to_lowercase(), constellation);
                }
            }
        }

        let mut light_pollution_place_to_mag: HashMap<LightPollution, [Option<star::MagnitudeToRadius>; star::MAGNITUDE_TO_RADIUS_OPTIONS]> =
            HashMap::with_capacity(caspr::renderer::MAG_TO_LIGHT_POLLUTION_RAW.len());
        for &(place, settings) in &caspr::renderer::MAG_TO_LIGHT_POLLUTION_RAW {
            light_pollution_place_to_mag.insert(place, settings);
        }

        let light_pollution_place = Self::mag_settings_to_light_pollution_place(sky_rendering_settings.mag_to_radius_settings[sky_rendering_settings.mag_to_radius_id], &light_pollution_place_to_mag);

        Ok((
            Self {
                stars: catalog,
                lines,
                deepskies,
                markers,
                game_markers: markers::game_markers::GameMarkers { active: true, markers: Vec::new() },
                star_names,
                constellations,
                light_pollution_place,
                light_pollution_place_to_mag,
            },
            question_objects,
        ))
    }

    pub fn mag_settings_to_light_pollution_place(
        radius_settings: star::MagnitudeToRadius,
        light_pollution_place_to_mag: &HashMap<LightPollution, [Option<star::MagnitudeToRadius>; star::MAGNITUDE_TO_RADIUS_OPTIONS]>,
    ) -> LightPollution {
        for (&place, &settings) in light_pollution_place_to_mag {
            for setting in settings.into_iter().flatten() {
                if setting == radius_settings {
                    return place;
                }
            }
        }
        LightPollution::NoSpecific
    }

    pub fn light_pollution_place_to_mag_settings(&self, place: &LightPollution, sky_rendering_settings: &caspr::sky_settings::SkySettings) -> star::MagnitudeToRadius {
        if let Some(settings) = self.light_pollution_place_to_mag.get(place) {
            if let Some(setting) = settings[sky_rendering_settings.mag_to_radius_id] {
                return setting;
            }
        }
        sky_rendering_settings.mag_to_radius_settings[sky_rendering_settings.mag_to_radius_id]
    }

    /// (ra, dec), both in radians
    pub fn determine_constellation(&self, point: (angle::Rad<f32>, angle::Rad<f32>)) -> Vec<String> {
        let mut in_constellations = Vec::new();
        'constellations: for constellation in &self.constellations {
            let (abbreviation, constellation) = constellation;
            for polygon in &constellation.polygons {
                if let Ok(true) = polygon.contains_point(&spherical_geometry::SphericalPoint::new(point.0.value(), point.1.value())) {
                    in_constellations.push(abbreviation.clone());
                    continue 'constellations;
                }
            }
        }
        in_constellations
    }
}

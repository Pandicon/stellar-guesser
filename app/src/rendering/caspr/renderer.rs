use crate::{
    enums::{LightPollution, RendererCategory, StorageKeys},
    game::{QuestionObject, QuestionObjectRaw},
    rendering::{caspr, themes::Theme},
    sky,
};
use angle::Angle;
use eframe::egui::{self, Align2, FontFamily, FontId};
use egui::epaint::Color32;
use nalgebra::{Rotation3, Vector3};
use sg_geometry::{intersections, LineSegment, Rectangle};
use std::{collections::HashMap, error::Error, f32::consts::PI, fs};

const SKY_OBJECTS_FOLDER: &str = "./data/sphere/sky-objects";
const LINES_FOLDER: &str = "./data/sphere/lines";
const MARKERS_FOLDER: &str = "./data/sphere/markers";
const STAR_NAMES_FOLDER: &str = "./data/sphere/named-stars";
const CONSTELLATION_NAMES: &str = "./data/constellations.csv";
const ZOOM_CAP: f32 = 100.0;

pub const MAG_TO_LIGHT_POLLUTION_RAW: [(LightPollution, [Option<sky::star::MagnitudeToRadius>; sky::star::MAGNITUDE_TO_RADIUS_OPTIONS]); 4] = [
    (
        LightPollution::Default,
        [Some(sky::star::MagnitudeToRadius::defaults()[0]), Some(sky::star::MagnitudeToRadius::defaults()[1])],
    ),
    (
        LightPollution::PragueDark,
        [
            Some(sky::star::MagnitudeToRadius::Linear { mag_scale: 1.0, mag_offset: 4.3 }),
            Some(sky::star::MagnitudeToRadius::Exponential { r_0: 2.3, n: 3.5, o: 0.21 }),
        ],
    ),
    (
        LightPollution::Prague,
        [
            Some(sky::star::MagnitudeToRadius::Linear { mag_scale: 0.75, mag_offset: 3.75 }),
            Some(sky::star::MagnitudeToRadius::Exponential { r_0: 1.4, n: 3.5, o: 0.21 }),
        ],
    ),
    (
        LightPollution::AverageVillage,
        [
            Some(sky::star::MagnitudeToRadius::Linear { mag_scale: 0.7, mag_offset: 5.7 }),
            Some(sky::star::MagnitudeToRadius::Exponential { r_0: 2.6, n: 3.0, o: 0.17 }),
        ],
    ),
];

// use geometry::{cast_onto_sphere, project_point};

use super::deepsky;
use super::sky_settings;
use super::stars::StarRenderer;

pub struct CellestialSphere {
    pub sky_settings: sky_settings::SkySettings,

    pub stars: HashMap<String, Vec<sky::star::Star>>,
    pub lines: HashMap<String, sky::lines::SkyLines>,
    pub deepskies: HashMap<String, sky::deepsky::Deepskies>,
    pub markers: HashMap<String, sky::markers::Markers>,
    pub game_markers: sky::markers::game_markers::GameMarkers,
    pub star_names: HashMap<String, Vec<sky::star_names::StarName>>,
    pub constellations: HashMap<String, sky::constellation::Constellation>,
    pub zoom: f32,
    pub fov: f32,
    pub camera_z: f32,
    star_renderers: HashMap<String, Vec<StarRenderer>>,
    line_renderers: HashMap<String, Vec<caspr::lines::LineRenderer>>,
    deepsky_renderers: HashMap<String, Vec<deepsky::DeepskyRenderer>>,
    marker_renderers: HashMap<String, Vec<caspr::markers::MarkerRenderer>>,

    pub light_pollution_place: LightPollution,
    pub light_pollution_place_to_mag: HashMap<LightPollution, [Option<sky::star::MagnitudeToRadius>; sky::star::MAGNITUDE_TO_RADIUS_OPTIONS]>,

    pub viewport_rect: egui::Rect,

    pub rotation: Rotation3<f32>,
}

impl CellestialSphere {
    //Renders a circle based on its current normal (does NOT account for the rotation of the sphere)
    pub fn render_circle(&self, normal: &Vector3<f32>, radius: f32, color: egui::epaint::Color32, painter: &egui::Painter) {
        let (projected_point, is_within_bounds) = sg_geometry::project_point(normal, self.zoom, self.viewport_rect);

        if is_within_bounds {
            painter.circle_filled(projected_point, radius, color);
        }
    }

    pub fn render_line(&self, start: &Vector3<f32>, end: &Vector3<f32>, colour: Color32, width: f32, painter: &egui::Painter) {
        let (start_point, is_start_within_bounds) = sg_geometry::project_point(start, self.zoom, self.viewport_rect);
        let (end_point, is_end_within_bounds) = sg_geometry::project_point(end, self.zoom, self.viewport_rect);

        let screen_rect = Rectangle::from(self.viewport_rect);

        // Allow the whole half sphere or what is within the FOV (whichever is greater)
        // This gets rid of lines on the other half of the sphere while also not removing lines that should be visible at large zooms
        let modified_camera_z = self.camera_z.max(0.0);

        // Neither the starting point nor the ending point is visible AND either of them is behind the camera
        // This avoids lines from the part of the sky that is behind us (north pole when looking at the south pole) being drawn over the screen
        if !(is_start_within_bounds || is_end_within_bounds) && (modified_camera_z < start.z || modified_camera_z < end.z) {
            return;
        }
        // Neither the starting point nor the ending point is behind the camera OR either of them is on the screen (out of the FOV cone, but within the screen rectangle) -> the line should be drawn
        // TODO: Fix it when the line crosses a corner of the screen - both of the end points go out of the screen and behind the camera while a part of the line should still be visible
        if is_start_within_bounds || is_end_within_bounds || intersections::rect_segment(screen_rect, LineSegment::new(start_point, end_point)) {
            painter.line_segment([start_point, end_point], egui::Stroke::new(width, colour));
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render_marker(
        &self,
        centre_vector: &Vector3<f32>,
        other_vector: &Option<Vector3<f32>>,
        circle: bool,
        pixel_size: Option<f32>,
        colour: Color32,
        width: f32,
        painter: &egui::Painter,
        label: Option<String>,
    ) {
        let (centre_point, is_centre_within_bounds) = sg_geometry::project_point(centre_vector, self.zoom, self.viewport_rect);
        if !is_centre_within_bounds {
            return;
        }
        let size = if let Some(other_point_vec) = other_vector {
            let (other_point, _) = sg_geometry::project_point(other_point_vec, self.zoom, self.viewport_rect);
            let vec_to = other_point - centre_point;
            vec_to.length()
        } else if let Some(pixel_size) = pixel_size {
            pixel_size
        } else {
            return;
        };
        if circle {
            painter.circle(centre_point, size, Color32::TRANSPARENT, egui::Stroke::new(width, colour));
        } else {
            painter.line_segment(
                [egui::pos2(centre_point.x, centre_point.y - size), egui::pos2(centre_point.x, centre_point.y + size)],
                egui::Stroke::new(width, colour),
            );
            painter.line_segment(
                [egui::pos2(centre_point.x - size, centre_point.y), egui::pos2(centre_point.x + size, centre_point.y)],
                egui::Stroke::new(width, colour),
            );
        }
        if self.sky_settings.render_labels {
            if let Some(text) = label {
                _ = painter.text(
                    egui::pos2(centre_point.x + size + 0.1, centre_point.y + size + 0.1),
                    Align2::LEFT_TOP,
                    text,
                    FontId::new(10.0, FontFamily::Monospace),
                    colour,
                );
            }
        }
    }

    // Renders the entire sphere view
    pub fn render_sky(&self, painter: &egui::Painter) {
        for line_renderers in self.line_renderers.values() {
            for line_renderer in line_renderers {
                line_renderer.render(self, painter);
            }
        }
        for star_renderers in self.star_renderers.values() {
            for star_renderer in star_renderers {
                star_renderer.render(painter);
            }
        }
        for deepsky_renderers in self.deepsky_renderers.values() {
            for deepsky_renderer in deepsky_renderers {
                deepsky_renderer.render(self, painter);
            }
        }
        // Make sure the game markers are rendered last, so they are not obstructed
        let mut keys: Vec<&String> = self.marker_renderers.keys().collect();
        keys.sort_by(|a, b| match (a.as_str() == "game", b.as_str() == "game") {
            (true, true) => std::cmp::Ordering::Equal,
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            (false, false) => a.cmp(b),
        });
        for key in keys {
            if let Some(marker_renderers) = self.marker_renderers.get(key) {
                for marker_renderer in marker_renderers {
                    marker_renderer.render(self, painter);
                }
            }
        }
    }

    pub fn load(storage: Option<&dyn eframe::Storage>, theme: &mut Theme) -> Result<(Self, Vec<QuestionObject>), Box<dyn Error>> {
        let object_images = match crate::files_handling::get_path_relative(crate::config::OBJECT_IMAGES_ADDON_FOLDER) {
            Ok(images_addon_dir) => {
                match images_addon_dir.try_exists() {
                    Ok(false) | Err(_) => {
                        log::warn!("The images add-on folder ({:?}) was not found", images_addon_dir);
                        None
                    }
                    Ok(true) => {
                        // The images add-on folder does exist
                        let mut list_dir = images_addon_dir.clone();
                        list_dir.push("list.csv");
                        if let Ok(list_file_content) = fs::read_to_string(list_dir) {
                            let mut objects_images = std::collections::HashMap::new();
                            #[allow(clippy::single_char_pattern)] // No idea why, but `"\""` works while `'"'` does not
                            let list_file_contents = list_file_content.replace("\"", "\\\"");
                            let mut reader = csv::ReaderBuilder::new().delimiter(b',').from_reader(list_file_contents.as_bytes());
                            for object_image_data in reader.deserialize() {
                                let mut object_image_data: crate::structs::image_info::DeepskyObjectImageInfo = object_image_data?;
                                let path_raw = &object_image_data.image;
                                let mut path = images_addon_dir.clone();
                                path.push("images");
                                for part in path_raw.split('/') {
                                    if part == "." {
                                        continue;
                                    }
                                    path.push(part);
                                }
                                match path.try_exists() {
                                    Ok(true) => {
                                        if let Ok(path) = url::Url::from_file_path(path) {
                                            object_image_data.image = path.as_str().to_owned();
                                        }
                                    }
                                    Ok(false) | Err(_) => {
                                        log::warn!("Couldn't find image {} (path checked: {:?})", path_raw, path);
                                    }
                                }
                                let entry = objects_images.entry(object_image_data.object_id).or_insert(Vec::new());
                                entry.push(object_image_data);
                            }
                            Some(objects_images)
                        } else {
                            None
                        }
                    }
                }
            }
            Err(err) => {
                log::error!("Could not locate the images folder: {err:?}");
                None
            }
        };

        let content_folder = [
            ["sky objects", SKY_OBJECTS_FOLDER],
            ["lines", LINES_FOLDER],
            ["markers", MARKERS_FOLDER],
            ["star names", STAR_NAMES_FOLDER],
        ];

        let sky_data_lists = {
            let mut sky_data = Vec::new();

            for (i, d) in content_folder.iter().enumerate() {
                let id = d[0];
                let folder = d[1];
                sky_data.push((id, Vec::new()));
                match crate::files_handling::read_dir_relative(folder) {
                    Ok(files) => {
                        let mut files_formatted = files
                            .iter()
                            .filter_map(|f| match f.get_name().as_ref() {
                                Some(file_name) => Some((file_name.clone(), f.to_owned())),
                                None => None,
                            })
                            .collect();
                        sky_data[i].1.append(&mut files_formatted);
                    }
                    Err(err) => log::error!("Failed to read directory {folder:?}: {err}"),
                }
            }
            sky_data
        };

        let sky_data_files = {
            let mut other_sky_data = Vec::new();
            let file_path = CONSTELLATION_NAMES;
            match crate::files_handling::read_file_relative(file_path) {
                Ok(file_info) => other_sky_data.push((String::from("constellation names"), file_info)),
                Err(err) => log::error!("Failed to read file {file_path:?}: {err}"),
            }
            other_sky_data
        };

        let mut sky_settings = sky_settings::SkySettings::from_raw(&sky_settings::SkySettingsRaw::default());
        if let Some(storage) = storage {
            if let Some(sky_settings_raw_str) = storage.get_string(StorageKeys::SkySettings.as_ref()) {
                match serde_json::from_str(&sky_settings_raw_str) {
                    Ok(data) => sky_settings = sky_settings::SkySettings::from_raw(&data),
                    Err(err) => log::error!("Failed to deserialize sky settings: {:?}", err),
                }
            }
        }

        let star_color = egui::epaint::Color32::WHITE;
        let mut catalog: HashMap<String, Vec<sky::star::Star>> = HashMap::new();

        let mut lines: HashMap<String, sky::lines::SkyLines> = HashMap::new();

        let mut deepskies: HashMap<String, sky::deepsky::Deepskies> = HashMap::new();
        let objects_images = object_images.unwrap_or(std::collections::HashMap::new());

        let mut star_names: HashMap<String, Vec<sky::star_names::StarName>> = HashMap::new();

        let mut markers: HashMap<String, sky::markers::Markers> = HashMap::new();

        let mut question_objects = Vec::new();

        for (id, data) in sky_data_lists {
            if id == "lines" {
                for (file_name, file_info) in data {
                    let mut reader = csv::ReaderBuilder::new().delimiter(b',').from_reader(file_info.get_contents());
                    let mut line_colour = None;
                    let mut lines_vec = Vec::new();
                    for line_raw in reader.deserialize() {
                        let line_raw: sky::lines::SkyLineRaw = line_raw?;
                        let (line, colour) = sky::lines::SkyLine::from_raw(line_raw);
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
                        sky::lines::SkyLines {
                            colour: line_colour,
                            active: *sky_settings.lines_categories_active.get(&file_name).unwrap_or(&true),
                            lines: lines_vec,
                        },
                    );
                    if !sky_settings.lines_categories_active.contains_key(&file_name) {
                        sky_settings.lines_categories_active.insert(file_name.clone(), true);
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
                        let object_raw: QuestionObjectRaw = object_raw?;
                        let names = object_raw.proper_names.clone();
                        let constellations = object_raw.constellations_abbreviations.clone();
                        let object_id = object_raw.object_id;
                        let object = QuestionObject::from_raw(
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
                                let star_raw = sky::star::StarRaw {
                                    object_id: object.object_id,
                                    ra: object.ra,
                                    dec: object.dec,
                                    vmag: object.mag.unwrap(),
                                    colour: object.colour.clone(),
                                    name: if names.is_empty() { None } else { Some(names) },
                                    bv: object.bv,
                                    constellations,
                                };
                                let star = sky::star::Star::from_raw(star_raw, star_color, override_star_colour);
                                let entry = catalog.entry(file_name.clone()).or_default();
                                entry.push(star);
                                if !sky_settings.stars_categories_active.contains_key(&file_name) {
                                    sky_settings.stars_categories_active.insert(file_name.clone(), true);
                                }
                            }
                            crate::game::ObjectType::Deepsky(inner) => {
                                let deepsky_raw = sky::deepsky::DeepskyRaw {
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
                                let (deepsky, colour) = sky::deepsky::Deepsky::from_raw(deepsky_raw, object.images.clone());
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
                        sky::deepsky::Deepskies {
                            colour: deepskies_colour,
                            active: *sky_settings.deepskies_categories_active.get(&file_name).unwrap_or(&true),
                            deepskies: deepskies_vec,
                        },
                    );
                    if !sky_settings.deepskies_categories_active.contains_key(&file_name) {
                        sky_settings.deepskies_categories_active.insert(file_name.clone(), true);
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
                        let star_name_raw: sky::star_names::StarNameRaw = star_name_raw?;
                        let star_name = sky::star_names::StarName::from_raw(star_name_raw);
                        match star_name {
                            Some(star_name) => {
                                let entry = star_names.entry(file_name.clone()).or_default();
                                entry.push(star_name);
                                if !sky_settings.star_names_categories_active.contains_key(&file_name) {
                                    sky_settings.star_names_categories_active.insert(file_name.clone(), true);
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
                        let marker_raw: sky::markers::MarkerRaw = marker_raw?;
                        let (marker, colour) = sky::markers::Marker::from_raw(marker_raw);
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
                        sky::markers::Markers {
                            colour: marker_colour,
                            active: *sky_settings.markers_categories_active.get(&file_name).unwrap_or(&true),
                            markers: markers_vec,
                        },
                    );
                    if !sky_settings.markers_categories_active.contains_key(&file_name) {
                        sky_settings.markers_categories_active.insert(file_name.clone(), true);
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
                    let constellation_raw: sky::constellation::ConstellationRaw = constellation_raw?;
                    let (constellation, abbreviation) = sky::constellation::Constellation::from_raw(constellation_raw)?;
                    constellations.insert(abbreviation.to_lowercase(), constellation);
                }
            }
        }

        let mut light_pollution_place_to_mag: HashMap<LightPollution, [Option<sky::star::MagnitudeToRadius>; sky::star::MAGNITUDE_TO_RADIUS_OPTIONS]> =
            HashMap::with_capacity(MAG_TO_LIGHT_POLLUTION_RAW.len());
        for &(place, settings) in &MAG_TO_LIGHT_POLLUTION_RAW {
            light_pollution_place_to_mag.insert(place, settings);
        }

        let light_pollution_place = CellestialSphere::mag_settings_to_light_pollution_place(sky_settings.mag_to_radius_settings[sky_settings.mag_to_radius_id], &light_pollution_place_to_mag);

        let viewport_rect = egui::Rect::from_two_pos(egui::pos2(0.0, 0.0), egui::pos2(0.0, 0.0));
        let zoom = 3.0_f32.sqrt();
        let fov = Self::zoom_to_fov(zoom);
        Ok((
            Self {
                sky_settings,
                stars: catalog,
                lines,
                deepskies,
                markers,
                game_markers: sky::markers::game_markers::GameMarkers { active: true, markers: Vec::new() },
                star_names,
                constellations,
                zoom,
                fov,
                camera_z: Self::fov_to_camera_z(fov),
                star_renderers: HashMap::new(),
                line_renderers: HashMap::new(),
                deepsky_renderers: HashMap::new(),
                marker_renderers: HashMap::new(),

                light_pollution_place,
                light_pollution_place_to_mag,

                viewport_rect,

                rotation: Rotation3::new(Vector3::new(0.0, 0.0, 0.0)),
            },
            question_objects,
        ))
    }

    // TODO: Make this always for example halve the FOV
    /// Returns if star renderers should be reinitialised
    pub fn zoom(&mut self, velocity: f32) -> bool {
        if velocity == 0.0 {
            return false;
        }
        let future_zoom = self.zoom + velocity * self.zoom;
        //A check is needed since negative zoom breaks everything
        if ZOOM_CAP > future_zoom && future_zoom > 0.0 {
            self.zoom = future_zoom;
            self.fov = Self::zoom_to_fov(self.zoom);
            self.camera_z = Self::fov_to_camera_z(self.fov);
            return true;
        }
        false
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn zoom_to_fov(zoom: f32) -> f32 {
        4.0 * (1.0 / zoom).atan() / PI * 180.0
    }

    pub fn fov_to_camera_z(fov_deg: f32) -> f32 {
        -((fov_deg / 180.0 * PI) / 2.0).cos()
    }

    pub fn init(&mut self) {
        let settings = self.light_pollution_place_to_mag_settings(&self.light_pollution_place);
        self.sky_settings.mag_to_radius_settings[self.sky_settings.mag_to_radius_id] = settings;
        if self.sky_settings.cloud_settings.enabled {
            crate::rendering::caspr::clouds::apply_dimming(&mut self.stars, &self.sky_settings.cloud_settings);
        }
        self.init_renderers();
    }

    /// Preserves disabled renderers - will reinitialise them, but will also keep them disabled
    pub fn init_renderers(&mut self) {
        {
            let mut old_renderers = HashMap::new();
            std::mem::swap(&mut self.star_renderers, &mut old_renderers);
            let mut active_star_groups = Vec::new();
            let mut all_disabled_renderers = std::collections::HashMap::new();
            for name in self.stars.keys() {
                let active = self.sky_settings.stars_categories_active.entry(name.to_owned()).or_insert(true);
                if !*active {
                    continue;
                }
                active_star_groups.push(name.to_owned());

                let mut disabled_renderers = std::collections::HashSet::new();
                if let Some(renderers) = old_renderers.get(name) {
                    for renderer in renderers {
                        if renderer.disabled {
                            disabled_renderers.insert(renderer.object_id);
                        }
                    }
                }
                if !disabled_renderers.is_empty() {
                    all_disabled_renderers.insert(name.to_owned(), disabled_renderers);
                }
            }
            for name in active_star_groups {
                self.init_single_renderer_group(RendererCategory::Stars, &name);
                if let Some(disabled_renderers) = all_disabled_renderers.get(&name) {
                    if let Some(renderers) = self.star_renderers.get_mut(&name) {
                        for renderer in renderers {
                            if disabled_renderers.contains(&renderer.object_id) {
                                renderer.disabled = true;
                            }
                        }
                    }
                }
            }
        }

        self.line_renderers = HashMap::new();
        let mut active_line_groups = Vec::new();
        for (name, lines) in &self.lines {
            if !lines.active {
                continue;
            }
            active_line_groups.push(name.to_owned());
        }
        for name in active_line_groups {
            self.init_single_renderer_group(RendererCategory::Lines, &name);
        }

        {
            let mut old_renderers = HashMap::new();
            std::mem::swap(&mut self.deepsky_renderers, &mut old_renderers);
            let mut active_deepsky_groups = Vec::new();
            let mut all_disabled_renderers = std::collections::HashMap::new();
            for name in self.stars.keys() {
                let active = self.sky_settings.deepskies_categories_active.entry(name.to_owned()).or_insert(true);
                if !*active {
                    continue;
                }
                active_deepsky_groups.push(name.to_owned());

                let mut disabled_renderers = std::collections::HashSet::new();
                if let Some(renderers) = old_renderers.get(name) {
                    for renderer in renderers {
                        if renderer.disabled {
                            disabled_renderers.insert(renderer.object_id);
                        }
                    }
                }
                if !disabled_renderers.is_empty() {
                    all_disabled_renderers.insert(name.to_owned(), disabled_renderers);
                }
            }
            for name in active_deepsky_groups {
                self.init_single_renderer_group(RendererCategory::Deepskies, &name);
                if let Some(disabled_renderers) = all_disabled_renderers.get(&name) {
                    if let Some(renderers) = self.deepsky_renderers.get_mut(&name) {
                        for renderer in renderers {
                            if disabled_renderers.contains(&renderer.object_id) {
                                renderer.disabled = true;
                            }
                        }
                    }
                }
            }
        }

        self.marker_renderers = HashMap::new();
        let mut active_markers_groups = Vec::new();
        for (name, markers) in &self.markers {
            if !markers.active {
                continue;
            }
            active_markers_groups.push(name.to_owned());
        }
        for name in active_markers_groups {
            self.init_single_renderer_group(RendererCategory::Markers, &name);
        }
        if self.game_markers.active {
            self.init_single_renderer_group(RendererCategory::Markers, "game");
        }

        if self.game_markers.active {
            self.init_single_renderer_group(RendererCategory::Markers, "game");
        }
    }

    /// Preserves disabled renderers - will reinitialise them, but will also keep them disabled
    pub fn reinit_renderer_category(&mut self, category: RendererCategory) {
        match category {
            RendererCategory::Stars => {
                let mut old_renderers = HashMap::new();
                std::mem::swap(&mut self.star_renderers, &mut old_renderers);
                let mut active_star_groups = Vec::new();
                let mut all_disabled_renderers = std::collections::HashMap::new();
                for name in self.stars.keys() {
                    let active = self.sky_settings.stars_categories_active.entry(name.to_owned()).or_insert(true);
                    if !*active {
                        continue;
                    }
                    active_star_groups.push(name.to_owned());

                    let mut disabled_renderers = std::collections::HashSet::new();
                    if let Some(renderers) = old_renderers.get(name) {
                        for renderer in renderers {
                            if renderer.disabled {
                                disabled_renderers.insert(renderer.object_id);
                            }
                        }
                    }
                    all_disabled_renderers.insert(name.to_owned(), disabled_renderers);
                }
                for name in active_star_groups {
                    self.init_single_renderer_group(RendererCategory::Stars, &name);
                    if let Some(disabled_renderers) = all_disabled_renderers.get(&name) {
                        if let Some(renderers) = self.star_renderers.get_mut(&name) {
                            for renderer in renderers {
                                if disabled_renderers.contains(&renderer.object_id) {
                                    renderer.disabled = true;
                                }
                            }
                        }
                    }
                }
            }
            RendererCategory::Lines => {
                self.line_renderers = HashMap::new();
                let mut active_line_groups = Vec::new();
                for (name, lines) in &self.lines {
                    if !lines.active {
                        continue;
                    }
                    active_line_groups.push(name.to_owned());
                }
                for name in active_line_groups {
                    self.init_single_renderer_group(RendererCategory::Lines, &name);
                }
            }
            RendererCategory::Deepskies => {
                let mut old_renderers = HashMap::new();
                std::mem::swap(&mut self.deepsky_renderers, &mut old_renderers);
                let mut active_deepsky_groups = Vec::new();
                let mut all_disabled_renderers = std::collections::HashMap::new();
                for name in self.stars.keys() {
                    let active = self.sky_settings.deepskies_categories_active.entry(name.to_owned()).or_insert(true);
                    if !*active {
                        continue;
                    }
                    active_deepsky_groups.push(name.to_owned());

                    let mut disabled_renderers = std::collections::HashSet::new();
                    if let Some(renderers) = old_renderers.get(name) {
                        for renderer in renderers {
                            if renderer.disabled {
                                disabled_renderers.insert(renderer.object_id);
                            }
                        }
                    }
                    if !disabled_renderers.is_empty() {
                        all_disabled_renderers.insert(name.to_owned(), disabled_renderers);
                    }
                }
                for name in active_deepsky_groups {
                    self.init_single_renderer_group(RendererCategory::Deepskies, &name);
                    if let Some(disabled_renderers) = all_disabled_renderers.get(&name) {
                        if let Some(renderers) = self.deepsky_renderers.get_mut(&name) {
                            for renderer in renderers {
                                if disabled_renderers.contains(&renderer.object_id) {
                                    renderer.disabled = true;
                                }
                            }
                        }
                    }
                }
            }
            RendererCategory::Markers => {
                self.marker_renderers = HashMap::new();
                let mut active_markers_groups = Vec::new();
                for (name, markers) in &self.markers {
                    if !markers.active {
                        continue;
                    }
                    active_markers_groups.push(name.to_owned());
                }
                for name in active_markers_groups {
                    self.init_single_renderer_group(RendererCategory::Markers, &name);
                }
                if self.game_markers.active {
                    self.init_single_renderer_group(RendererCategory::Markers, "game");
                }

                if self.game_markers.active {
                    self.init_single_renderer_group(RendererCategory::Markers, "game");
                }
            }
        }
    }

    pub fn init_single_renderer_group(&mut self, category: RendererCategory, name: &str) {
        match category {
            RendererCategory::Stars => {
                if let Some(stars) = self.stars.get(name) {
                    self.star_renderers.insert(
                        name.to_string(),
                        stars
                            .iter()
                            .map(|star| {
                                star.get_renderer(
                                    self.rotation.matrix(),
                                    self.sky_settings.mag_to_radius_settings[self.sky_settings.mag_to_radius_id],
                                    angle::Deg(self.fov),
                                    self.zoom,
                                    self.viewport_rect,
                                )
                            })
                            .collect(),
                    );
                }
            }
            RendererCategory::Lines => {
                if let Some(lines) = self.lines.get(name) {
                    self.line_renderers
                        .insert(name.to_string(), lines.lines.iter().map(|line| line.get_renderer(self.rotation.matrix(), lines.colour)).collect());
                }
            }
            RendererCategory::Deepskies => {
                if let Some(deepskies) = self.deepskies.get(name) {
                    let mut disabled_renderers = std::collections::HashSet::new();
                    if let Some(renderers) = self.deepsky_renderers.get(name) {
                        for renderer in renderers {
                            if renderer.disabled {
                                disabled_renderers.insert(renderer.object_id);
                            }
                        }
                    }
                    self.deepsky_renderers.insert(
                        name.to_string(),
                        deepskies
                            .deepskies
                            .iter()
                            .map(|deepsky| {
                                let mut renderer = deepsky.get_renderer(self.rotation.matrix(), deepskies.colour);
                                if disabled_renderers.contains(&deepsky.object_id) {
                                    renderer.disabled = true;
                                }
                                renderer
                            })
                            .collect(),
                    );
                }
            }
            RendererCategory::Markers => {
                if name == "game" {
                    self.marker_renderers.insert(
                        name.to_string(),
                        self.game_markers.markers.iter().filter_map(|marker| marker.get_renderer(self.rotation.matrix())).collect(),
                    );
                } else if let Some(markers) = self.markers.get(name) {
                    self.marker_renderers.insert(
                        name.to_string(),
                        markers.markers.iter().filter_map(|marker| marker.get_renderer(self.rotation.matrix(), markers.colour)).collect(),
                    );
                }
            }
        }
    }

    pub fn deinit_single_renderer_group(&mut self, category: RendererCategory, name: &str) {
        match category {
            RendererCategory::Stars => {
                self.star_renderers.insert(name.to_string(), Vec::new());
            }
            RendererCategory::Lines => {
                self.line_renderers.insert(name.to_string(), Vec::new());
            }
            RendererCategory::Deepskies => {
                self.deepsky_renderers.insert(name.to_string(), Vec::new());
            }
            RendererCategory::Markers => {
                self.marker_renderers.insert(name.to_string(), Vec::new());
            }
        }
    }

    pub fn enable_single_renderer(&mut self, object_id: u64) {
        for renderer_group in self.star_renderers.values_mut() {
            for renderer in renderer_group {
                if renderer.object_id == object_id {
                    renderer.disabled = false;
                }
            }
        }
        for renderer_group in self.deepsky_renderers.values_mut() {
            for renderer in renderer_group {
                if renderer.object_id == object_id {
                    renderer.disabled = false;
                }
            }
        }
    }

    pub fn disable_single_renderer(&mut self, object_id: u64) {
        for renderer_group in self.star_renderers.values_mut() {
            for renderer in renderer_group {
                if renderer.object_id == object_id {
                    renderer.disabled = true;
                }
            }
        }
        for renderer_group in self.deepsky_renderers.values_mut() {
            for renderer in renderer_group {
                if renderer.object_id == object_id {
                    renderer.disabled = true;
                }
            }
        }
    }

    /*pub fn mag_to_radius(&self, vmag: f32) -> f32 {
        let mag = self.sky_settings.mag_scale * (self.sky_settings.mag_offset - vmag) + 0.5;
        if mag < 0.35 {
            0.0
        } else {
            mag
        }
    }*/

    pub fn project_screen_pos(&self, screen_pos: egui::Pos2) -> Vector3<f32> {
        sg_geometry::cast_onto_sphere(&self.viewport_rect, &screen_pos, self.rotation, self.get_zoom())
    }

    pub fn mag_settings_to_light_pollution_place(
        radius_settings: sky::star::MagnitudeToRadius,
        light_pollution_place_to_mag: &HashMap<LightPollution, [Option<sky::star::MagnitudeToRadius>; sky::star::MAGNITUDE_TO_RADIUS_OPTIONS]>,
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

    pub fn light_pollution_place_to_mag_settings(&self, place: &LightPollution) -> sky::star::MagnitudeToRadius {
        if let Some(settings) = self.light_pollution_place_to_mag.get(place) {
            if let Some(setting) = settings[self.sky_settings.mag_to_radius_id] {
                return setting;
            }
        }
        self.sky_settings.mag_to_radius_settings[self.sky_settings.mag_to_radius_id]
    }
    /*pub fn to_equatorial_coordinates(vector: Vector3<f32>) -> (f32, f32) {
        cartesian_to_spherical(vector)
    }*/
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

    pub fn rotate_between_points(&mut self, initial_pos: &Vector3<f32>, final_pos: &Vector3<f32>) -> Option<()> {
        if initial_pos == final_pos {
            return None;
        }
        if let Some(rotation_matrix) = Rotation3::rotation_between(initial_pos, final_pos) {
            if rotation_matrix.matrix()[0].is_nan() {
                return None;
            }
            self.rotation *= rotation_matrix;
        } else {
            return None;
        }
        Some(())
    }

    /// Rotates the view to look at the point. It has to be taken without rotations.
    pub fn look_at_point(&mut self, point: &Vector3<f32>) -> Option<()> {
        let z_axis = Vector3::new(0.0, 0.0, -1.0);
        let y_axis = Vector3::new(0.0, 1.0, 0.0);
        let axis = if point.cross(&z_axis).magnitude_squared() < 0.05 { y_axis } else { z_axis };
        let rotation_matrix = Rotation3::look_at_rh(point, &axis);
        if rotation_matrix.matrix()[0].is_nan() {
            return None;
        }
        self.rotation = rotation_matrix;
        Some(())
    }
}

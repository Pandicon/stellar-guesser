use crate::{enums::LightPollution, structs::graphics_settings::GraphicsSettings};
use egui::epaint::Color32;
use nalgebra::{Rotation3, Vector3};
use std::{collections::HashMap, error::Error, fs};

const DEEPSKIES_FOLDER: &str = "./sphere/deepsky";
const LINES_FOLDER: &str = "./sphere/lines";
const MARKERS_FOLDER: &str = "./sphere/markers";
const STARS_FOLDER: &str = "./sphere/stars";
const STAR_NAMES_FOLDER: &str = "./sphere/named-stars";
const CONSTELLATION_VERTICES: &str = "./data/constellation_vertices.csv";
const CONSTELLATION_NAMES: &str = "./data/constellations.csv";
const ZOOM_CAP: f32 = 100.0;

const MAG_TO_LIGHT_POLLUTION_RAW: [(f32, f32, LightPollution); 3] = [(6.0, 0.3, LightPollution::Default), (3.0, 0.5, LightPollution::Prague), (4.2, 0.5, LightPollution::AverageVillage)];

use crate::geometry;
use geometry::{cartesian_to_spherical, cast_onto_sphere, is_inside_polygon, project_point};

use super::deepsky::{Deepsky, DeepskyRaw, DeepskyRenderer};
use super::lines::{LineRenderer, SkyLine, SkyLineRaw};
use super::markers::{Marker, MarkerRaw, MarkerRenderer};
use super::star_names::{StarName, StarNameRaw};
use super::stars::{Star, StarRaw, StarRenderer};

use super::constellation::{BorderVertex, Constellation, ConstellationRaw};

const MERIDIAN_CONSTELLATIONS: [&str; 10] = ["cep", "cas", "and", "peg", "pis", "cet", "scl", "phe", "tuc", "oct"];
const OBJECT_IMAGES_FOLDER: &str = crate::OBJECT_IMAGES_ADDON_FOLDER;

pub struct CellestialSphere {
	pub stars: HashMap<String, Vec<Star>>,
	pub stars_categories_active: HashMap<String, bool>,
	pub lines: HashMap<String, Vec<SkyLine>>,
	pub lines_categories_active: HashMap<String, bool>,
	pub deepskies: HashMap<String, Vec<Deepsky>>,
	pub deepskies_categories_active: HashMap<String, bool>,
	pub markers: HashMap<String, Vec<Marker>>,
	pub markers_categories_active: HashMap<String, bool>,
	pub star_names: HashMap<String, Vec<StarName>>,
	pub constellations: HashMap<String, Constellation>,
	pub star_names_categories_active: HashMap<String, bool>,
	pub zoom: f32,
	star_renderers: HashMap<String, Vec<StarRenderer>>,
	line_renderers: HashMap<String, Vec<LineRenderer>>,
	deepsky_renderers: HashMap<String, Vec<DeepskyRenderer>>,
	marker_renderers: HashMap<String, Vec<MarkerRenderer>>,

	pub mag_scale: f32,
	pub mag_offset: f32,
	pub light_pollution_place: LightPollution,
	light_pollution_place_to_mag: HashMap<LightPollution, [f32; 2]>,

	pub viewport_rect: egui::Rect,

	pub rotation: Rotation3<f32>,
	pub deepsky_render_mag_decrease: f32,
}

impl CellestialSphere {
	//Renders a circle based on its current normal (does NOT account for the rotation of the sphere)
	pub fn render_circle(&self, normal: &Vector3<f32>, radius: f32, color: egui::epaint::Color32, painter: &egui::Painter) {
		let (projected_point, is_within_bounds) = project_point(normal, self.zoom, self.viewport_rect);

		if is_within_bounds {
			painter.circle_filled(projected_point, radius, color);
		}
	}

	pub fn render_line(&self, start: &Vector3<f32>, end: &Vector3<f32>, colour: Color32, width: f32, painter: &egui::Painter) {
		let (start_point, is_start_within_bounds) = project_point(start, self.zoom, self.viewport_rect);
		let (end_point, is_end_within_bounds) = project_point(end, self.zoom, self.viewport_rect);

		if is_start_within_bounds || is_end_within_bounds {
			painter.line_segment([start_point, end_point], egui::Stroke::new(width, colour));
		}
	}

	pub fn render_marker(&self, centre_vector: &Vector3<f32>, other_vector: &Option<Vector3<f32>>, circle: bool, pixel_size: Option<f32>, colour: Color32, width: f32, painter: &egui::Painter) {
		let (centre_point, is_centre_within_bounds) = project_point(centre_vector, self.zoom, self.viewport_rect);
		if !is_centre_within_bounds {
			return;
		}
		let size = if let Some(other_point_vec) = other_vector {
			let (other_point, _) = project_point(other_point_vec, self.zoom, self.viewport_rect);
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
	}

	//Renders the entire sphere view
	pub fn render_sky(&self, painter: &egui::Painter, graphics_settings: &GraphicsSettings) {
		//some stuff lol
		for line_renderers in self.line_renderers.values() {
			for line_renderer in line_renderers {
				line_renderer.render(&self, painter);
			}
		}
		for star_renderers in self.star_renderers.values() {
			for star_renderer in star_renderers {
				star_renderer.render(self, painter, graphics_settings);
			}
		}
		for marker_renderers in self.marker_renderers.values() {
			for marker_renderer in marker_renderers {
				marker_renderer.render(self, painter);
			}
		}
		for deepsky_renderers in self.deepsky_renderers.values() {
			for deepsky_renderer in deepsky_renderers {
				deepsky_renderer.render(&self, painter);
			}
		}
	}

	pub fn load(storage: Option<&dyn eframe::Storage>) -> Result<Self, Box<dyn Error>> {
		let object_images = if let Ok(executable_dir) = std::env::current_exe() {
			let mut images_addon_dir = executable_dir;
			images_addon_dir.pop();
			for part in OBJECT_IMAGES_FOLDER.split('/') {
				if part == "." {
					continue;
				}
				images_addon_dir.push(part);
			}
			match images_addon_dir.try_exists() {
				Ok(false) | Err(_) => {
					dbg!("The images add-on folder was not found");
					None
				}
				Ok(true) => {
					// The images add-on folder does exist
					let mut list_dir = images_addon_dir.clone();
					list_dir.push("list.csv");
					let mut objects_images = Vec::new();
					let reader: Result<csv::Reader<std::fs::File>, csv::Error> = csv::Reader::from_path(list_dir);
					for object_image_data in reader?.deserialize() {
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
								if let Some(path) = path.to_str() {
									let path = path.replace("\\", "/");
									object_image_data.image = format!("file://{path}");
								}
							}
							Ok(false) | Err(_) => {
								println!("Couldn't find image {} (path checked: {:?})", path_raw, path);
							}
						}
						objects_images.push(object_image_data);
					}
					Some(objects_images)
				}
			}
		} else {
			println!("Couldn't load the executable directory and therefore couldn't load the images");
			None
		};

		let star_color = egui::epaint::Color32::WHITE;
		let mut catalog: HashMap<String, Vec<Star>> = HashMap::new();
		let mut stars_categories_active = HashMap::new();
		let files = fs::read_dir(STARS_FOLDER);
		for file in (files?).flatten() {
			let path = file.path();
			let file_name = path.file_name();
			if file_name.is_none() {
				continue;
			}
			let file_name = file_name.unwrap().to_str();
			if file_name.is_none() {
				continue;
			}
			let file_name = file_name.unwrap().to_string();
			let reader: Result<csv::Reader<std::fs::File>, csv::Error> = csv::Reader::from_path(path);

			for star_raw in reader?.deserialize() {
				let star_raw: StarRaw = star_raw?;
				let star = Star::from_raw(star_raw, star_color);
				let entry = catalog.entry(file_name.clone()).or_default();
				entry.push(star);
				if !stars_categories_active.contains_key(&file_name) {
					stars_categories_active.insert(file_name.clone(), true);
				}
			}
		}

		let mut lines: HashMap<String, Vec<SkyLine>> = HashMap::new();
		let mut lines_categories_active = HashMap::new();
		let files: Result<fs::ReadDir, std::io::Error> = fs::read_dir(LINES_FOLDER);
		for file in (files?).flatten() {
			let path = file.path();
			let file_name = path.file_name();
			if file_name.is_none() {
				continue;
			}
			let file_name = file_name.unwrap().to_str();
			if file_name.is_none() {
				continue;
			}
			let file_name = file_name.unwrap().to_string();
			let reader: Result<csv::Reader<std::fs::File>, csv::Error> = csv::Reader::from_path(file.path());

			for line_raw in reader?.deserialize() {
				let line_raw: SkyLineRaw = line_raw?;
				let line = SkyLine::from_raw(line_raw, star_color);
				let entry = lines.entry(file_name.clone()).or_default();
				entry.push(line);
				if !lines_categories_active.contains_key(&file_name) {
					lines_categories_active.insert(file_name.clone(), true);
				}
			}
		}

		let mut deepskies: HashMap<String, Vec<Deepsky>> = HashMap::new();
		let mut deepskies_categories_active = HashMap::new();
		let files: Result<fs::ReadDir, std::io::Error> = fs::read_dir(DEEPSKIES_FOLDER);
		let objects_images = object_images.unwrap_or(Vec::new());
		for file in (files?).flatten() {
			let path = file.path();
			let file_name = path.file_name();
			if file_name.is_none() {
				continue;
			}
			let file_name = file_name.unwrap().to_str();
			if file_name.is_none() {
				continue;
			}
			let file_name = file_name.unwrap().to_string();
			let reader: Result<csv::Reader<std::fs::File>, csv::Error> = csv::Reader::from_path(file.path());

			for deepsky_raw in reader?.deserialize() {
				let deepsky_raw: DeepskyRaw = deepsky_raw?;
				let deepsky_images_raw = objects_images
					.iter()
					.filter(|image_data| {
						let designation = image_data.object_designation.to_lowercase().replace(' ', "");
						let mut res = false;
						if let Some(ngc_num) = &deepsky_raw.ngc {
							if designation.starts_with("ngc") {
								let number = designation.chars().filter(|c| c.is_digit(10)).collect::<String>();
								res |= &number == ngc_num;
							}
						}
						if let Some(ic_num) = &deepsky_raw.ic {
							if designation.starts_with("ic") {
								let number = designation.chars().filter(|c| c.is_digit(10)).collect::<String>();
								res |= &number == ic_num;
							}
						}
						if let Some(c_num) = &deepsky_raw.caldwell {
							if designation.starts_with('c') {
								let number = designation.chars().filter(|c| c.is_digit(10)).collect::<String>();
								res |= &number == c_num;
							}
						}
						if let Some(m_num) = &deepsky_raw.messier {
							if designation.starts_with('m') {
								let number = designation.chars().filter(|c| c.is_digit(10)).collect::<String>();
								res |= &number == m_num;
							}
						}
						res
					})
					.map(|image_data| crate::structs::image_info::ImageInfo {
						path: image_data.image.clone(),
						source: image_data.image_source.clone(),
					})
					.collect::<Vec<crate::structs::image_info::ImageInfo>>();
				let deepsky = Deepsky::from_raw(deepsky_raw, star_color, deepsky_images_raw);
				let entry = deepskies.entry(file_name.clone()).or_default();
				entry.push(deepsky);
				if !deepskies_categories_active.contains_key(&file_name) {
					deepskies_categories_active.insert(file_name.clone(), true);
				}
			}
		}
		let mut star_names: HashMap<String, Vec<StarName>> = HashMap::new();
		let mut star_names_categories_active = HashMap::new();
		let files: Result<fs::ReadDir, std::io::Error> = fs::read_dir(STAR_NAMES_FOLDER);
		for file in (files?).flatten() {
			let path = file.path();
			let file_name = path.file_name();
			if file_name.is_none() {
				continue;
			}
			let file_name = file_name.unwrap().to_str();
			if file_name.is_none() {
				continue;
			}
			let file_name = file_name.unwrap().to_string();
			let reader: Result<csv::Reader<std::fs::File>, csv::Error> = csv::Reader::from_path(file.path());

			for star_name_raw in reader?.deserialize() {
				let star_name_raw: StarNameRaw = star_name_raw?;
				let star_name = StarName::from_raw(star_name_raw);
				match star_name {
					Some(star_name) => {
						let entry = star_names.entry(file_name.clone()).or_default();
						entry.push(star_name);
						if !star_names_categories_active.contains_key(&file_name) {
							star_names_categories_active.insert(file_name.clone(), true);
						}
					}
					None => continue,
				}
			}
		}
		//TODO:Add linking between stars and their names

		let mut markers: HashMap<String, Vec<Marker>> = HashMap::new();
		let mut markers_categories_active = HashMap::new();
		markers.insert(String::from("game"), Vec::new());
		markers_categories_active.insert(String::from("game"), true);
		let files: Result<fs::ReadDir, std::io::Error> = fs::read_dir(MARKERS_FOLDER);
		for file in (files?).flatten() {
			let path = file.path();
			let file_name = path.file_name();
			if file_name.is_none() {
				continue;
			}
			let file_name = file_name.unwrap().to_str();
			if file_name.is_none() {
				continue;
			}
			let file_name = file_name.unwrap().to_string();
			let reader: Result<csv::Reader<std::fs::File>, csv::Error> = csv::Reader::from_path(file.path());

			for marker_raw in reader?.deserialize() {
				let marker_raw: MarkerRaw = marker_raw?;
				let marker = Marker::from_raw(marker_raw, star_color);
				let entry = markers.entry(file_name.clone()).or_default();
				entry.push(marker);
				if !markers_categories_active.contains_key(&file_name) {
					markers_categories_active.insert(file_name.clone(), true);
				}
			}
		}
		let mut constellations = HashMap::new();
		let reader = csv::Reader::from_path(CONSTELLATION_NAMES);
		for constellation_raw in reader?.deserialize() {
			let constellation_raw: ConstellationRaw = constellation_raw?;
			let (constellation, abbreviation) = Constellation::from_raw(constellation_raw);
			constellations.insert(abbreviation.to_lowercase(), constellation);
		}
		let reader = csv::Reader::from_path(CONSTELLATION_VERTICES);
		for constellation_vertex in reader?.deserialize() {
			let constellation_vertex: BorderVertex = constellation_vertex?;
			match constellations.get_mut(&constellation_vertex.constellation.to_lowercase()) {
				Some(constellation) => {
					let position = constellation_vertex.get_position();
					constellation.vertices.push(position);
				}
				None => {
					println!("FUCK");
				}
			}
		}

		if let Some(storage) = storage {
			if let Some(star_files_to_not_render) = storage.get_string("star_files_to_not_render") {
				for file_name in star_files_to_not_render.split('|') {
					stars_categories_active.entry(file_name.to_string()).and_modify(|val| *val = false);
				}
			}
			if let Some(line_files_to_not_render) = storage.get_string("line_files_to_not_render") {
				for file_name in line_files_to_not_render.split('|') {
					lines_categories_active.entry(file_name.to_string()).and_modify(|val| *val = false);
				}
			}
			if let Some(deepsky_files_to_not_render) = storage.get_string("deepsky_files_to_not_render") {
				for file_name in deepsky_files_to_not_render.split('|') {
					deepskies_categories_active.entry(file_name.to_string()).and_modify(|val| *val = false);
				}
			}
			if let Some(marker_files_to_not_render) = storage.get_string("marker_files_to_not_render") {
				for file_name in marker_files_to_not_render.split('|') {
					markers_categories_active.entry(file_name.to_string()).and_modify(|val| *val = false);
				}
			}
			if let Some(star_names_files_to_not_use) = storage.get_string("star_names_files_to_not_use") {
				for file_name in star_names_files_to_not_use.split('|') {
					star_names_categories_active.entry(file_name.to_string()).and_modify(|val| *val = false);
				}
			}
		}

		let mut light_pollution_place_to_mag: HashMap<LightPollution, [f32; 2]> = HashMap::with_capacity(MAG_TO_LIGHT_POLLUTION_RAW.len());
		for &(mag_offset, mag_scale, place) in &MAG_TO_LIGHT_POLLUTION_RAW {
			light_pollution_place_to_mag.insert(place, [mag_offset, mag_scale]);
		}

		let viewport_rect = egui::Rect::from_two_pos(egui::pos2(0.0, 0.0), egui::pos2(0.0, 0.0));
		Ok(Self {
			stars: catalog,
			stars_categories_active,
			lines,
			lines_categories_active,
			deepskies,
			deepskies_categories_active,
			markers,
			markers_categories_active,
			star_names,
			star_names_categories_active,
			constellations,
			zoom: 1.0,
			star_renderers: HashMap::new(),
			line_renderers: HashMap::new(),
			deepsky_renderers: HashMap::new(),
			marker_renderers: HashMap::new(),

			mag_scale: 0.3,
			mag_offset: 6.0,
			light_pollution_place: LightPollution::Default,
			light_pollution_place_to_mag,

			viewport_rect,
			deepsky_render_mag_decrease: 0.0,

			rotation: Rotation3::new(Vector3::new(0.0, 0.0, 0.0)),
		})
	}

	// TODO: Make this always for example halve the FOV
	pub fn zoom(&mut self, velocity: f32) {
		let future_zoom = self.zoom + velocity * self.zoom;
		//A check is needed since negative zoom breaks everything
		if ZOOM_CAP > future_zoom && future_zoom > 0.0 {
			self.zoom = future_zoom
		}
	}

	pub fn get_zoom(&self) -> f32 {
		self.zoom
	}

	pub fn init(&mut self) {
		let [mag_offset, mag_scale] = self.light_pollution_place_to_mag_settings(&self.light_pollution_place);
		self.mag_offset = mag_offset;
		self.mag_scale = mag_scale;
		self.init_renderers();
	}

	pub fn init_renderers(&mut self) {
		self.star_renderers = HashMap::new();
		let mut active_star_groups = Vec::new();
		for name in self.stars.keys() {
			let active = self.stars_categories_active.entry(name.to_owned()).or_insert(true);
			if !*active {
				continue;
			}
			active_star_groups.push(name.to_owned());
		}
		for name in active_star_groups {
			self.init_single_renderer("stars", &name);
		}

		self.line_renderers = HashMap::new();
		let mut active_line_groups = Vec::new();
		for name in self.lines.keys() {
			let active = self.lines_categories_active.entry(name.to_owned()).or_insert(true);
			if !*active {
				continue;
			}
			active_line_groups.push(name.to_owned());
		}
		for name in active_line_groups {
			self.init_single_renderer("lines", &name);
		}

		self.deepsky_renderers = HashMap::new();
		let mut active_deepsky_groups = Vec::new();
		for name in self.deepskies.keys() {
			let active = self.deepskies_categories_active.entry(name.to_owned()).or_insert(true);
			if !*active {
				continue;
			}
			active_deepsky_groups.push(name.to_owned());
		}
		for name in active_deepsky_groups {
			self.init_single_renderer("deepskies", &name);
		}

		self.marker_renderers = HashMap::new();
		let mut active_markers_groups = Vec::new();
		for name in self.markers.keys() {
			let active = self.markers_categories_active.entry(name.to_owned()).or_insert(true);
			if !*active {
				continue;
			}
			active_markers_groups.push(name.to_owned());
		}
		for name in active_markers_groups {
			self.init_single_renderer("markers", &name);
		}
	}

	pub fn init_single_renderer(&mut self, category: &str, name: &str) {
		if category == "stars" {
			if let Some(stars) = self.stars.get(name) {
				self.star_renderers
					.insert(name.to_string(), stars.iter().map(|star| star.get_renderer(self.rotation.matrix())).collect());
			}
		} else if category == "lines" {
			if let Some(lines) = self.lines.get(name) {
				self.line_renderers
					.insert(name.to_string(), lines.iter().map(|line| line.get_renderer(self.rotation.matrix())).collect());
			}
		} else if category == "deepskies" {
			if let Some(deepskies) = self.deepskies.get(name) {
				self.deepsky_renderers
					.insert(name.to_string(), deepskies.iter().map(|deepsky| deepsky.get_renderer(self.rotation.matrix())).collect());
			}
		} else if category == "markers" {
			if let Some(markers) = self.markers.get(name) {
				self.marker_renderers
					.insert(name.to_string(), markers.iter().filter_map(|marker| marker.get_renderer(self.rotation.matrix())).collect());
			}
		}
	}

	pub fn deinit_single_renderer(&mut self, category: &str, name: &str) {
		if category == "stars" {
			self.star_renderers.insert(name.to_string(), Vec::new());
		} else if category == "lines" {
			self.line_renderers.insert(name.to_string(), Vec::new());
		} else if category == "deepskies" {
			self.deepsky_renderers.insert(name.to_string(), Vec::new());
		} else if category == "markers" {
			self.marker_renderers.insert(name.to_string(), Vec::new());
		}
	}

	pub fn mag_to_radius(&self, vmag: f32) -> f32 {
		let mag = self.mag_scale * (self.mag_offset - vmag) + 0.5;
		if mag < 0.35 {
			return 0.0;
		} else {
			return mag;
		}
	}

	pub fn project_screen_pos(&self, screen_pos: egui::Pos2) -> Vector3<f32> {
		cast_onto_sphere(self, &screen_pos)
	}

	pub fn mag_settings_to_light_pollution_place(&self, mag_offset: f32, mag_scale: f32) -> LightPollution {
		for (&place, &[mag_off, mag_sca]) in &self.light_pollution_place_to_mag {
			if mag_off == mag_offset && mag_sca == mag_scale {
				return place;
			}
		}
		LightPollution::NoSpecific
	}

	pub fn light_pollution_place_to_mag_settings(&self, place: &LightPollution) -> [f32; 2] {
		if let Some(settings) = self.light_pollution_place_to_mag.get(place) {
			*settings
		} else {
			[self.mag_offset, self.mag_scale]
		}
	}
	pub fn to_equatorial_coordinates(vector: Vector3<f32>) -> (f32, f32) {
		cartesian_to_spherical(vector)
	}
	pub fn determine_constellation(&self, point: (f32, f32)) -> String {
		let mut in_constellation = String::from("Undefined");
		for constellation in &self.constellations {
			let (abbreviation, constellation) = constellation;
			if is_inside_polygon(constellation.vertices.to_owned(), point, MERIDIAN_CONSTELLATIONS.contains(&abbreviation.as_str())) {
				in_constellation = abbreviation.to_owned();
			}
		}
		if in_constellation == "Undefined" {
			let (_ra, dec) = point;
			if dec > 0.0 {
				in_constellation = String::from("umi");
			} else {
				in_constellation = String::from("");
			}
		}
		in_constellation
	}
}

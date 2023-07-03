use crate::structs::graphics_settings::GraphicsSettings;
use eframe::{egui, epaint::Color32};
use nalgebra::{Matrix3, Vector3};
use std::{collections::HashMap, error::Error, f32::consts::PI, fs};

const LINES_FOLDER: &str = "./sphere/lines";
const STARS_FOLDER: &str = "./sphere/stars";

#[path = "../../geometry.rs"]
mod geometry;
use geometry::project_point;

mod lines;
use lines::{LineRenderer, SkyLine, SkyLineRaw};
mod stars;
use stars::{Star, StarRaw, StarRenderer};

pub struct Marker {
	pub normal: Vector3<f32>,
}

pub struct CellestialSphere {
	pub stars: HashMap<String, Vec<Star>>,
	pub stars_categories_active: HashMap<String, bool>,
	pub lines: Vec<SkyLine>,
	pub markers: Vec<Marker>,
	zoom: f32,
	star_renderers: HashMap<String, Vec<StarRenderer>>,
	line_renderers: Vec<LineRenderer>,
	mag_scale: f32,
	mag_offset: f32,
	pub viewport_rect: egui::Rect,

	pub rotation_dec: f32,
	pub rotation_ra: f32,
	pub rotation_matrix: Matrix3<f32>,
}

impl CellestialSphere {
	//Renders a circle based on its current normal (does NOT account for the rotation of the sphere)
	pub fn render_circle(&self, normal: &Vector3<f32>, radius: f32, color: eframe::epaint::Color32, painter: &egui::Painter) {
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

	//Renders the entire sphere view
	pub fn render_sky(&self, painter: &egui::Painter, graphics_settings: &GraphicsSettings) {
		//some stuff lol
		for line_renderer in &self.line_renderers {
			line_renderer.render(&self, painter);
		}
		for (_, star_renderers) in &self.star_renderers {
			for star_renderer in star_renderers {
				star_renderer.render(&self, painter, graphics_settings);
			}
		}
	}

	pub fn load() -> Result<Self, Box<dyn Error>> {
		let star_color = eframe::epaint::Color32::WHITE;
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
				let entry = catalog.entry(file_name.clone()).or_insert(Vec::new());
				entry.push(star);
				if !stars_categories_active.contains_key(&file_name) {
					stars_categories_active.insert(file_name.clone(), true);
				}
			}
		}
		let mut lines: Vec<SkyLine> = Vec::new();
		let files: Result<fs::ReadDir, std::io::Error> = fs::read_dir(LINES_FOLDER);
		for file in (files?).flatten() {
			let reader: Result<csv::Reader<std::fs::File>, csv::Error> = csv::Reader::from_path(file.path());

			for line_raw in reader?.deserialize() {
				let line_raw: SkyLineRaw = line_raw?;
				let line = SkyLine::from_raw(line_raw, star_color);
				lines.push(line);
			}
		}

		let viewport_rect = egui::Rect::from_two_pos(egui::pos2(0.0, 0.0), egui::pos2(0.0, 0.0));
		Ok(Self {
			stars: catalog,
			stars_categories_active,
			lines,
			markers: Vec::new(),
			zoom: 1.0,
			star_renderers: HashMap::new(),
			line_renderers: Vec::new(),
			mag_scale: 0.3,
			mag_offset: 6.0,
			viewport_rect,
			rotation_dec: 0.0,
			rotation_ra: 0.0,
			rotation_matrix: Matrix3::identity(),
		})
	}

	// TODO: Make this always for example halve the FOV
	pub fn zoom(&mut self, velocity: f32) {
		let future_zoom = self.zoom + velocity;
		//A check is needed since negative zoom breaks everything
		if future_zoom > 0.0 {
			self.zoom = future_zoom
		}
	}

	pub fn get_zoom(&self) -> f32 {
		self.zoom
	}

	pub fn init(&mut self) {
		self.init_renderers();
	}

	pub fn init_renderers(&mut self) {
		let (rot_de_s, rot_de_c) = ((90.0 - self.rotation_dec) * PI / 180.0).sin_cos();
		let (rot_ra_s, rot_ra_c) = (self.rotation_ra * PI / 180.0).sin_cos();
		let rotation_x_matrix = Matrix3::new(1.0, 0.0, 0.0, 0.0, rot_de_c, -rot_de_s, 0.0, rot_de_s, rot_de_c);
		let rotation_z_matrix = Matrix3::new(rot_ra_c, -rot_ra_s, 0.0, rot_ra_s, rot_ra_c, 0.0, 0.0, 0.0, 1.0);
		let rotation_matrix = rotation_x_matrix * rotation_z_matrix;
		self.rotation_matrix = rotation_matrix;
		self.star_renderers = HashMap::new();
		let mut active_star_groups = Vec::new();
		for (name, _) in &self.stars {
			let active = self.stars_categories_active.entry(name.to_owned()).or_insert(true);
			if !*active {
				continue;
			}
			active_star_groups.push(name.to_owned());
		}
		for name in active_star_groups {
			self.init_single_renderer("stars", &name, rotation_matrix);
		}
		self.line_renderers = self.lines.iter().map(|i| i.get_renderer(rotation_matrix)).collect();
	}

	pub fn init_single_renderer(&mut self, category: &str, name: &str, rotation_matrix: Matrix3<f32>) {
		if category == "stars" {
			if let Some(stars) = self.stars.get(name) {
				self.star_renderers.insert(name.to_string(), stars.iter().map(|star| star.get_renderer(rotation_matrix)).collect());
			}
		}
	}

	pub fn deinit_single_renderer(&mut self, category: &str, name: &str) {
		if category == "stars" {
			self.star_renderers.insert(name.to_string(), Vec::new());
		}
	}

	pub fn mag_to_radius(&self, vmag: f32) -> f32 {
		self.mag_scale * (self.mag_offset - vmag)
	}
}

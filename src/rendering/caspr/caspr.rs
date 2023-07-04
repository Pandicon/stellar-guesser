use eframe::{egui, epaint::Color32};
use nalgebra::{Matrix3, Vector3,Rotation3};
use std::{error::Error, f32::consts::PI, fs};

const LINES_FOLDER: &str = "./sphere/lines";
const STARS_FOLDER: &str = "./sphere/stars";

#[path = "../../geometry.rs"]
mod geometry;
use geometry::project_point;

mod lines;
use lines::{LineRenderer, SkyLine, SkyLineRaw};
mod stars;
use stars::{Star, StarRaw, StarRenderer};

use self::geometry::cast_onto_sphere;

pub struct Marker {
	pub normal: Vector3<f32>,
}

pub struct CellestialSphere {
	pub stars: Vec<Star>,
	pub lines: Vec<SkyLine>,
	pub markers: Vec<Marker>,
	zoom: f32,
	star_renderers: Vec<StarRenderer>,
	line_renderers: Vec<LineRenderer>,
	mag_scale: f32,
	mag_offset: f32,
	star_color: eframe::epaint::Color32,
	pub viewport_rect: egui::Rect,

	pub rotation:Rotation3<f32>
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
	pub fn render_sky(&self, painter: &egui::Painter) {
		//some stuff lol
		for line_renderer in &self.line_renderers {
			line_renderer.render(&self, painter)
		}
		for star_renderer in &self.star_renderers {
			star_renderer.render(&self, painter)
		}
	}

	pub fn load() -> Result<Self, Box<dyn Error>> {
		let star_color = eframe::epaint::Color32::WHITE;
		let mut catalog: Vec<Star> = Vec::new();
		let files = fs::read_dir(STARS_FOLDER);
		for file in (files?).flatten() {
			let reader: Result<csv::Reader<std::fs::File>, csv::Error> = csv::Reader::from_path(file.path());

			for star_raw in reader?.deserialize() {
				let star_raw: StarRaw = star_raw?;
				let star = Star::from_raw(star_raw, star_color);
				catalog.push(star);
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
			lines,
			markers: Vec::new(),
			zoom: 1.0,
			star_renderers: Vec::new(),
			line_renderers: Vec::new(),
			mag_scale: 0.3,
			mag_offset: 6.0,
			star_color,
			viewport_rect,
			rotation:Rotation3::new(Vector3::new(0.0, 0.0,0.0))
		})
	}

	// TODO: Make this always for example halve the FOV
	pub fn zoom(&mut self, velocity: f32) {
		let future_zoom = self.zoom + velocity*self.zoom;
		//A check is needed since negative zoom breaks everything
		if future_zoom >0.0{
			self.zoom=future_zoom
		}
	}

	pub fn get_zoom(&self) -> f32 {
		self.zoom
	}

	pub fn init(&mut self) {
		self.init_renderers();
	}

	pub fn init_renderers(&mut self) {
		self.star_renderers = self.stars.iter().map(|i| i.get_renderer(self.rotation.matrix())).collect();
		self.line_renderers = self.lines.iter().map(|i| i.get_renderer(self.rotation.matrix())).collect();
	}

	pub fn mag_to_radius(&self, vmag: f32) -> f32 {
		self.mag_scale * (self.mag_offset - vmag)+0.5
	}

	pub fn project_screen_pos(&self,screen_pos:egui::Pos2) -> Vector3<f32>{
		cast_onto_sphere(self, &screen_pos)
	}
}

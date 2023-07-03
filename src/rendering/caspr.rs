use eframe::{egui, epaint::Color32};
use nalgebra::{Matrix3, Vector2, Vector3};
use serde::Deserialize;
use std::{error::Error, f32::consts::PI, fs};

const STARS_FOLDER: &str = "./sphere/stars";

#[path = "../geometry.rs"]
mod geometry;

fn get_point_vector(ra: f32, dec: f32, rotation_matrix: Matrix3<f32>) -> Vector3<f32> {
	let (ra_s, ra_c) = ((-ra) * PI / 180.0).sin_cos();
	let (de_s, de_c) = ((90.0 - dec) * PI / 180.0).sin_cos();
	rotation_matrix * Vector3::new(de_s * ra_c, de_s * ra_s, de_c)
}

fn project_point(vector: &Vector3<f32>, zoom: f32, viewport_rect: egui::Rect) -> (egui::Pos2, bool) {
	let scale_factor = 1.0 - vector[2] / zoom;

	let rect_size = Vector2::new(viewport_rect.max[0] - viewport_rect.min[0], viewport_rect.max[1] - viewport_rect.min[1]);

	let screen_ratio = 2.0 / (rect_size[0] * rect_size[0] + rect_size[1] * rect_size[1]).sqrt();

	let point_coordinates = Vector2::new(vector[0] / scale_factor, vector[1] / scale_factor);

	(
		egui::Pos2::new(point_coordinates[0] / screen_ratio + rect_size[0] / 2.0, point_coordinates[1] / screen_ratio + rect_size[1] / 2.0),
		// Is it within the bounds that we want to render in? //TODO: Use the geometry::is_in_rect function
		// TODO: Probably fix this - see how it is rendering into the top panel
		((rect_size[0] * screen_ratio / 2.0 > point_coordinates[0]) && (point_coordinates[0] > -rect_size[0] * screen_ratio / 2.0))
			|| ((rect_size[1] * screen_ratio / 2.0 > point_coordinates[1]) && (point_coordinates[1] > -rect_size[1] * screen_ratio / 2.0)),
	)
}

#[derive(Clone, Copy, Deserialize)]
pub struct Star {
	pub ra: f32,
	pub dec: f32,
	pub vmag: f32,
	pub colour: Color32,
}

#[derive(Clone, Deserialize)]
pub struct StarRaw {
	pub ra: f32,
	pub dec: f32,
	pub vmag: f32,
	pub colour: Option<String>,
}

impl Star {
	pub fn get_renderer(&self, rotation_matrix: Matrix3<f32>) -> StarRenderer {
		StarRenderer::new(get_point_vector(self.ra, self.dec, rotation_matrix), self.vmag, self.colour)
	}

	pub fn from_raw(raw_star: StarRaw, default_colour: Color32) -> Self {
		let colour = if let Some(colour_string) = raw_star.colour {
			if let Ok(mut col_raw) = i64::from_str_radix(&colour_string, 16) {
				let a = col_raw % 256;
				col_raw /= 256; // a < 256, so there is no need to subtract it before division as it can only create a decimal part which is dropped in integer division
				let b = col_raw % 256;
				col_raw /= 256;
				let g = col_raw % 256;
				col_raw /= 256;
				let r = col_raw;
				Color32::from_rgba_premultiplied(r as u8, g as u8, b as u8, a as u8)
			} else {
				default_colour
			}
		} else {
			default_colour
		};
		Self {
			ra: raw_star.ra,
			dec: raw_star.dec,
			vmag: raw_star.vmag,
			colour,
		}
	}
}

pub struct Marker {
	pub normal: Vector3<f32>,
}

pub struct StarRenderer {
	pub unit_vector: Vector3<f32>,
	pub vmag: f32,
	pub colour: Color32,
}

impl StarRenderer {
	pub fn new(vector: Vector3<f32>, magnitude: f32, colour: Color32) -> Self {
		Self {
			unit_vector: vector,
			vmag: magnitude,
			colour,
		}
	}

	pub fn render(&self, cellestial_sphere: &CellestialSphere, painter: &egui::Painter) {
		cellestial_sphere.render_circle(&self.unit_vector, cellestial_sphere.mag_to_radius(self.vmag), self.colour, painter);
	}
}

pub struct CellestialSphere {
	pub stars: Vec<Star>,
	pub markers: Vec<Marker>,
	zoom: f32,
	star_renderers: Vec<StarRenderer>,
	mag_scale: f32,
	mag_offset: f32,
	star_color: eframe::epaint::Color32,
	pub viewport_rect: egui::Rect,

	pub rotation_dec: f32,
	pub rotation_ra: f32,
}

impl CellestialSphere {
	//Renders a circle based on its current normal (does NOT account for the rotation of the sphere)
	pub fn render_circle(&self, normal: &Vector3<f32>, radius: f32, color: eframe::epaint::Color32, painter: &egui::Painter) {
		let (projected_point, is_within_bounds) = project_point(normal, self.zoom, self.viewport_rect);

		if is_within_bounds {
			painter.circle_filled(projected_point, radius, color);
		}
	}

	//Renders the entire sphere view
	pub fn render_sky(&self, painter: &egui::Painter) {
		//some stuff lol
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

		let viewport_rect = egui::Rect::from_two_pos(egui::pos2(0.0, 0.0), egui::pos2(0.0, 0.0));
		Ok(Self {
			stars: catalog,
			markers: Vec::new(),
			zoom: 1.0,
			star_renderers: Vec::new(),
			mag_scale: 0.3,
			mag_offset: 6.0,
			star_color,
			viewport_rect,
			rotation_dec: 0.0,
			rotation_ra: 0.0,
		})
	}

	// TODO: Make this always for example halve the FOV
	pub fn zoom(&mut self, velocity: f32) {
		self.zoom += velocity;
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
		self.star_renderers = self.stars.iter().map(|i| i.get_renderer(rotation_matrix)).collect()
	}

	pub fn mag_to_radius(&self, vmag: f32) -> f32 {
		self.mag_scale * (self.mag_offset - vmag)
	}
}

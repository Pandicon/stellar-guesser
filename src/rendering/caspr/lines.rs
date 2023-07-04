use eframe::{egui, epaint::Color32};
use nalgebra::{Matrix3, Vector3};
use serde::Deserialize;

#[path = "../../geometry.rs"]
mod geometry;
use geometry::get_point_vector;

#[path = "../../graphics.rs"]
mod graphics;
use graphics::parse_colour;

use super::CellestialSphere;

pub struct SkyLine {
	pub ra_start: f32,
	pub dec_start: f32,
	pub ra_end: f32,
	pub dec_end: f32,
	pub colour: Color32,
	pub width: f32,
}

impl SkyLine {
	pub fn get_renderer(&self, rotation_matrix: &Matrix3<f32>) -> LineRenderer {
		LineRenderer::new(
			get_point_vector(self.ra_start, self.dec_start, rotation_matrix),
			get_point_vector(self.ra_end, self.dec_end, rotation_matrix),
			self.colour,
			self.width,
		)
	}

	pub fn from_raw(raw_line: SkyLineRaw, default_colour: Color32) -> Self {
		let colour = parse_colour(raw_line.colour, default_colour);
		Self {
			ra_start: raw_line.ra_start,
			dec_start: raw_line.dec_start,
			ra_end: raw_line.ra_end,
			dec_end: raw_line.dec_end,
			colour,
			width: raw_line.width,
		}
	}
}

#[derive(Clone, Deserialize)]
pub struct SkyLineRaw {
	pub ra_start: f32,
	pub dec_start: f32,
	pub ra_end: f32,
	pub dec_end: f32,
	pub colour: Option<String>,
	pub width: f32,
}

pub struct LineRenderer {
	pub start: Vector3<f32>,
	pub end: Vector3<f32>,
	pub colour: Color32,
	pub width: f32,
}

impl LineRenderer {
	pub fn new(start: Vector3<f32>, end: Vector3<f32>, colour: Color32, width: f32) -> Self {
		Self { start, end, colour, width }
	}

	pub fn render(&self, cellestial_sphere: &CellestialSphere, painter: &egui::Painter) {
		cellestial_sphere.render_line(&self.start, &self.end, self.colour, self.width, painter);
	}
}

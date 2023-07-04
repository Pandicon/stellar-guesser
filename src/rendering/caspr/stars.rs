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
	pub fn get_renderer(&self, rotation_matrix: &Matrix3<f32>) -> StarRenderer {
		StarRenderer::new(get_point_vector(self.ra, self.dec, rotation_matrix), self.vmag, self.colour)
	}

	pub fn from_raw(raw_star: StarRaw, default_colour: Color32) -> Self {
		let colour = parse_colour(raw_star.colour, default_colour);
		Self {
			ra: raw_star.ra,
			dec: raw_star.dec,
			vmag: raw_star.vmag,
			colour,
		}
	}
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

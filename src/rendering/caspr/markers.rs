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
pub struct Marker {
	pub ra: f32,
	pub dec: f32,
	pub colour: Color32,
	pub line_width: f32,
	pub angular_radius: Option<f32>,
	pub pixel_radius: Option<f32>,
	pub angular_width: Option<f32>,
	pub pixel_width: Option<f32>,
}

#[derive(Clone, Deserialize)]
pub struct MarkerRaw {
	pub ra: f32,
	pub dec: f32,
	pub colour: String,
	pub line_width: f32,
	pub angular_radius: Option<f32>,
	pub pixel_radius: Option<f32>,
	pub angular_width: Option<f32>,
	pub pixel_width: Option<f32>,
}

impl Marker {
	pub fn get_renderer(&self, rotation_matrix: &Matrix3<f32>) -> Option<MarkerRenderer> {
		if self.angular_radius.is_none() && self.pixel_radius.is_none() && self.angular_width.is_none() && self.pixel_width.is_none() {
			return None;
		}
		let other_vec = if let Some(angular_radius) = self.angular_radius {
			Some(get_point_vector(
				self.ra,
				if self.dec + angular_radius <= 90.0 {
					self.dec + angular_radius
				} else {
					self.dec - angular_radius
				},
				rotation_matrix,
			))
		} else if let Some(angular_width) = self.angular_width {
			Some(get_point_vector(
				self.ra,
				if self.dec + angular_width <= 90.0 { self.dec + angular_width } else { self.dec - angular_width },
				rotation_matrix,
			))
		} else {
			None
		};
		Some(MarkerRenderer::new(get_point_vector(self.ra, self.dec, rotation_matrix), other_vec, &self))
	}

	pub fn from_raw(raw_marker: MarkerRaw, default_colour: Color32) -> Self {
		let colour = parse_colour(Some(raw_marker.colour), default_colour);
		Self {
			ra: raw_marker.ra,
			dec: raw_marker.dec,
			colour,
			line_width: raw_marker.line_width,
			angular_radius: raw_marker.angular_radius,
			pixel_radius: raw_marker.pixel_radius,
			angular_width: raw_marker.angular_width,
			pixel_width: raw_marker.pixel_width,
		}
	}
}

pub struct MarkerRenderer {
	pub unit_vector: Vector3<f32>,
	pub unit_vector_other_point: Option<Vector3<f32>>,
	pub colour: Color32,
	pub line_width: f32,
	pub angular_radius: Option<f32>,
	pub pixel_radius: Option<f32>,
	pub angular_width: Option<f32>,
	pub pixel_width: Option<f32>,
	pub circle: bool,
}

impl MarkerRenderer {
	pub fn new(vector: Vector3<f32>, vector_other_point: Option<Vector3<f32>>, marker: &Marker) -> Self {
		Self {
			unit_vector: vector,
			unit_vector_other_point: vector_other_point,
			colour: marker.colour,
			line_width: marker.line_width,
			angular_radius: marker.angular_radius,
			pixel_radius: marker.pixel_radius,
			angular_width: marker.angular_width,
			pixel_width: marker.pixel_width,
			circle: marker.angular_radius.is_some() || marker.pixel_radius.is_some(),
		}
	}

	pub fn render(&self, cellestial_sphere: &CellestialSphere, painter: &egui::Painter) {
		cellestial_sphere.render_marker(
			&self.unit_vector,
			&self.unit_vector_other_point,
			self.circle,
			if self.circle { self.pixel_radius } else { self.pixel_width },
			self.colour,
			self.line_width,
			painter,
		)
	}
}

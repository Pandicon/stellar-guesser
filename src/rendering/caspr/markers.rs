use eframe::{egui, epaint::Color32};
use nalgebra::{Matrix3, Vector3};
use serde::Deserialize;

#[path = "../../geometry.rs"]
mod geometry;
use geometry::get_point_vector;

#[path = "../../graphics.rs"]
mod graphics;
use graphics::parse_colour;

use crate::caspr::CellestialSphere;

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
		} else {
			self.angular_width.map(|angular_width| {
				get_point_vector(
					self.ra,
					if self.dec + angular_width <= 90.0 { self.dec + angular_width } else { self.dec - angular_width },
					rotation_matrix,
				)
			})
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

	/**
	 * ra - the right ascension
	 * dec - the declination
	 * colour - the colour of the marker
	 * line_width - the width of the line of the marker
	 * half_size - the distance from the centre to the edge of the marker (radius for circular markers)
	 * circular - if the marker is circular or not, if not then it is a cross
	 * angular_size - if the half_size is in degrees or in pixels
	 */
	pub fn new(ra: f32, dec: f32, colour: Color32, line_width: f32, half_size: f32, circular: bool, angular_size: bool) -> Self {
		let [angular_radius, pixel_radius, angular_width, pixel_width] = if circular {
			if angular_size {
				[Some(half_size), None, None, None]
			} else {
				[None, Some(half_size), None, None]
			}
		} else {
			if angular_size {
				[None, None, Some(half_size), None]
			} else {
				[None, None, None, Some(half_size)]
			}
		};
		Self {
			ra,
			dec,
			colour,
			line_width,
			angular_radius,
			pixel_radius,
			angular_width,
			pixel_width,
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

use egui::epaint::Color32;
use nalgebra::{Matrix3, Vector3};
use serde::Deserialize;

use crate::geometry;
use geometry::get_point_vector;

use crate::graphics;
use graphics::parse_colour;

use super::caspr::CellestialSphere;

#[derive(Clone, Deserialize)]
pub struct Deepsky {
	pub names: Option<Vec<String>>,
	pub messier: Option<String>,
	pub caldwell: Option<String>,
	pub ngc: Option<String>,
	pub ic: Option<String>,
	pub object_type: Option<String>,
	pub constellation: String,
	pub ra: f32,
	pub dec: f32,
	pub mag: String,
	pub distance: f32,
	pub colour: Color32,
	pub images: Vec<crate::structs::image_info::ImageInfo>,
}

#[derive(Clone, Deserialize)]
pub struct DeepskyRaw {
	pub names: Option<String>,
	pub messier: Option<String>,
	pub caldwell: Option<String>,
	pub ngc: Option<String>,
	pub ic: Option<String>,
	pub object_type: Option<String>,
	pub constellation: String,
	pub ra: f32,
	pub dec: f32,
	pub mag: String,
	pub distance: f32,
	pub colour: Option<String>,
}

impl Deepsky {
	pub fn get_renderer(&self, rotation_matrix: &Matrix3<f32>) -> DeepskyRenderer {
		DeepskyRenderer::new(get_point_vector(self.ra, self.dec, rotation_matrix), self.colour)
	}

	pub fn from_raw(raw_deepsky: DeepskyRaw, default_colour: Color32, images_data: Vec<crate::structs::image_info::ImageInfo>) -> Self {
		let names = raw_deepsky.names.map(|raw_names| raw_names.split(';').map(|s| s.to_owned()).collect());
		let colour = parse_colour(raw_deepsky.colour, default_colour);
		Self {
			names,
			messier: raw_deepsky.messier,
			caldwell: raw_deepsky.caldwell,
			ngc: raw_deepsky.ngc,
			ic: raw_deepsky.ic,
			object_type: raw_deepsky.object_type,
			constellation: raw_deepsky.constellation,
			ra: raw_deepsky.ra,
			dec: raw_deepsky.dec,
			mag: raw_deepsky.mag,
			distance: raw_deepsky.distance,
			colour,
			images: images_data,
		}
	}
}

pub struct DeepskyRenderer {
	pub unit_vector: Vector3<f32>,
	pub colour: Color32,
}

impl DeepskyRenderer {
	pub fn new(vector: Vector3<f32>, colour: Color32) -> Self {
		Self { unit_vector: vector, colour }
	}

	pub fn render(&self, cellestial_sphere: &CellestialSphere, painter: &egui::Painter) {
		//cellestial_sphere.render_circle(&self.unit_vector, cellestial_sphere.mag_to_radius(self.vmag - magnitude_decrease), self.colour, painter);
		cellestial_sphere.render_marker(&self.unit_vector, &None, false, Some(5.0), self.colour, 1.5, painter);
	}
}

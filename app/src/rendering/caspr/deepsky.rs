use eframe::egui;
use egui::epaint::Color32;
use nalgebra::{Matrix3, Vector3};
use serde::Deserialize;

use crate::graphics::parse_colour_option;

use super::renderer::CellestialSphere;

pub struct Deepskies {
    pub colour: Color32,
    pub active: bool,
    pub deepskies: Vec<Deepsky>,
}

#[derive(Clone, Deserialize)]
pub struct Deepsky {
    pub object_id: u64,
    pub names: Option<Vec<String>>,
    pub messier: Option<u32>,
    pub caldwell: Option<u32>,
    pub ngc: Option<u32>,
    pub ic: Option<u32>,
    pub object_type: Option<String>,
    pub constellation: String,
    pub ra: angle::Deg<f32>,
    pub dec: angle::Deg<f32>,
    pub mag: String,
    pub distance: f32,
    pub images: Vec<crate::structs::image_info::ImageInfo>,
}

#[derive(Clone, Deserialize)]
pub struct DeepskyRaw {
    pub object_id: u64,
    pub names: Option<String>,
    pub messier: Option<u32>,
    pub caldwell: Option<u32>,
    pub ngc: Option<u32>,
    pub ic: Option<u32>,
    pub object_type: Option<String>,
    pub constellation: String,
    pub ra: angle::Deg<f32>,
    pub dec: angle::Deg<f32>,
    pub mag: String,
    pub distance: f32,
    pub colour: Option<String>,
}

impl Deepsky {
    pub fn get_renderer(&self, rotation_matrix: &Matrix3<f32>, colour: Color32) -> DeepskyRenderer {
        let name = match self.messier {
            Some(num) => Some(format!("M {num}")),
            None => self.caldwell.map(|num| format!("C {num}")),
        };

        DeepskyRenderer::new(sg_geometry::get_point_vector(self.ra, self.dec, rotation_matrix), colour, name)
    }

    pub fn from_raw(raw_deepsky: DeepskyRaw, images_data: Vec<crate::structs::image_info::ImageInfo>) -> (Self, Option<Color32>) {
        let names = raw_deepsky.names.map(|raw_names| raw_names.split(';').map(|s| s.to_owned()).filter(|s| !s.is_empty()).collect());
        let colour = parse_colour_option(raw_deepsky.colour);
        (
            Self {
                object_id: raw_deepsky.object_id,
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
                images: images_data,
            },
            colour,
        )
    }
}

pub struct DeepskyRenderer {
    pub unit_vector: Vector3<f32>,
    pub colour: Color32,
    pub label: Option<String>,
}

impl DeepskyRenderer {
    pub fn new(vector: Vector3<f32>, colour: Color32, label: Option<String>) -> Self {
        Self { unit_vector: vector, colour, label }
    }

    pub fn render(&self, cellestial_sphere: &CellestialSphere, painter: &egui::Painter) {
        //cellestial_sphere.render_circle(&self.unit_vector, cellestial_sphere.mag_to_radius(self.vmag - magnitude_decrease), self.colour, painter);
        cellestial_sphere.render_marker(&self.unit_vector, &None, false, Some(5.0), self.colour, 1.5, painter, self.label.clone());
    }
}

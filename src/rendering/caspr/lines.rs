use egui::epaint::Color32;
use nalgebra::{Matrix3, Vector3};
use serde::Deserialize;

use crate::geometry;
use geometry::get_point_vector;

use crate::graphics;
use graphics::parse_colour_option;

use super::renderer::CellestialSphere;

pub struct SkyLines {
    pub colour: Color32,
    pub active: bool,
    pub lines: Vec<SkyLine>,
}

pub struct SkyLine {
    pub ra_start: f32,
    pub dec_start: f32,
    pub ra_end: f32,
    pub dec_end: f32,
    pub width: f32,
}

impl SkyLine {
    pub fn get_renderer(&self, rotation_matrix: &Matrix3<f32>, colour: Color32) -> LineRenderer {
        LineRenderer::new(
            get_point_vector(self.ra_start, self.dec_start, rotation_matrix),
            get_point_vector(self.ra_end, self.dec_end, rotation_matrix),
            colour,
            self.width,
        )
    }

    pub fn from_raw(raw_line: SkyLineRaw) -> (Self, Option<Color32>) {
        let colour = parse_colour_option(raw_line.colour);
        (
            Self {
                ra_start: raw_line.ra_start,
                dec_start: raw_line.dec_start,
                ra_end: raw_line.ra_end,
                dec_end: raw_line.dec_end,
                width: raw_line.width,
            },
            colour,
        )
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

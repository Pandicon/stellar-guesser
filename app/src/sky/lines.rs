use eframe::egui;
use egui::epaint::Color32;
use nalgebra::Matrix3;
use serde::Deserialize;

use crate::{graphics, rendering::caspr};
use graphics::parse_colour_option;

pub struct SkyLines {
    pub colour: Color32,
    pub active: bool,
    pub lines: Vec<SkyLine>,
}

pub struct SkyLine {
    pub ra_start: angle::Deg<f32>,
    pub dec_start: angle::Deg<f32>,
    pub ra_end: angle::Deg<f32>,
    pub dec_end: angle::Deg<f32>,
    pub width: f32,
}

impl SkyLine {
    pub fn get_renderer(&self, rotation_matrix: &Matrix3<f32>, colour: Color32) -> caspr::lines::LineRenderer {
        caspr::lines::LineRenderer::new(
            sg_geometry::get_point_vector(self.ra_start, self.dec_start, rotation_matrix),
            sg_geometry::get_point_vector(self.ra_end, self.dec_end, rotation_matrix),
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
    pub ra_start: angle::Deg<f32>,
    pub dec_start: angle::Deg<f32>,
    pub ra_end: angle::Deg<f32>,
    pub dec_end: angle::Deg<f32>,
    pub colour: Option<String>,
    pub width: f32,
}

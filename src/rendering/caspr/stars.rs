use crate::{rendering::themes::Theme, structs::graphics_settings::GraphicsSettings};
use eframe::egui;
use egui::epaint::Color32;
use nalgebra::{Matrix3, Vector3};
use serde::Deserialize;

use crate::geometry;
use geometry::get_point_vector;

use crate::graphics;
use graphics::parse_colour;

use super::{renderer::CellestialSphere, star_names::StarName};

#[derive(Clone, Deserialize)]
pub struct Star {
    pub ra: angle::Deg<f32>,
    pub dec: angle::Deg<f32>,
    pub vmag: f32,
    pub colour: Color32,
    #[allow(dead_code)]
    name_str: Option<String>,
    pub name: Option<StarName>,
    pub constellations_abbreviations: Vec<String>,
}

#[derive(Clone, Deserialize)]
pub struct StarRaw {
    pub ra: angle::Deg<f32>,
    pub dec: angle::Deg<f32>,
    pub vmag: f32,
    pub colour: Option<String>,
    pub name: Option<String>,
    pub constellations: String,
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
            name_str: raw_star.name,
            name: None,
            constellations_abbreviations: raw_star.constellations.split(';').map(|abbrev| abbrev.to_string()).collect(),
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

    pub fn render(&self, cellestial_sphere: &CellestialSphere, painter: &egui::Painter, graphics_settings: &GraphicsSettings, theme: &Theme) {
        cellestial_sphere.render_circle(
            &self.unit_vector,
            cellestial_sphere.mag_to_radius(self.vmag),
            if graphics_settings.use_default_star_colour {
                theme.game_visuals.default_star_colour
            } else {
                self.colour
            },
            painter,
        );
    }
}

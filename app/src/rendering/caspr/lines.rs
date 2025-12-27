use eframe::egui;
use egui::epaint::Color32;
use nalgebra::Vector3;

use super::renderer::CellestialSphere;

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

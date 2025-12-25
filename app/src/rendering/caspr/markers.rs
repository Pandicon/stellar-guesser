use eframe::egui;
use egui::epaint::Color32;
use nalgebra::Vector3;

use crate::rendering::caspr::renderer::CellestialSphere;
use crate::sky;

pub struct MarkerRenderer {
    pub unit_vector: Vector3<f32>,
    pub unit_vector_other_point: Option<Vector3<f32>>,
    pub colour: Color32,
    pub line_width: f32,
    pub angular_radius: Option<angle::Deg<f32>>,
    pub pixel_radius: Option<f32>,
    pub angular_width: Option<angle::Deg<f32>>,
    pub pixel_width: Option<f32>,
    pub circle: bool,
    pub label: Option<String>,
}

impl MarkerRenderer {
    pub fn new(vector: Vector3<f32>, vector_other_point: Option<Vector3<f32>>, marker: &sky::markers::Marker, colour: Color32) -> Self {
        Self {
            unit_vector: vector,
            unit_vector_other_point: vector_other_point,
            colour,
            line_width: marker.line_width,
            angular_radius: marker.angular_radius,
            pixel_radius: marker.pixel_radius,
            angular_width: marker.angular_width,
            pixel_width: marker.pixel_width,
            circle: marker.angular_radius.is_some() || marker.pixel_radius.is_some(),
            label: marker.label.map(|a| a.iter().collect()),
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
            self.label.clone(),
        )
    }
}

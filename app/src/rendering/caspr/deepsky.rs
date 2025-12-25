use eframe::egui;
use egui::epaint::Color32;
use nalgebra::Vector3;

use super::renderer::CellestialSphere;

pub struct DeepskyRenderer {
    pub object_id: u64,
    pub unit_vector: Vector3<f32>,
    pub colour: Color32,
    pub label: Option<String>,
    pub disabled: bool,
}

impl DeepskyRenderer {
    pub fn new(object_id: u64, vector: Vector3<f32>, colour: Color32, label: Option<String>, disabled: bool) -> Self {
        Self {
            object_id,
            unit_vector: vector,
            colour,
            label,
            disabled,
        }
    }

    pub fn render(&self, cellestial_sphere: &CellestialSphere, painter: &egui::Painter) {
        if self.disabled {
            return;
        }
        //cellestial_sphere.render_circle(&self.unit_vector, cellestial_sphere.mag_to_radius(self.vmag - magnitude_decrease), self.colour, painter);
        cellestial_sphere.render_marker(&self.unit_vector, &None, false, Some(5.0), self.colour, 1.5, painter, self.label.clone());
    }
}

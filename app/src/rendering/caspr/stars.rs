use angle::Angle;
use eframe::egui;
use egui::epaint::Color32;

use crate::sky::star::MagnitudeToRadius;

pub struct StarRenderer {
    pub object_id: u64,
    pub screen_pos: egui::Pos2,
    pub is_on_screen: bool,
    pub radius: f32,
    pub colour: Color32,
    pub disabled: bool,
}

impl StarRenderer {
    pub fn new(object_id: u64, radius: f32, colour: Color32, screen_pos: egui::Pos2, is_on_screen: bool, disabled: bool) -> Self {
        Self {
            object_id,
            screen_pos,
            is_on_screen,
            radius,
            colour,
            disabled,
        }
    }

    pub fn render(&self, painter: &egui::Painter) {
        if self.disabled {
            return;
        }
        if Self::radius_enough_to_render(self.radius) && self.is_on_screen {
            painter.circle_filled(self.screen_pos, self.radius, self.colour);
        }
    }

    pub fn magnitude_to_radius(function_choice: MagnitudeToRadius, magnitude: f32, fov: angle::Deg<f32>) -> f32 {
        match function_choice {
            MagnitudeToRadius::Linear { mag_scale, mag_offset } => mag_scale * (mag_offset - magnitude),
            MagnitudeToRadius::Exponential { r_0, n, o } => r_0 * (180.0 * n / fov.value()).ln() * 10.0_f32.powf(-o * magnitude),
        }
    }

    pub fn radius_enough_to_render(radius: f32) -> bool {
        radius >= crate::MINIMUM_CIRCLE_RADIUS_TO_RENDER
    }
}

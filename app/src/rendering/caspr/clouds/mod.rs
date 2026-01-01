use angle::Angle;
use eframe::egui;
use noise::{MultiFractal, NoiseFn};

use crate::{rendering::caspr, sky};

pub mod renderer;

// The physical model is as follows:
// Each layer of clouds absorbs a set fraction of light from each star, so the received flux is F ~ F_0 * exp(-number of layers).
// However, the change in magnitude is m - m_0 ~ -log(F / F_0) = -log(exp(-number of layers)) = number of layers.
// So the decrease in magnitude is linear in the thickness of the cloud (roughly).
pub fn apply_dimming(sky: &mut sky::Sky, cellestial_sphere: &mut caspr::renderer::CellestialSphere) {
    let settings = &cellestial_sphere.sky_settings.cloud_settings;
    let seed = (chrono::Utc::now().timestamp().abs() % (u32::MAX as i64)) as u32;
    let cloud_generator: noise::Billow<noise::SuperSimplex> = noise::Billow::new(seed).set_octaves(settings.iterations);

    let texture_size = (90.0 / cellestial_sphere.sky_settings.cloud_settings.resolution.value()).floor() as u32;
    let (texture_data_faces, decrease_offset, multi) = renderer::CloudsRenderer::generate_texture_data(texture_size, &cloud_generator, settings);
    cellestial_sphere.textures.clouds_texture_to_upload = Some(caspr::textures::cubemap::Cubemap::<u8> {
        texture_size,
        texture_data: texture_data_faces,
        changed: true,
    });

    for star_set in sky.stars.values_mut() {
        for star in star_set {
            let mut coordinates = spherical_geometry::SphericalPoint::ra_dec_to_cartesian(*star.ra.to_rad().as_value(), *star.dec.to_rad().as_value());
            coordinates.y = -coordinates.y; // Convert to the graphics convention
            let decrease_raw = cloud_generator.get([coordinates.x as f64, coordinates.y as f64, coordinates.z as f64]) as f32;
            let decrease = (multi * (decrease_raw - decrease_offset)).max(0.0);
            star.magnitude_offset = decrease;
        }
    }
}

pub fn disable(stars: &mut std::collections::HashMap<String, Vec<sky::star::Star>>) {
    for star_set in stars.values_mut() {
        for star in star_set {
            star.magnitude_offset = 0.0;
        }
    }
}

#[derive(Clone, Copy)]
struct CloudSettingsChangesState {
    colour_changed: bool,
}

impl Default for CloudSettingsChangesState {
    fn default() -> Self {
        Self { colour_changed: false }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy)]
#[serde(default)]
pub struct CloudSettings {
    pub coverage: f32,
    pub thickness: f32,
    pub iterations: usize,
    pub enabled: bool,
    pub recalculate_on_change: bool,

    pub render: bool,
    pub opaque_thickness: f32,
    pub resolution: angle::Deg<f32>,
    colour: egui::Color32,

    #[serde(skip)]
    changes_state: CloudSettingsChangesState,
}

impl Default for CloudSettings {
    fn default() -> Self {
        Self {
            coverage: 0.5,
            thickness: 4.0,
            iterations: 8,
            enabled: false,
            recalculate_on_change: false,

            render: false,
            opaque_thickness: 4.0,
            resolution: angle::Deg(0.5),
            colour: egui::Color32::from_rgb(77, 77, 77),

            changes_state: Default::default(),
        }
    }
}

impl CloudSettings {
    pub fn clamp(&mut self) {
        self.coverage = self.coverage.clamp(0.0, 1.0);
        if self.thickness < 0.0 {
            self.thickness = 0.0;
        }
        if self.opaque_thickness < 0.0 {
            self.opaque_thickness = 0.0;
        }
        if self.iterations < 1 {
            self.iterations = 1;
        }
    }

    pub fn set_colour(&mut self, colour: egui::Color32) {
        if self.colour != colour {
            self.colour = colour;
            self.changes_state.colour_changed = true;
        }
    }

    pub fn get_colour(&self) -> egui::Color32 {
        self.colour
    }

    pub fn reset_changes_state(&mut self) {
        self.changes_state = Default::default();
    }
}

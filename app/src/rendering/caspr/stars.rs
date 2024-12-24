use angle::Angle;
use eframe::egui;
use egui::epaint::Color32;
use nalgebra::{Matrix3, Vector3};
use serde::Deserialize;

use crate::graphics;
use graphics::parse_colour;

use super::{renderer::CellestialSphere, star_names::StarName};

#[derive(Clone, Deserialize)]
pub struct Star {
    pub ra: angle::Deg<f32>,
    pub dec: angle::Deg<f32>,
    pub vmag: f32,
    pub default_colour: Color32,
    pub override_colour: Option<Color32>,
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
    pub bv: Option<String>,
    pub constellations: String,
}

impl Star {
    pub fn get_renderer(&self, rotation_matrix: &Matrix3<f32>, magnitude_to_radius_function: MagnitudeToRadius, fov: angle::Deg<f32>) -> StarRenderer {
        let colour = if let Some(col) = self.override_colour { col } else { self.default_colour };
        StarRenderer::new(sg_geometry::get_point_vector(self.ra, self.dec, rotation_matrix), self.vmag, colour, magnitude_to_radius_function, fov)
    }

    pub fn from_raw(raw_star: StarRaw, default_colour: Color32, override_colour: Option<Color32>) -> Self {
        let colour = if let Some(bv) = raw_star.bv {
            if let Ok(bv) = bv.parse() {
                let temperature = Star::bv_to_temperature(bv);
                Star::temperature_to_colour(temperature)
            } else {
                parse_colour(raw_star.colour, default_colour)
            }
        } else {
            parse_colour(raw_star.colour, default_colour)
        };
        Self {
            ra: raw_star.ra,
            dec: raw_star.dec,
            vmag: raw_star.vmag,
            default_colour: colour,
            override_colour,
            name_str: raw_star.name,
            name: None,
            constellations_abbreviations: raw_star.constellations.split(';').map(|abbrev| abbrev.to_string()).collect(),
        }
    }

    pub fn bv_to_temperature(bv: f32) -> f32 {
        4600.0 * (1.0 / (0.92 * bv + 1.7) + 1.0 / (0.92 * bv + 0.62))
    }

    pub fn temperature_to_colour(temperature: f32) -> Color32 {
        #[allow(clippy::const_is_empty)]
        if TEMPERATURE_TO_COLOUR.is_empty() {
            return Color32::WHITE;
        }
        match TEMPERATURE_TO_COLOUR.binary_search_by(|val| val.0.total_cmp(&temperature)) {
            Ok(i) => TEMPERATURE_TO_COLOUR[i].1,
            Err(i) => {
                // Could be inserted at i -> i-1 lower, i greater
                if i >= TEMPERATURE_TO_COLOUR.len() {
                    return TEMPERATURE_TO_COLOUR.iter().last().unwrap().1;
                }
                if i == 0 {
                    return TEMPERATURE_TO_COLOUR[0].1;
                }
                let mul = (temperature - TEMPERATURE_TO_COLOUR[i - 1].0) / (TEMPERATURE_TO_COLOUR[i].0 - TEMPERATURE_TO_COLOUR[i - 1].0);
                debug_assert!((0.0..=1.0).contains(&mul), "The colour multiplier has to be between 0 and 1");
                let dr = TEMPERATURE_TO_COLOUR[i].1.r() as f32 - TEMPERATURE_TO_COLOUR[i - 1].1.r() as f32;
                let r = TEMPERATURE_TO_COLOUR[i - 1].1.r() + (dr * mul) as u8;
                let dg = TEMPERATURE_TO_COLOUR[i].1.g() as f32 - TEMPERATURE_TO_COLOUR[i - 1].1.g() as f32;
                let g = TEMPERATURE_TO_COLOUR[i - 1].1.g() + (dg * mul) as u8;
                let db = TEMPERATURE_TO_COLOUR[i].1.b() as f32 - TEMPERATURE_TO_COLOUR[i - 1].1.b() as f32;
                let b = TEMPERATURE_TO_COLOUR[i - 1].1.b() + (db * mul) as u8;
                Color32::from_rgb(r, g, b)
            }
        }
    }
}

pub struct StarRenderer {
    pub unit_vector: Vector3<f32>,
    pub radius: f32,
    pub colour: Color32,
}

impl StarRenderer {
    pub fn new(vector: Vector3<f32>, magnitude: f32, colour: Color32, magnitude_to_radius_function: MagnitudeToRadius, fov: angle::Deg<f32>) -> Self {
        Self {
            unit_vector: vector,
            radius: Self::magnitude_to_radius(magnitude_to_radius_function, magnitude, fov),
            colour,
        }
    }

    pub fn render(&self, cellestial_sphere: &CellestialSphere, painter: &egui::Painter) {
        if self.radius >= crate::MINIMUM_CIRCLE_RADIUS_TO_RENDER {
            cellestial_sphere.render_circle(&self.unit_vector, self.radius, self.colour, painter);
        }
    }

    pub fn magnitude_to_radius(function_choice: MagnitudeToRadius, magnitude: f32, fov: angle::Deg<f32>) -> f32 {
        match function_choice {
            MagnitudeToRadius::Linear { mag_scale, mag_offset } => mag_scale * (mag_offset - magnitude),
            MagnitudeToRadius::Exponential { r_0, n, o } => r_0 * (180.0 * n / fov.value()).ln() * 10.0_f32.powf(-o * magnitude),
        }
    }
}

pub const MAGNITUDE_TO_RADIUS_OPTIONS: usize = 2;

#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, PartialEq)]
pub enum MagnitudeToRadius {
    /// r = mag_scale * (mag_offset - magnitude)
    Linear {
        /// A size multiplier
        mag_scale: f32,
        /// Highest visible magnitude
        mag_offset: f32,
    },
    /// r = r_0 * ln(180*n/fov) * 10^(-o*magnitude)
    Exponential {
        /// A size multiplier
        r_0: f32,
        /// How much does the size change (proportionally) when changing the FOV
        n: f32,
        /// How much does the size change (proportionally) when changing the magnitude
        o: f32,
    },
}

impl MagnitudeToRadius {
    pub fn defaults() -> [Self; MAGNITUDE_TO_RADIUS_OPTIONS] {
        [Self::Linear { mag_scale: 0.5, mag_offset: 6.0 }, Self::Exponential { r_0: 3.2, n: 3.0, o: 0.15 }]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Linear { .. } => "Linear",
            Self::Exponential { .. } => "Exponential",
        }
    }
}

// TODO: Put this into a config file? Maybe could be a part of the theme and this could be a default?
const TEMPERATURE_TO_COLOUR: [(f32, Color32); 391] = [
    (1000.0, Color32::from_rgb(255, 51, 0)),
    (1100.0, Color32::from_rgb(255, 69, 0)),
    (1200.0, Color32::from_rgb(255, 82, 0)),
    (1300.0, Color32::from_rgb(255, 93, 0)),
    (1400.0, Color32::from_rgb(255, 102, 0)),
    (1500.0, Color32::from_rgb(255, 111, 0)),
    (1600.0, Color32::from_rgb(255, 118, 0)),
    (1700.0, Color32::from_rgb(255, 124, 0)),
    (1800.0, Color32::from_rgb(255, 130, 0)),
    (1900.0, Color32::from_rgb(255, 135, 0)),
    (2000.0, Color32::from_rgb(255, 141, 11)),
    (2100.0, Color32::from_rgb(255, 146, 29)),
    (2200.0, Color32::from_rgb(255, 152, 41)),
    (2300.0, Color32::from_rgb(255, 157, 51)),
    (2400.0, Color32::from_rgb(255, 162, 60)),
    (2500.0, Color32::from_rgb(255, 166, 69)),
    (2600.0, Color32::from_rgb(255, 170, 77)),
    (2700.0, Color32::from_rgb(255, 174, 84)),
    (2800.0, Color32::from_rgb(255, 178, 91)),
    (2900.0, Color32::from_rgb(255, 182, 98)),
    (3000.0, Color32::from_rgb(255, 185, 105)),
    (3100.0, Color32::from_rgb(255, 189, 111)),
    (3200.0, Color32::from_rgb(255, 192, 118)),
    (3300.0, Color32::from_rgb(255, 195, 124)),
    (3400.0, Color32::from_rgb(255, 198, 130)),
    (3500.0, Color32::from_rgb(255, 201, 135)),
    (3600.0, Color32::from_rgb(255, 203, 141)),
    (3700.0, Color32::from_rgb(255, 206, 146)),
    (3800.0, Color32::from_rgb(255, 208, 151)),
    (3900.0, Color32::from_rgb(255, 211, 156)),
    (4000.0, Color32::from_rgb(255, 213, 161)),
    (4100.0, Color32::from_rgb(255, 215, 166)),
    (4200.0, Color32::from_rgb(255, 217, 171)),
    (4300.0, Color32::from_rgb(255, 219, 175)),
    (4400.0, Color32::from_rgb(255, 221, 180)),
    (4500.0, Color32::from_rgb(255, 223, 184)),
    (4600.0, Color32::from_rgb(255, 225, 188)),
    (4700.0, Color32::from_rgb(255, 226, 192)),
    (4800.0, Color32::from_rgb(255, 228, 196)),
    (4900.0, Color32::from_rgb(255, 229, 200)),
    (5000.0, Color32::from_rgb(255, 231, 204)),
    (5100.0, Color32::from_rgb(255, 232, 208)),
    (5200.0, Color32::from_rgb(255, 234, 211)),
    (5300.0, Color32::from_rgb(255, 235, 215)),
    (5400.0, Color32::from_rgb(255, 237, 218)),
    (5500.0, Color32::from_rgb(255, 238, 222)),
    (5600.0, Color32::from_rgb(255, 239, 225)),
    (5700.0, Color32::from_rgb(255, 240, 228)),
    (5800.0, Color32::from_rgb(255, 241, 231)),
    (5900.0, Color32::from_rgb(255, 243, 234)),
    (6000.0, Color32::from_rgb(255, 244, 237)),
    (6100.0, Color32::from_rgb(255, 245, 240)),
    (6200.0, Color32::from_rgb(255, 246, 243)),
    (6300.0, Color32::from_rgb(255, 247, 245)),
    (6400.0, Color32::from_rgb(255, 248, 248)),
    (6500.0, Color32::from_rgb(255, 249, 251)),
    (6600.0, Color32::from_rgb(255, 249, 253)),
    (6700.0, Color32::from_rgb(254, 250, 255)),
    (6800.0, Color32::from_rgb(252, 248, 255)),
    (6900.0, Color32::from_rgb(250, 247, 255)),
    (7000.0, Color32::from_rgb(247, 245, 255)),
    (7100.0, Color32::from_rgb(245, 244, 255)),
    (7200.0, Color32::from_rgb(243, 243, 255)),
    (7300.0, Color32::from_rgb(241, 241, 255)),
    (7400.0, Color32::from_rgb(239, 240, 255)),
    (7500.0, Color32::from_rgb(238, 239, 255)),
    (7600.0, Color32::from_rgb(236, 238, 255)),
    (7700.0, Color32::from_rgb(234, 237, 255)),
    (7800.0, Color32::from_rgb(233, 236, 255)),
    (7900.0, Color32::from_rgb(231, 234, 255)),
    (8000.0, Color32::from_rgb(229, 233, 255)),
    (8100.0, Color32::from_rgb(228, 233, 255)),
    (8200.0, Color32::from_rgb(227, 232, 255)),
    (8300.0, Color32::from_rgb(225, 231, 255)),
    (8400.0, Color32::from_rgb(224, 230, 255)),
    (8500.0, Color32::from_rgb(223, 229, 255)),
    (8600.0, Color32::from_rgb(221, 228, 255)),
    (8700.0, Color32::from_rgb(220, 227, 255)),
    (8800.0, Color32::from_rgb(219, 226, 255)),
    (8900.0, Color32::from_rgb(218, 226, 255)),
    (9000.0, Color32::from_rgb(217, 225, 255)),
    (9100.0, Color32::from_rgb(216, 224, 255)),
    (9200.0, Color32::from_rgb(215, 223, 255)),
    (9300.0, Color32::from_rgb(214, 223, 255)),
    (9400.0, Color32::from_rgb(213, 222, 255)),
    (9500.0, Color32::from_rgb(212, 221, 255)),
    (9600.0, Color32::from_rgb(211, 221, 255)),
    (9700.0, Color32::from_rgb(210, 220, 255)),
    (9800.0, Color32::from_rgb(209, 220, 255)),
    (9900.0, Color32::from_rgb(208, 219, 255)),
    (10000.0, Color32::from_rgb(207, 218, 255)),
    (10100.0, Color32::from_rgb(207, 218, 255)),
    (10200.0, Color32::from_rgb(206, 217, 255)),
    (10300.0, Color32::from_rgb(205, 217, 255)),
    (10400.0, Color32::from_rgb(204, 216, 255)),
    (10500.0, Color32::from_rgb(204, 216, 255)),
    (10600.0, Color32::from_rgb(203, 215, 255)),
    (10700.0, Color32::from_rgb(202, 215, 255)),
    (10800.0, Color32::from_rgb(202, 214, 255)),
    (10900.0, Color32::from_rgb(201, 214, 255)),
    (11000.0, Color32::from_rgb(200, 213, 255)),
    (11100.0, Color32::from_rgb(200, 213, 255)),
    (11200.0, Color32::from_rgb(199, 212, 255)),
    (11300.0, Color32::from_rgb(198, 212, 255)),
    (11400.0, Color32::from_rgb(198, 212, 255)),
    (11500.0, Color32::from_rgb(197, 211, 255)),
    (11600.0, Color32::from_rgb(197, 211, 255)),
    (11700.0, Color32::from_rgb(196, 210, 255)),
    (11800.0, Color32::from_rgb(196, 210, 255)),
    (11900.0, Color32::from_rgb(195, 210, 255)),
    (12000.0, Color32::from_rgb(195, 209, 255)),
    (12100.0, Color32::from_rgb(194, 209, 255)),
    (12200.0, Color32::from_rgb(194, 208, 255)),
    (12300.0, Color32::from_rgb(193, 208, 255)),
    (12400.0, Color32::from_rgb(193, 208, 255)),
    (12500.0, Color32::from_rgb(192, 207, 255)),
    (12600.0, Color32::from_rgb(192, 207, 255)),
    (12700.0, Color32::from_rgb(191, 207, 255)),
    (12800.0, Color32::from_rgb(191, 206, 255)),
    (12900.0, Color32::from_rgb(190, 206, 255)),
    (13000.0, Color32::from_rgb(190, 206, 255)),
    (13100.0, Color32::from_rgb(190, 206, 255)),
    (13200.0, Color32::from_rgb(189, 205, 255)),
    (13300.0, Color32::from_rgb(189, 205, 255)),
    (13400.0, Color32::from_rgb(188, 205, 255)),
    (13500.0, Color32::from_rgb(188, 204, 255)),
    (13600.0, Color32::from_rgb(188, 204, 255)),
    (13700.0, Color32::from_rgb(187, 204, 255)),
    (13800.0, Color32::from_rgb(187, 204, 255)),
    (13900.0, Color32::from_rgb(187, 203, 255)),
    (14000.0, Color32::from_rgb(186, 203, 255)),
    (14100.0, Color32::from_rgb(186, 203, 255)),
    (14200.0, Color32::from_rgb(186, 203, 255)),
    (14300.0, Color32::from_rgb(185, 202, 255)),
    (14400.0, Color32::from_rgb(185, 202, 255)),
    (14500.0, Color32::from_rgb(185, 202, 255)),
    (14600.0, Color32::from_rgb(184, 202, 255)),
    (14700.0, Color32::from_rgb(184, 201, 255)),
    (14800.0, Color32::from_rgb(184, 201, 255)),
    (14900.0, Color32::from_rgb(184, 201, 255)),
    (15000.0, Color32::from_rgb(183, 201, 255)),
    (15100.0, Color32::from_rgb(183, 201, 255)),
    (15200.0, Color32::from_rgb(183, 200, 255)),
    (15300.0, Color32::from_rgb(182, 200, 255)),
    (15400.0, Color32::from_rgb(182, 200, 255)),
    (15500.0, Color32::from_rgb(182, 200, 255)),
    (15600.0, Color32::from_rgb(182, 200, 255)),
    (15700.0, Color32::from_rgb(181, 199, 255)),
    (15800.0, Color32::from_rgb(181, 199, 255)),
    (15900.0, Color32::from_rgb(181, 199, 255)),
    (16000.0, Color32::from_rgb(181, 199, 255)),
    (16100.0, Color32::from_rgb(180, 199, 255)),
    (16200.0, Color32::from_rgb(180, 198, 255)),
    (16300.0, Color32::from_rgb(180, 198, 255)),
    (16400.0, Color32::from_rgb(180, 198, 255)),
    (16500.0, Color32::from_rgb(179, 198, 255)),
    (16600.0, Color32::from_rgb(179, 198, 255)),
    (16700.0, Color32::from_rgb(179, 198, 255)),
    (16800.0, Color32::from_rgb(179, 197, 255)),
    (16900.0, Color32::from_rgb(179, 197, 255)),
    (17000.0, Color32::from_rgb(178, 197, 255)),
    (17100.0, Color32::from_rgb(178, 197, 255)),
    (17200.0, Color32::from_rgb(178, 197, 255)),
    (17300.0, Color32::from_rgb(178, 197, 255)),
    (17400.0, Color32::from_rgb(178, 196, 255)),
    (17500.0, Color32::from_rgb(177, 196, 255)),
    (17600.0, Color32::from_rgb(177, 196, 255)),
    (17700.0, Color32::from_rgb(177, 196, 255)),
    (17800.0, Color32::from_rgb(177, 196, 255)),
    (17900.0, Color32::from_rgb(177, 196, 255)),
    (18000.0, Color32::from_rgb(176, 196, 255)),
    (18100.0, Color32::from_rgb(176, 195, 255)),
    (18200.0, Color32::from_rgb(176, 195, 255)),
    (18300.0, Color32::from_rgb(176, 195, 255)),
    (18400.0, Color32::from_rgb(176, 195, 255)),
    (18500.0, Color32::from_rgb(176, 195, 255)),
    (18600.0, Color32::from_rgb(175, 195, 255)),
    (18700.0, Color32::from_rgb(175, 195, 255)),
    (18800.0, Color32::from_rgb(175, 194, 255)),
    (18900.0, Color32::from_rgb(175, 194, 255)),
    (19000.0, Color32::from_rgb(175, 194, 255)),
    (19100.0, Color32::from_rgb(175, 194, 255)),
    (19200.0, Color32::from_rgb(174, 194, 255)),
    (19300.0, Color32::from_rgb(174, 194, 255)),
    (19400.0, Color32::from_rgb(174, 194, 255)),
    (19500.0, Color32::from_rgb(174, 194, 255)),
    (19600.0, Color32::from_rgb(174, 194, 255)),
    (19700.0, Color32::from_rgb(174, 193, 255)),
    (19800.0, Color32::from_rgb(174, 193, 255)),
    (19900.0, Color32::from_rgb(173, 193, 255)),
    (20000.0, Color32::from_rgb(173, 193, 255)),
    (20100.0, Color32::from_rgb(173, 193, 255)),
    (20200.0, Color32::from_rgb(173, 193, 255)),
    (20300.0, Color32::from_rgb(173, 193, 255)),
    (20400.0, Color32::from_rgb(173, 193, 255)),
    (20500.0, Color32::from_rgb(173, 193, 255)),
    (20600.0, Color32::from_rgb(173, 192, 255)),
    (20700.0, Color32::from_rgb(172, 192, 255)),
    (20800.0, Color32::from_rgb(172, 192, 255)),
    (20900.0, Color32::from_rgb(172, 192, 255)),
    (21000.0, Color32::from_rgb(172, 192, 255)),
    (21100.0, Color32::from_rgb(172, 192, 255)),
    (21200.0, Color32::from_rgb(172, 192, 255)),
    (21300.0, Color32::from_rgb(172, 192, 255)),
    (21400.0, Color32::from_rgb(172, 192, 255)),
    (21500.0, Color32::from_rgb(171, 192, 255)),
    (21600.0, Color32::from_rgb(171, 192, 255)),
    (21700.0, Color32::from_rgb(171, 191, 255)),
    (21800.0, Color32::from_rgb(171, 191, 255)),
    (21900.0, Color32::from_rgb(171, 191, 255)),
    (22000.0, Color32::from_rgb(171, 191, 255)),
    (22100.0, Color32::from_rgb(171, 191, 255)),
    (22200.0, Color32::from_rgb(171, 191, 255)),
    (22300.0, Color32::from_rgb(171, 191, 255)),
    (22400.0, Color32::from_rgb(170, 191, 255)),
    (22500.0, Color32::from_rgb(170, 191, 255)),
    (22600.0, Color32::from_rgb(170, 191, 255)),
    (22700.0, Color32::from_rgb(170, 191, 255)),
    (22800.0, Color32::from_rgb(170, 190, 255)),
    (22900.0, Color32::from_rgb(170, 190, 255)),
    (23000.0, Color32::from_rgb(170, 190, 255)),
    (23100.0, Color32::from_rgb(170, 190, 255)),
    (23200.0, Color32::from_rgb(170, 190, 255)),
    (23300.0, Color32::from_rgb(170, 190, 255)),
    (23400.0, Color32::from_rgb(169, 190, 255)),
    (23500.0, Color32::from_rgb(169, 190, 255)),
    (23600.0, Color32::from_rgb(169, 190, 255)),
    (23700.0, Color32::from_rgb(169, 190, 255)),
    (23800.0, Color32::from_rgb(169, 190, 255)),
    (23900.0, Color32::from_rgb(169, 190, 255)),
    (24000.0, Color32::from_rgb(169, 190, 255)),
    (24100.0, Color32::from_rgb(169, 190, 255)),
    (24200.0, Color32::from_rgb(169, 189, 255)),
    (24300.0, Color32::from_rgb(169, 189, 255)),
    (24400.0, Color32::from_rgb(169, 189, 255)),
    (24500.0, Color32::from_rgb(168, 189, 255)),
    (24600.0, Color32::from_rgb(168, 189, 255)),
    (24700.0, Color32::from_rgb(168, 189, 255)),
    (24800.0, Color32::from_rgb(168, 189, 255)),
    (24900.0, Color32::from_rgb(168, 189, 255)),
    (25000.0, Color32::from_rgb(168, 189, 255)),
    (25100.0, Color32::from_rgb(168, 189, 255)),
    (25200.0, Color32::from_rgb(168, 189, 255)),
    (25300.0, Color32::from_rgb(168, 189, 255)),
    (25400.0, Color32::from_rgb(168, 189, 255)),
    (25500.0, Color32::from_rgb(168, 189, 255)),
    (25600.0, Color32::from_rgb(168, 189, 255)),
    (25700.0, Color32::from_rgb(167, 188, 255)),
    (25800.0, Color32::from_rgb(167, 188, 255)),
    (25900.0, Color32::from_rgb(167, 188, 255)),
    (26000.0, Color32::from_rgb(167, 188, 255)),
    (26100.0, Color32::from_rgb(167, 188, 255)),
    (26200.0, Color32::from_rgb(167, 188, 255)),
    (26300.0, Color32::from_rgb(167, 188, 255)),
    (26400.0, Color32::from_rgb(167, 188, 255)),
    (26500.0, Color32::from_rgb(167, 188, 255)),
    (26600.0, Color32::from_rgb(167, 188, 255)),
    (26700.0, Color32::from_rgb(167, 188, 255)),
    (26800.0, Color32::from_rgb(167, 188, 255)),
    (26900.0, Color32::from_rgb(167, 188, 255)),
    (27000.0, Color32::from_rgb(167, 188, 255)),
    (27100.0, Color32::from_rgb(166, 188, 255)),
    (27200.0, Color32::from_rgb(166, 188, 255)),
    (27300.0, Color32::from_rgb(166, 188, 255)),
    (27400.0, Color32::from_rgb(166, 187, 255)),
    (27500.0, Color32::from_rgb(166, 187, 255)),
    (27600.0, Color32::from_rgb(166, 187, 255)),
    (27700.0, Color32::from_rgb(166, 187, 255)),
    (27800.0, Color32::from_rgb(166, 187, 255)),
    (27900.0, Color32::from_rgb(166, 187, 255)),
    (28000.0, Color32::from_rgb(166, 187, 255)),
    (28100.0, Color32::from_rgb(166, 187, 255)),
    (28200.0, Color32::from_rgb(166, 187, 255)),
    (28300.0, Color32::from_rgb(166, 187, 255)),
    (28400.0, Color32::from_rgb(166, 187, 255)),
    (28500.0, Color32::from_rgb(166, 187, 255)),
    (28600.0, Color32::from_rgb(166, 187, 255)),
    (28700.0, Color32::from_rgb(165, 187, 255)),
    (28800.0, Color32::from_rgb(165, 187, 255)),
    (28900.0, Color32::from_rgb(165, 187, 255)),
    (29000.0, Color32::from_rgb(165, 187, 255)),
    (29100.0, Color32::from_rgb(165, 187, 255)),
    (29200.0, Color32::from_rgb(165, 187, 255)),
    (29300.0, Color32::from_rgb(165, 187, 255)),
    (29400.0, Color32::from_rgb(165, 187, 255)),
    (29500.0, Color32::from_rgb(165, 186, 255)),
    (29600.0, Color32::from_rgb(165, 186, 255)),
    (29700.0, Color32::from_rgb(165, 186, 255)),
    (29800.0, Color32::from_rgb(165, 186, 255)),
    (29900.0, Color32::from_rgb(165, 186, 255)),
    (30000.0, Color32::from_rgb(165, 186, 255)),
    (30100.0, Color32::from_rgb(165, 186, 255)),
    (30200.0, Color32::from_rgb(165, 186, 255)),
    (30300.0, Color32::from_rgb(165, 186, 255)),
    (30400.0, Color32::from_rgb(165, 186, 255)),
    (30500.0, Color32::from_rgb(165, 186, 255)),
    (30600.0, Color32::from_rgb(164, 186, 255)),
    (30700.0, Color32::from_rgb(164, 186, 255)),
    (30800.0, Color32::from_rgb(164, 186, 255)),
    (30900.0, Color32::from_rgb(164, 186, 255)),
    (31000.0, Color32::from_rgb(164, 186, 255)),
    (31100.0, Color32::from_rgb(164, 186, 255)),
    (31200.0, Color32::from_rgb(164, 186, 255)),
    (31300.0, Color32::from_rgb(164, 186, 255)),
    (31400.0, Color32::from_rgb(164, 186, 255)),
    (31500.0, Color32::from_rgb(164, 186, 255)),
    (31600.0, Color32::from_rgb(164, 186, 255)),
    (31700.0, Color32::from_rgb(164, 186, 255)),
    (31800.0, Color32::from_rgb(164, 186, 255)),
    (31900.0, Color32::from_rgb(164, 186, 255)),
    (32000.0, Color32::from_rgb(164, 185, 255)),
    (32100.0, Color32::from_rgb(164, 185, 255)),
    (32200.0, Color32::from_rgb(164, 185, 255)),
    (32300.0, Color32::from_rgb(164, 185, 255)),
    (32400.0, Color32::from_rgb(164, 185, 255)),
    (32500.0, Color32::from_rgb(164, 185, 255)),
    (32600.0, Color32::from_rgb(164, 185, 255)),
    (32700.0, Color32::from_rgb(163, 185, 255)),
    (32800.0, Color32::from_rgb(163, 185, 255)),
    (32900.0, Color32::from_rgb(163, 185, 255)),
    (33000.0, Color32::from_rgb(163, 185, 255)),
    (33100.0, Color32::from_rgb(163, 185, 255)),
    (33200.0, Color32::from_rgb(163, 185, 255)),
    (33300.0, Color32::from_rgb(163, 185, 255)),
    (33400.0, Color32::from_rgb(163, 185, 255)),
    (33500.0, Color32::from_rgb(163, 185, 255)),
    (33600.0, Color32::from_rgb(163, 185, 255)),
    (33700.0, Color32::from_rgb(163, 185, 255)),
    (33800.0, Color32::from_rgb(163, 185, 255)),
    (33900.0, Color32::from_rgb(163, 185, 255)),
    (34000.0, Color32::from_rgb(163, 185, 255)),
    (34100.0, Color32::from_rgb(163, 185, 255)),
    (34200.0, Color32::from_rgb(163, 185, 255)),
    (34300.0, Color32::from_rgb(163, 185, 255)),
    (34400.0, Color32::from_rgb(163, 185, 255)),
    (34500.0, Color32::from_rgb(163, 185, 255)),
    (34600.0, Color32::from_rgb(163, 185, 255)),
    (34700.0, Color32::from_rgb(163, 185, 255)),
    (34800.0, Color32::from_rgb(163, 185, 255)),
    (34900.0, Color32::from_rgb(163, 185, 255)),
    (35000.0, Color32::from_rgb(163, 184, 255)),
    (35100.0, Color32::from_rgb(163, 184, 255)),
    (35200.0, Color32::from_rgb(162, 184, 255)),
    (35300.0, Color32::from_rgb(162, 184, 255)),
    (35400.0, Color32::from_rgb(162, 184, 255)),
    (35500.0, Color32::from_rgb(162, 184, 255)),
    (35600.0, Color32::from_rgb(162, 184, 255)),
    (35700.0, Color32::from_rgb(162, 184, 255)),
    (35800.0, Color32::from_rgb(162, 184, 255)),
    (35900.0, Color32::from_rgb(162, 184, 255)),
    (36000.0, Color32::from_rgb(162, 184, 255)),
    (36100.0, Color32::from_rgb(162, 184, 255)),
    (36200.0, Color32::from_rgb(162, 184, 255)),
    (36300.0, Color32::from_rgb(162, 184, 255)),
    (36400.0, Color32::from_rgb(162, 184, 255)),
    (36500.0, Color32::from_rgb(162, 184, 255)),
    (36600.0, Color32::from_rgb(162, 184, 255)),
    (36700.0, Color32::from_rgb(162, 184, 255)),
    (36800.0, Color32::from_rgb(162, 184, 255)),
    (36900.0, Color32::from_rgb(162, 184, 255)),
    (37000.0, Color32::from_rgb(162, 184, 255)),
    (37100.0, Color32::from_rgb(162, 184, 255)),
    (37200.0, Color32::from_rgb(162, 184, 255)),
    (37300.0, Color32::from_rgb(162, 184, 255)),
    (37400.0, Color32::from_rgb(162, 184, 255)),
    (37500.0, Color32::from_rgb(162, 184, 255)),
    (37600.0, Color32::from_rgb(162, 184, 255)),
    (37700.0, Color32::from_rgb(162, 184, 255)),
    (37800.0, Color32::from_rgb(162, 184, 255)),
    (37900.0, Color32::from_rgb(162, 184, 255)),
    (38000.0, Color32::from_rgb(162, 184, 255)),
    (38100.0, Color32::from_rgb(162, 184, 255)),
    (38200.0, Color32::from_rgb(162, 184, 255)),
    (38300.0, Color32::from_rgb(161, 184, 255)),
    (38400.0, Color32::from_rgb(161, 184, 255)),
    (38500.0, Color32::from_rgb(161, 184, 255)),
    (38600.0, Color32::from_rgb(161, 183, 255)),
    (38700.0, Color32::from_rgb(161, 183, 255)),
    (38800.0, Color32::from_rgb(161, 183, 255)),
    (38900.0, Color32::from_rgb(161, 183, 255)),
    (39000.0, Color32::from_rgb(161, 183, 255)),
    (39100.0, Color32::from_rgb(161, 183, 255)),
    (39200.0, Color32::from_rgb(161, 183, 255)),
    (39300.0, Color32::from_rgb(161, 183, 255)),
    (39400.0, Color32::from_rgb(161, 183, 255)),
    (39500.0, Color32::from_rgb(161, 183, 255)),
    (39600.0, Color32::from_rgb(161, 183, 255)),
    (39700.0, Color32::from_rgb(161, 183, 255)),
    (39800.0, Color32::from_rgb(161, 183, 255)),
    (39900.0, Color32::from_rgb(161, 183, 255)),
    (40000.0, Color32::from_rgb(161, 183, 255)),
];

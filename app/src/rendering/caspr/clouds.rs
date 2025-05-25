use angle::Angle;
use noise::{MultiFractal, NoiseFn};

// The physical model is as follows:
// Each layer of clouds absorbs a set fraction of light from each star, so the received flux is F ~ F_0 * exp(-number of layers).
// However, the change in magnitude is m - m_0 ~ -log(F / F_0) = -log(exp(-number of layers)) = number of layers.
// So the decrease in magnitude is linear in the thickness of the cloud (roughly).
pub fn apply_dimming(stars: &mut std::collections::HashMap<String, Vec<super::stars::Star>>, settings: &CloudSettings) {
    let seed = (chrono::Utc::now().timestamp().abs() % (u32::MAX as i64)) as u32;
    let cloud_generator: noise::Billow<noise::SuperSimplex> = noise::Billow::new(seed).set_octaves(settings.iterations);
    let mut generated_decreases = std::collections::HashMap::<[u32; 2], f32>::new();
    let mut decreases = Vec::new();
    for star_set in stars.values_mut() {
        for star in star_set {
            let coordinates = spherical_geometry::SphericalPoint::ra_dec_to_cartesian(*star.ra.to_rad().as_value(), *star.dec.to_rad().as_value());
            let decrease = cloud_generator.get([coordinates.x as f64, coordinates.y as f64, coordinates.z as f64]) as f32;
            generated_decreases.insert([star.ra.as_value().to_bits(), star.dec.as_value().to_bits()], decrease);
            decreases.push(decrease);
        }
    }
    // Puts NaNs last, but that is not an issue
    // Source: https://users.rust-lang.org/t/sorting-a-vec-of-f32-without-ever-panic/37540/2 (https://web.archive.org/web/20250404080805/https://users.rust-lang.org/t/sorting-a-vec-of-f32-without-ever-panic/37540)
    decreases.sort_by(|a, b| match a.partial_cmp(b) {
        Some(ord) => ord,
        None => match (a.is_nan(), b.is_nan()) {
            (true, true) => std::cmp::Ordering::Equal,
            (true, _) => std::cmp::Ordering::Greater,
            (_, true) => std::cmp::Ordering::Less,
            (_, _) => std::cmp::Ordering::Equal, // should never happen
        },
    });
    let decrease_offset = decreases[(((decreases.len() - 1) as f32) * (1.0 - settings.coverage)).floor() as usize]; // This offset ensures that the chosen (via the coverage setting) part of the sky is covered
    let multi = settings.thickness / (decreases[decreases.len() - 1] - decrease_offset); // This multiplier ensures that the maximum decrease is the one chosen via the `thickness` setting

    for star_set in stars.values_mut() {
        for star in star_set {
            if let Some(decrease_raw) = generated_decreases.get(&[star.ra.as_value().to_bits(), star.dec.as_value().to_bits()]) {
                let decrease = (multi * (decrease_raw - decrease_offset)).max(0.0);
                star.magnitude_offset = decrease;
            }
        }
    }
}

pub fn disable(stars: &mut std::collections::HashMap<String, Vec<super::stars::Star>>) {
    for star_set in stars.values_mut() {
        for star in star_set {
            star.magnitude_offset = 0.0;
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy)]
pub struct CloudSettings {
    pub coverage: f32,
    pub thickness: f32,
    pub iterations: usize,
    pub enabled: bool,
    pub recalculate_on_change: bool,
}

impl Default for CloudSettings {
    fn default() -> Self {
        Self {
            coverage: 0.5,
            thickness: 4.0,
            iterations: 16,
            enabled: false,
            recalculate_on_change: false,
        }
    }
}

impl CloudSettings {
    pub fn clamp(&mut self) {
        self.coverage = self.coverage.clamp(0.0, 1.0);
        if self.thickness < 0.0 {
            self.thickness = 0.0;
        }
        if self.iterations < 1 {
            self.iterations = 1;
        }
    }
}

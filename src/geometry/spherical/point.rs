use std::f32::consts::PI;
use nalgebra::Vector3;

/// A point on a unit sphere, given by its right ascension and declination
///
/// The right ascension is measured "from north to west" - the same way it goes if you look at the sky in the northern hemisphere. This means that at RA=0 you start at the x-axis and then with increasing RA you go towards negative values of y.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SphericalPoint {
    pub ra: f32,
    pub dec: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl SphericalPoint {
    pub fn new(ra: f32, dec: f32) -> Self {
        let cartesian = Self::ra_dec_to_cartesian(ra, dec);
        Self {
            ra,
            dec,
            x: cartesian.x,
            y: cartesian.y,
            z: cartesian.z
        }
    }

    pub fn from_cartesian_vector3(vector: Vector3<f32>) -> Self {
        let (dec, ra) = crate::geometry::cartesian_to_spherical(vector);
        let dec = PI / 2.0 - vector.normalize().z.acos();
        let mut ra = vector.y.atan2(vector.x);
        if ra < 0.0 {
            ra += 2.0 * PI;
        }
        Self::new(ra, dec)
    }

    pub fn from_cartesian(x: f32, y: f32, z: f32) -> Self {
        Self::from_cartesian_vector3(Vector3::new(x, y, z))
    }

    pub fn cartesian(&self) -> Vector3<f32> {
        let x = self.dec.cos() * self.ra.cos();
        let y = self.dec.cos() * self.ra.sin();
        let z = self.dec.sin();
        Vector3::new(x, y, z)
    }

    pub fn ra_dec_to_cartesian(ra: f32, dec: f32) -> Vector3<f32> {
        let x = dec.cos() * ra.cos();
        let y = -dec.cos() * ra.sin();
        let z = dec.sin();
        Vector3::new(x, y, z)
    }

    pub fn approximately_equals(&self, other: &Self, tolerance: f32) -> bool {
        let cos_angle = self.cartesian().dot(&other.cartesian());
        // For roughly equal vectors, the angle between them is ~0 -> cos(angle) ~ 1 -> (1 - cos(angle)) ~ 0
        // cos(x) ~ 1-x^2/2 for small x -> 1 - cos(x) ~ x^2/2 -> tolerance needs to be squared if it is supposed to serve as a tolerance on "how many radians apart can the vectors point to still consider them as equal"
        (1.0 - cos_angle) < tolerance.powi(2)
    }

    /// Calculates -1/tan(distance between points)
    ///
    /// Useful when sorting points based on distance without needing to know the actual distance as it avoid inverse trigonometric functions. -1/tan(x) is increasing for 0 < x < pi, which is (more than) the needed range
    pub fn minus_cotan_distance(&self, other: &Self) -> f32 {
        let angle_sin = self.cartesian().cross(&other.cartesian()).magnitude();
        let angle_cos = self.cartesian().dot(&other.cartesian());
        -angle_cos/angle_sin
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn polar_to_cartesian() {
        let max_delta = 10e-6;

        let north_pole = SphericalPoint::new(PI / 3.0, PI / 2.0); // The value of ra is irrelevant here
        let north_pole_cartesian = north_pole.cartesian();
        let north_pole_correct = Vector3::new(0.0, 0.0, 1.0);
        assert!((north_pole_cartesian.x - north_pole_correct.x).abs() < max_delta && (north_pole_cartesian.y - north_pole_correct.y).abs() < max_delta && (north_pole_cartesian.z - north_pole_correct.z).abs() < max_delta);

        let south_pole = SphericalPoint::new(PI / 5.0, -PI / 2.0); // The value of ra is irrelevant here
        let south_pole_cartesian = south_pole.cartesian();
        let south_pole_correct = Vector3::new(0.0, 0.0, -1.0);
        assert!((south_pole_cartesian.x - south_pole_correct.x).abs() < max_delta && (south_pole_cartesian.y - south_pole_correct.y).abs() < max_delta && (south_pole_cartesian.z - south_pole_correct.z).abs() < max_delta);

        let north = SphericalPoint::new(0.0, 0.0);
        let north_cartesian = north.cartesian();
        let north_correct = Vector3::new(1.0, 0.0, 0.0);
        assert!((north_cartesian.x - north_correct.x).abs() < max_delta && (north_cartesian.y - north_correct.y).abs() < max_delta && (north_cartesian.z - north_correct.z).abs() < max_delta);

        let north2 = SphericalPoint::new(2.0 * PI, 0.0);
        let north2_cartesian = north2.cartesian();
        let north2_correct = Vector3::new(1.0, 0.0, 0.0);
        assert!((north2_cartesian.x - north2_correct.x).abs() < max_delta && (north2_cartesian.y - north2_correct.y).abs() < max_delta && (north2_cartesian.z - north2_correct.z).abs() < max_delta);

        let east = SphericalPoint::new(PI / 2.0, 0.0);
        let east_cartesian = east.cartesian();
        let east_correct = Vector3::new(0.0, 1.0, 0.0);
        assert!((east_cartesian.x - east_correct.x).abs() < max_delta && (east_cartesian.y - east_correct.y).abs() < max_delta && (east_cartesian.z - east_correct.z).abs() < max_delta);

        let south = SphericalPoint::new(PI, 0.0);
        let south_cartesian = south.cartesian();
        let south_correct = Vector3::new(-1.0, 0.0, 0.0);
        assert!((south_cartesian.x - south_correct.x).abs() < max_delta && (south_cartesian.y - south_correct.y).abs() < max_delta && (south_cartesian.z - south_correct.z).abs() < max_delta);

        let west = SphericalPoint::new(-PI / 2.0, 0.0);
        let west_cartesian = west.cartesian();
        let west_correct = Vector3::new(0.0, -1.0, 0.0);
        assert!((west_cartesian.x - west_correct.x).abs() < max_delta && (west_cartesian.y - west_correct.y).abs() < max_delta && (west_cartesian.z - west_correct.z).abs() < max_delta);

        let west2 = SphericalPoint::new(3.0 / 2.0 * PI, 0.0);
        let west2_cartesian = west2.cartesian();
        let west2_correct = Vector3::new(0.0, -1.0, 0.0);
        assert!((west2_cartesian.x - west2_correct.x).abs() < max_delta && (west2_cartesian.y - west2_correct.y).abs() < max_delta && (west2_cartesian.z - west2_correct.z).abs() < max_delta);
    }

    #[test]
    fn approximately_equal() {
        let tolerance = 10e-4;
        for i in 1..49 {
            for j in 1..49 {
                let ra = 2.0 * PI / 50.0 * (i as f32);
                let dec = PI / 50.0 * (j as f32) - PI / 2.0; // Only ranges from -PI/2 to PI/2
                let point = SphericalPoint::new(ra, dec);
                let point_2 = SphericalPoint::new(ra + tolerance / 500.0 * (i as f32), dec);
                assert!(point.approximately_equals(&point_2, tolerance));
            }
        }

        for i in 1..49 {
            for j in 1..49 {
                let ra = 2.0 * PI / 50.0 * (i as f32);
                let dec = PI / 50.0 * (j as f32) - PI / 2.0; // Only ranges from -PI/2 to PI/2
                let point = SphericalPoint::new(ra, dec);
                let point_2 = SphericalPoint::new(ra + tolerance * 100.0 * (i as f32 + 1.0), dec); // Needs to be fairly off or the error gets lost in the conversions (the tolerance here is too small compared to the input angles)
                dbg!(point);
                dbg!(point_2);
                assert!(!point.approximately_equals(&point_2, tolerance));
            }
        }
    }

    #[test]
    fn polar_to_cartesian_and_back() {
        let tolerance = 10e-4;
        for i in 1..49 {
            for j in 1..49 {
                let ra = 2.0 * PI / 50.0 * (i as f32);
                let dec = PI / 50.0 * (j as f32) - PI / 2.0; // Only ranges from -PI/2 to PI/2
                let point = SphericalPoint::new(ra, dec);
                let cartesian = point.cartesian();
                let point_2 = SphericalPoint::from_cartesian_vector3(cartesian);

                #[cfg(test)]
                dbg!(point);
                #[cfg(test)]
                dbg!(point_2);

                assert!((point.ra - point_2.ra).abs() < tolerance && (point.dec - point_2.dec).abs() < tolerance);
            }
        }
    }
}

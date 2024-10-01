use nalgebra::Vector3;
use crate::geometry::spherical::point::SphericalPoint;
use crate::geometry::spherical::{SphericalError, VEC_LEN_IS_ZERO};
use crate::geometry::spherical::great_circle_arc::GreatCircleArc;

/// A great circle on a unit sphere, given by two points on it
#[derive(Clone, Copy)]
pub struct GreatCircle {
    start: SphericalPoint,
    end: SphericalPoint,
    normal: Vector3<f32>
}

impl GreatCircle {
    /// Creates a new great circle passing through the two points provided
    ///
    /// If the points are essentially equal or essentially antipodal, returns `SphericalError::AntipodalOrTooClosePoints` as in the case of identical or antipodal points the great circle is not uniquely defined
    pub fn new(point1: SphericalPoint, point2: SphericalPoint) -> Result<Self, SphericalError> {
        if point1.cartesian().cross(&point2.cartesian()).magnitude_squared() < VEC_LEN_IS_ZERO.powi(2) {
            return Err(SphericalError::AntipodalOrTooClosePoints);
        }
        Ok(Self {
            start: point1,
            end: point2,
            normal: point1.cartesian().cross(&point2.cartesian()).normalize()
        })
    }

    /// Creates a great circle from an arc
    pub fn from_arc(arc: &GreatCircleArc) -> Self {
        Self {
            start: arc.start(),
            end: arc.end(),
            normal: arc.normal()
        }
    }

    pub fn start(&self) -> SphericalPoint {
        self.start
    }

    pub fn end(&self) -> SphericalPoint {
        self.end
    }

    pub fn normal(&self) -> Vector3<f32> {
        self.normal
    }

    pub fn intersect_great_circle(&self, other: &GreatCircle) -> Result<[SphericalPoint; 2], SphericalError> {
        let normal1 = self.start.cartesian().cross(&self.end.cartesian());
        let normal2 = other.start.cartesian().cross(&other.end.cartesian());

        let res = normal1.cross(&normal2);
        if res.magnitude_squared() < VEC_LEN_IS_ZERO.powi(2) {
            return Err(SphericalError::IdenticalGreatCircles);
        }
        let res_norm = res.normalize();
        Ok(
            [SphericalPoint::from_cartesian_vector3(res_norm), SphericalPoint::from_cartesian_vector3(-res_norm)]
        )
    }

    pub fn contains_point(&self, point: &SphericalPoint) -> bool {
        let tolerance: f32 = 10e-6;
        let normal = self.start.cartesian().cross(&self.end.cartesian());
        normal.dot(&point.cartesian()) < tolerance
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use super::*;

    #[test]
    fn gecaa_2020_theory_07() {
        let delta = 10e-2;

        let start_1 = SphericalPoint::new(-PI/2.0, 0.0);
        let end_1 = SphericalPoint::new(0.0, 15.0 * PI / 180.0);
        let circle_1 = GreatCircle::new(start_1, end_1).expect("The points are fairly far away");

        let start_2 = SphericalPoint::new(-210.0 * PI / 180.0, 23.5 * PI / 180.0); // Switch RA direction as the question measures azimuth from north to east
        let end_2 = SphericalPoint::new(-255.0 * PI / 180.0, 75.0 * PI / 180.0); // Switch RA direction as the question measures azimuth from north to east
        let circle_2 = GreatCircle::new(start_2, end_2).expect("The points are fairly far away");

        let intersections = circle_1.intersect_great_circle(&circle_2).expect("The paths are not parallel");

        #[cfg(test)]
        dbg!(intersections);

        let [(ra_1, dec_1), (ra_2, dec_2)] = if intersections[0].ra < intersections[1].ra {
            [(intersections[1].ra, intersections[1].dec), (intersections[0].ra, intersections[0].dec)]
        } else {
            [(intersections[0].ra, intersections[0].dec), (intersections[1].ra, intersections[1].dec)]
        };

        let (ra_1_corr, dec_1_corr) = ((360.0 - 21.94) * PI / 180.0, 13.96 * PI / 180.0); // Once again switch RA direction as the question measures azimuth from north to east
        let (ra_2_corr, dec_2_corr) = ((360.0 - 201.94) * PI / 180.0, -13.96 * PI / 180.0); // Once again switch RA direction as the question measures azimuth from north to east

        assert!((ra_1_corr - ra_1).abs() < delta && (dec_1_corr - dec_1).abs() < delta);
        assert!((ra_2_corr - ra_2).abs() < delta && (dec_2_corr - dec_2).abs() < delta);
    }

    #[test]
    fn contains_point() {
        let equator = GreatCircle::new(SphericalPoint::new(0.0, 0.0), SphericalPoint::new(PI / 2.0, 0.0)).expect("The points are far enough");
        let point_on_equator = SphericalPoint::new(PI / 3.0, 0.0);
        let point_outside_equator = SphericalPoint::new(0.0, 10e-5);

        assert!(equator.contains_point(&point_on_equator));
        assert!(!equator.contains_point(&point_outside_equator));

        let circle_2 = GreatCircle::new(SphericalPoint::new(PI / 5.0, PI / 4.0), SphericalPoint::new(PI / 5.0, -PI / 4.0)).expect("The points are far enough");
        assert!(circle_2.contains_point(&SphericalPoint::new(PI / 5.0,  -PI / 7.0)));
    }
}
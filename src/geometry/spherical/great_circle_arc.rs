use nalgebra::Vector3;
use crate::geometry::spherical::point::SphericalPoint;
use crate::geometry::spherical::{SphericalError, VEC_LEN_IS_ZERO};
use crate::geometry::spherical::great_circle::GreatCircle;

/// A great circle on a unit sphere, given by two points on it
#[derive(Clone, Copy)]
pub struct GreatCircleArc {
    start: SphericalPoint,
    end: SphericalPoint,
    normal: Vector3<f32>
}

impl GreatCircleArc {
    /// Creates a new great circle arc passing through the two points provided, taking the shorter of the two possible paths
    ///
    /// If the points are essentially equal or essentially antipodal, returns `SphericalError::AntipodalOrTooClosePoints` as in the case of identical or antipodal points the great circle (and therefore also the arc) is not uniquely defined
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

    pub fn start(&self) -> SphericalPoint {
        self.start
    }

    pub fn end(&self) -> SphericalPoint {
        self.end
    }

    pub fn normal(&self) -> Vector3<f32> {
        self.normal
    }

    pub fn intersect_great_circle(&self, other: &GreatCircle) -> Result<Vec<SphericalPoint>, SphericalError> {
        let normal1 = self.start.cartesian().cross(&self.end.cartesian());
        let normal2 = other.start().cartesian().cross(&other.end().cartesian());

        let res = normal1.cross(&normal2);
        if res.magnitude_squared() < VEC_LEN_IS_ZERO.powi(2) {
            return Err(SphericalError::IdenticalGreatCircles);
        }
        let res_norm = res.normalize();
        let point_1 = SphericalPoint::from_cartesian_vector3(res_norm);
        let point_2 = SphericalPoint::from_cartesian_vector3(-res_norm);

        #[cfg(test)]
        dbg!(point_1);
        #[cfg(test)]
        dbg!(point_2);

        let mut intersections = Vec::new();
        if self.contains_point(&point_1) {
            intersections.push(point_1);
        }
        if self.contains_point(&point_2) {
            intersections.push(point_2);
        }
        Ok(
            intersections
        )
    }

    /// Returns the intersections of the arc with the great circle, clamped to the arc. So if there are no intersections, the endpoint closest to the potential intersections (of the great circle and the arc extended into a great circle) is returned.
    pub fn intersect_great_circle_clamped(&self, other: &GreatCircle) -> Result<Vec<SphericalPoint>, SphericalError> {
        let normal1 = self.start.cartesian().cross(&self.end.cartesian());
        let normal2 = other.start().cartesian().cross(&other.end().cartesian());

        let res = normal1.cross(&normal2);
        if res.magnitude_squared() < VEC_LEN_IS_ZERO.powi(2) {
            return Err(SphericalError::IdenticalGreatCircles);
        }
        let res_norm = res.normalize();
        let point_1 = SphericalPoint::from_cartesian_vector3(res_norm);
        let point_2 = SphericalPoint::from_cartesian_vector3(-res_norm);

        #[cfg(test)]
        dbg!(point_1);
        #[cfg(test)]
        dbg!(point_2);

        let mut intersections = Vec::new();
        if self.contains_point(&point_1) {
            intersections.push(point_1);
        }
        if self.contains_point(&point_2) {
            intersections.push(point_2);
        }
        if intersections.is_empty() {
            let start_1_distance = self.start.minus_cotan_distance(&point_1);
            let start_2_distance = self.start.minus_cotan_distance(&point_2);
            let start_distance = start_1_distance.min(start_2_distance);
            let end_1_distance = self.end.minus_cotan_distance(&point_1);
            if end_1_distance < start_distance {
                intersections.push(self.end);
            } else {
                let end_2_distance = self.end.minus_cotan_distance(&point_2);
                if end_2_distance < start_distance {
                    intersections.push(self.end);
                } else {
                    intersections.push(self.start);
                }
            }
        }
        Ok(
            intersections
        )
    }

    pub fn contains_point(&self, point: &SphericalPoint) -> bool {
        let tolerance: f32 = 10e-5;
        let great_circle = GreatCircle::from_arc(&self);
        if !great_circle.contains_point(point) {
            return false;
        }
        // If the point is approximately equal to either of the ends, it obviously is on the arc
        if self.start.approximately_equals(&point, tolerance) || self.end.approximately_equals(&point, tolerance) {
            return true;
        }
        // If angle AOP + angle POB = angle AOB, then the point is on the arc -> Check for cos(AOP + POB) = cos(AOP) * cos(POB) - sin(AOP) * sin(POB)
        // This way we avoid relatively costly inverse trigonometric functions
        let cos_aob = self.start.cartesian().dot(&self.end.cartesian());
        let cos_aop = self.start.cartesian().dot(&point.cartesian());
        let cos_pob = point.cartesian().dot(&self.end.cartesian());

        #[cfg(test)]
        dbg!(cos_aob, cos_aop, cos_pob);

        // If either of the cosines is smaller than cos(angle AOB), then the angular distance from one of the endings of the arc to the point is greater than the length of the arc -> the point must be outside
        // This avoids the issue of points that are opposite the arc
        if cos_aob - cos_aop > tolerance || cos_aob - cos_pob > tolerance {
            return false;
        }
        let sin_aop = self.start.cartesian().cross(&point.cartesian()).magnitude();
        let sin_pob = point.cartesian().cross(&self.end.cartesian()).magnitude();
        let cos_aob_calc = cos_aop * cos_pob - sin_aop * sin_pob;

        #[cfg(test)]
        dbg!(cos_aob_calc);

        (cos_aob - cos_aob_calc).abs() < tolerance
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use super::*;

    #[test]
    fn contains_point() {
        let equator_north_to_west = GreatCircleArc::new(SphericalPoint::new(0.0, 0.0), SphericalPoint::new(PI / 2.0, 0.0)).expect("The points are far enough");
        let north = SphericalPoint::new(0.0, 0.0);
        let northwest = SphericalPoint::new(PI / 4.0, 0.0);
        let west = SphericalPoint::new(PI / 2.0, 0.0);
        let southeast = SphericalPoint::new(-3.0 * PI / 4.0, 0.0);
        let outside_opposite = SphericalPoint::new(-3.0 * PI / 4.0 - PI / 10.0, 0.0);
        let outside_in_plane = SphericalPoint::new(-PI / 4.0, 0.0);
        let outside_above = SphericalPoint::new(PI / 4.0, PI / 7.0);
        let outside_total = SphericalPoint::new(PI, -PI / 3.0);

        assert!(equator_north_to_west.contains_point(&north));
        assert!(equator_north_to_west.contains_point(&northwest));
        assert!(equator_north_to_west.contains_point(&west));
        assert!(!equator_north_to_west.contains_point(&outside_opposite));
        assert!(!equator_north_to_west.contains_point(&southeast));
        assert!(!equator_north_to_west.contains_point(&outside_in_plane));
        assert!(!equator_north_to_west.contains_point(&outside_above));
        assert!(!equator_north_to_west.contains_point(&outside_total));

        let tolerance = 10e-5;
        for i in 0..360 {
            let angle = 2.0 * PI / 360.0 * (i as f32);
            let point = SphericalPoint::new(angle, 0.0);
            dbg!(point);
            assert_eq!(equator_north_to_west.contains_point(&point), angle < PI / 2.0 || (PI / 2.0 - angle).abs() < tolerance);
        }
    }

    #[test]
    fn intersect_great_circle() {
        let tolerance = 10e-4;
        let arc_1 = GreatCircleArc::new(SphericalPoint::new(0.0, PI / 4.0), SphericalPoint::new(0.0, -PI / 4.0)).expect("The points are far enough");
        let circle_1 = GreatCircle::new(SphericalPoint::new(PI / 4.0, 0.0), SphericalPoint::new(-PI / 4.0, 0.0)).expect("The points are far enough");
        let intersections_1 = arc_1.intersect_great_circle(&circle_1).expect("The circles are not parallel");
        assert_eq!(intersections_1.len(), 1);
        assert!(intersections_1[0].approximately_equals(&SphericalPoint::new(0.0, 0.0), tolerance));

        let arc_2 = GreatCircleArc::new(SphericalPoint::new(0.0, PI / 6.0), SphericalPoint::new(0.0, PI / 4.0)).expect("The points are far enough");
        let circle_2 = GreatCircle::new(SphericalPoint::new(PI / 4.0, 0.0), SphericalPoint::new(-PI / 4.0, 0.0)).expect("The points are far enough");
        let intersections_2 = arc_2.intersect_great_circle(&circle_2).expect("The circles are not parallel");
        assert!(intersections_2.is_empty());

        let arc_3 = GreatCircleArc::new(SphericalPoint::new(0.0, 0.0), SphericalPoint::new(PI / 2.0, PI / 4.0)).expect("The points are far enough");
        let circle_3 = GreatCircle::new(SphericalPoint::new(PI / 4.0, 0.0), SphericalPoint::new(-PI / 4.0, 0.0)).expect("The points are far enough");
        let intersections_3 = arc_3.intersect_great_circle(&circle_3).expect("The circles are not parallel");
        assert_eq!(intersections_3.len(), 1);
        assert!(intersections_3[0].approximately_equals(&SphericalPoint::new(0.0, 0.0), tolerance));

        let arc_4 = GreatCircleArc::new(SphericalPoint::new(PI / 5.0,  -PI / 7.0), SphericalPoint::new(PI / 2.0, PI / 4.0)).expect("The points are far enough");
        let circle_4 = GreatCircle::new(SphericalPoint::new(PI / 5.0, PI / 4.0), SphericalPoint::new(PI / 5.0, -PI / 4.0)).expect("The points are far enough");
        let intersections_4 = arc_4.intersect_great_circle(&circle_4).expect("The circles are not parallel");
        assert_eq!(intersections_4.len(), 1);
        assert!(intersections_4[0].approximately_equals(&SphericalPoint::new(PI / 5.0,  -PI / 7.0), tolerance));
    }

    #[test]
    fn intersect_great_circle_clamped() {
        let tolerance = 10e-4;
        let arc_1 = GreatCircleArc::new(SphericalPoint::new(0.0, PI / 4.0), SphericalPoint::new(0.0, -PI / 4.0)).expect("The points are far enough");
        let circle_1 = GreatCircle::new(SphericalPoint::new(PI / 4.0, 0.0), SphericalPoint::new(-PI / 4.0, 0.0)).expect("The points are far enough");
        let intersections_1 = arc_1.intersect_great_circle_clamped(&circle_1).expect("The circles are not parallel");
        assert_eq!(intersections_1.len(), 1);
        assert!(intersections_1[0].approximately_equals(&SphericalPoint::new(0.0, 0.0), tolerance));

        let arc_2 = GreatCircleArc::new(SphericalPoint::new(0.0, PI / 6.0), SphericalPoint::new(0.0, PI / 4.0)).expect("The points are far enough");
        let circle_2 = GreatCircle::new(SphericalPoint::new(PI / 4.0, 0.0), SphericalPoint::new(-PI / 4.0, 0.0)).expect("The points are far enough");
        let intersections_2 = arc_2.intersect_great_circle_clamped(&circle_2).expect("The circles are not parallel");
        assert_eq!(intersections_2.len(), 1);
        assert!(intersections_2[0].approximately_equals(&SphericalPoint::new(0.0, PI / 6.0), tolerance));

        let arc_3 = GreatCircleArc::new(SphericalPoint::new(0.0, 0.0), SphericalPoint::new(PI / 2.0, PI / 4.0)).expect("The points are far enough");
        let circle_3 = GreatCircle::new(SphericalPoint::new(PI / 4.0, 0.0), SphericalPoint::new(-PI / 4.0, 0.0)).expect("The points are far enough");
        let intersections_3 = arc_3.intersect_great_circle_clamped(&circle_3).expect("The circles are not parallel");
        assert_eq!(intersections_3.len(), 1);
        assert!(intersections_3[0].approximately_equals(&SphericalPoint::new(0.0, 0.0), tolerance));

        let arc_4 = GreatCircleArc::new(SphericalPoint::new(PI / 5.0,  -PI / 7.0), SphericalPoint::new(PI / 2.0, PI / 4.0)).expect("The points are far enough");
        let circle_4 = GreatCircle::new(SphericalPoint::new(PI / 5.0, PI / 4.0), SphericalPoint::new(PI / 5.0, -PI / 4.0)).expect("The points are far enough");
        let intersections_4 = arc_4.intersect_great_circle_clamped(&circle_4).expect("The circles are not parallel");
        assert_eq!(intersections_4.len(), 1);
        assert!(intersections_4[0].approximately_equals(&SphericalPoint::new(PI / 5.0,  -PI / 7.0), tolerance));

        let arc_5 = GreatCircleArc::new(SphericalPoint::new(PI / 5.0, PI / 7.0), SphericalPoint::new(PI / 2.0, PI / 6.0)).expect("The points are far enough");
        let circle_5 = GreatCircle::new(SphericalPoint::new(PI / 2.0 + PI / 5.0, 0.0), SphericalPoint::new(PI / 2.0 + PI / 5.0, PI / 4.0)).expect("The points are far enough");
        let intersections_5 = arc_5.intersect_great_circle_clamped(&circle_5).expect("The circles are not parallel");
        assert_eq!(intersections_5.len(), 1);
        assert!(intersections_5[0].approximately_equals(&SphericalPoint::new(PI / 2.0, PI / 6.0), tolerance));
    }
}
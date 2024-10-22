use crate::renderer::CellestialSphere;
use angle::Angle;
use eframe::egui;
use egui::Pos2;
use nalgebra::{Matrix3, Vector2, Vector3};
use rand::{rngs::ThreadRng, Rng};
use spherical_geometry::SphericalPoint;
use std::f32::consts::PI;

pub mod intersections;

// const POLYGONLIMIT: f32 = 180.0;
const VIEWPORT_OFFSET: f32 = 10.0;

#[derive(Clone, Copy)]
pub struct LineSegment {
    pub start: Pos2,
    pub end: Pos2,
}

impl LineSegment {
    pub fn new(start: Pos2, end: Pos2) -> Self {
        Self { start, end }
    }
}

impl From<[[f32; 2]; 2]> for LineSegment {
    fn from(value: [[f32; 2]; 2]) -> Self {
        Self {
            start: Pos2::from(value[0]),
            end: Pos2::from(value[1]),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Rectangle {
    pub top_left: Pos2,
    pub top_right: Pos2,
    pub bottom_left: Pos2,
    pub bottom_right: Pos2,
}

impl Rectangle {
    pub fn sides(&self) -> [LineSegment; 4] {
        [
            LineSegment::new(self.top_left, self.bottom_left),
            LineSegment::new(self.bottom_left, self.bottom_right),
            LineSegment::new(self.bottom_right, self.top_right),
            LineSegment::new(self.top_right, self.top_left),
        ]
    }
}

impl From<egui::Rect> for Rectangle {
    fn from(value: egui::Rect) -> Self {
        let mut left_x = value.min.x;
        let mut right_x = value.max.x;
        if left_x > right_x {
            std::mem::swap(&mut left_x, &mut right_x);
        }
        let mut top_y = value.min.y;
        let mut bottom_y = value.max.y;
        if bottom_y > top_y {
            std::mem::swap(&mut top_y, &mut bottom_y);
        }
        Self {
            top_left: Pos2::new(left_x, top_y),
            top_right: Pos2::new(right_x, top_y),
            bottom_left: Pos2::new(left_x, bottom_y),
            bottom_right: Pos2::new(right_x, bottom_y),
        }
    }
}

pub fn is_in_rect<T: PartialOrd>(point: [T; 2], rect: [[T; 2]; 2]) -> bool {
    let [upper_left, bottom_right] = rect;
    point[0] >= upper_left[0] && point[0] <= bottom_right[0] && point[1] >= upper_left[1] && point[1] <= bottom_right[1]
}

pub fn get_point_vector(ra: angle::Deg<f32>, dec: angle::Deg<f32>, rotation_matrix: &Matrix3<f32>) -> Vector3<f32> {
    let (ra_s, ra_c) = (-(ra.to_rad().value() as f64)).sin_cos();
    let (de_s, de_c) = (std::f64::consts::PI / 2.0 - (dec.to_rad().value() as f64)).sin_cos();
    rotation_matrix * Vector3::new((de_s * ra_c) as f32, (de_s * ra_s) as f32, (de_c) as f32)
}

pub fn project_point(vector: &Vector3<f32>, zoom: f32, viewport_rect: egui::Rect) -> (egui::Pos2, bool) {
    let scale_factor = 1.0 - vector[2];

    let rect_size = Vector2::new(viewport_rect.max[0] - viewport_rect.min[0], viewport_rect.max[1] - viewport_rect.min[1]);

    let screen_ratio = 2.0 / (rect_size[0] * rect_size[0] + rect_size[1] * rect_size[1]).sqrt();

    let point_coordinates = Vector2::new(vector[0] * zoom / scale_factor, vector[1] * zoom / scale_factor);

    let final_coordinates = egui::Pos2::new(point_coordinates[0] / screen_ratio + rect_size[0] / 2.0, point_coordinates[1] / screen_ratio + rect_size[1] / 2.0);

    (
        final_coordinates,
        is_in_rect(
            final_coordinates.into(),
            [
                [viewport_rect.min[0] - VIEWPORT_OFFSET, viewport_rect.min[1] - VIEWPORT_OFFSET],
                [viewport_rect.max[0] + VIEWPORT_OFFSET, viewport_rect.max[1] + VIEWPORT_OFFSET],
            ],
        ),
    )
}

//something is broken over here and I have no idea what it is...
pub fn cast_onto_sphere(cellestial_sphere: &CellestialSphere, screen_position: &egui::Pos2) -> Vector3<f32> {
    let rect_size = Vector2::new(
        cellestial_sphere.viewport_rect.max[0] - cellestial_sphere.viewport_rect.min[0],
        cellestial_sphere.viewport_rect.max[1] - cellestial_sphere.viewport_rect.min[1],
    );

    let screen_ratio = 2.0 / (rect_size[0] * rect_size[0] + rect_size[1] * rect_size[1]).sqrt();

    let plane_coordinates = Vector2::new((screen_position[0] - rect_size[0] / 2.0) * screen_ratio, (screen_position[1] - rect_size[1] / 2.0) * screen_ratio);

    cast_onto_sphere_plane_position(cellestial_sphere, plane_coordinates)
}

pub fn cast_onto_sphere_plane_position(cellestial_sphere: &CellestialSphere, plane_coordinates: Vector2<f32>) -> Vector3<f32> {
    let scaling_factor = cellestial_sphere.zoom * cellestial_sphere.zoom + plane_coordinates[0] * plane_coordinates[0] + plane_coordinates[1] * plane_coordinates[1];

    cellestial_sphere.rotation.matrix().try_inverse().expect("FUCK")
        * Vector3::new(
            2.0 * cellestial_sphere.get_zoom() * cellestial_sphere.get_zoom() * plane_coordinates[0] / scaling_factor,
            2.0 * cellestial_sphere.get_zoom() * cellestial_sphere.get_zoom() * plane_coordinates[1] / scaling_factor,
            -(cellestial_sphere.get_zoom() * cellestial_sphere.get_zoom() - plane_coordinates[0] * plane_coordinates[0] - plane_coordinates[1] * plane_coordinates[1]) * cellestial_sphere.get_zoom()
                / (scaling_factor),
        )
}

/** Returns a (dec, ra) pair (both in radians) */
pub fn cartesian_to_spherical(vector: Vector3<f32>) -> (angle::Rad<f32>, angle::Rad<f32>) {
    /*let v = vector.normalize();
    let x = v[0];
    let y = v[1];
    let ra = y.atan2(x);
    let sin_dec = x / ra.cos();
    let cos_dec = v[2];
    let dec = sin_dec.atan2(cos_dec);
    (PI / 2.0 - dec, -ra)*/
    let dec = PI / 2.0 - vector.normalize()[2].acos();
    let mut ra = -(vector[1].atan2(vector[0]));
    if ra < 0.0 {
        ra += 2.0 * PI;
    }
    (angle::Rad(dec), angle::Rad(ra))
}
pub fn cast_onto_sphere_dec_ra(cellestial_sphere: &CellestialSphere, screen_position: &egui::Pos2) -> [angle::Rad<f32>; 2] {
    let sphere_position = cast_onto_sphere(cellestial_sphere, screen_position);
    let (dec, ra) = cartesian_to_spherical(sphere_position);
    [dec, ra]
}
/**
 * initial_position: (ra, dec) both in radians
 * final_position: (ra, dec) both in radians
 */
pub fn angular_distance(initial_position: (angle::Rad<f32>, angle::Rad<f32>), final_position: (angle::Rad<f32>, angle::Rad<f32>)) -> angle::Rad<f32> {
    let (i_ra, i_dec) = initial_position;
    let (f_ra, f_dec) = final_position;

    /*
    a = ?
    A = i_ra - f_ra
    b = PI/2 - i_dec
    c = PI/2 - f_dec
    cos(a) = cos(b)cos(c) + sin(b)sin(c)cos(A)
    */

    let b = PI / 2.0 - i_dec.value();
    let c = PI / 2.0 - f_dec.value();
    angle::Rad((b.cos() * c.cos() + b.sin() * c.sin() * (i_ra - f_ra).cos()).acos())

    // (i_dec.cos() * f_dec.cos() + i_dec.sin() * i_dec.sin() * (i_ra - f_ra).cos()).acos()
}
/// Returns a (ra, dec) pair, both in degrees
pub fn generate_random_point(rng: &mut ThreadRng) -> (angle::Deg<f32>, angle::Deg<f32>) {
    // Generate a random right ascension as normal
    // Then generate a height from the dec = 0 plane and compute the declination from that (since having a uniform distribution of declinations results in higher points density at the poles)
    // This works since it essentially generates a random point on a cylinder and then projects it onto the sphere using an "inverse orthographic projection", which conserves areas and therefore also the fact that the points are uniformly distributed across the area.
    (
        angle::Deg(rng.gen_range(0.0..360.0)),
        angle::Deg((90.0 - rng.gen_range(-1.0_f32..=1.0_f32).acos() * 180.0 / PI).clamp(-90.0, 90.0)),
    )
}
pub fn ccw(a: Pos2, b: Pos2, c: Pos2) -> bool {
    (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
}
pub fn intersect(a: LineSegment, b: LineSegment) -> bool {
    ccw(a.start, b.start, b.end) != ccw(a.end, b.start, b.end) && ccw(a.start, a.end, b.start) != ccw(a.start, a.end, b.end)
}

pub fn is_inside_polygon(polygon: Vec<SphericalPoint>, point: (f32, f32), meridian_constellation: bool) -> bool {
    let (pra, pdec) = point;
    let mut crossed = 0;
    for i in 0..polygon.len() {
        let startpoint = polygon[i];
        let endpoint = polygon[(i + 1) % polygon.len()];
        let (ira, idec) = (startpoint.ra(), startpoint.dec());
        let (fra, fdec) = (endpoint.ra(), endpoint.dec());
        #[allow(clippy::collapsible_else_if)] // I believe this is more readable
        if meridian_constellation {
            if intersect(
                LineSegment::from([[(ira + 180.0) % 360.0, idec], [(fra + 180.0) % 360.0, fdec]]),
                LineSegment::from([[(pra + 180.0) % 360.0, pdec], [0.0, 0.0]]),
            ) {
                crossed += 1;
            }
        } else {
            if intersect(LineSegment::from([[ira, idec], [fra, fdec]]), LineSegment::from([[pra, pdec], [0.0, 0.0]])) {
                crossed += 1;
            }
        }
    }
    crossed % 2 == 1
}

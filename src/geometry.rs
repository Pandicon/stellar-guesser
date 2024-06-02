use nalgebra::{Matrix3, Vector2, Vector3};
use rand::{rngs::ThreadRng, Rng};
use std::f32::consts::PI;

use crate::caspr::CellestialSphere;

// const POLYGONLIMIT: f32 = 180.0;
const VIEWPORT_OFFSET: f32 = 10.0;

pub fn is_in_rect<T: PartialOrd>(point: [T; 2], rect: [[T; 2]; 2]) -> bool {
    let [upper_left, bottom_right] = rect;
    point[0] >= upper_left[0] && point[0] <= bottom_right[0] && point[1] >= upper_left[1] && point[1] <= bottom_right[1]
}

pub fn get_point_vector(ra: f32, dec: f32, rotation_matrix: &Matrix3<f32>) -> Vector3<f32> {
    let (ra_s, ra_c) = ((-ra as f64) * std::f64::consts::PI / 180.0).sin_cos();
    let (de_s, de_c) = ((90.0 - dec as f64) * std::f64::consts::PI / 180.0).sin_cos();
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
pub fn cartesian_to_spherical(vector: Vector3<f32>) -> (f32, f32) {
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
    (dec, ra)
}
pub fn cast_onto_sphere_dec_ra(cellestial_sphere: &CellestialSphere, screen_position: &egui::Pos2) -> [f32; 2] {
    let sphere_position = cast_onto_sphere(cellestial_sphere, &screen_position);
    let (dec, ra) = cartesian_to_spherical(sphere_position);
    [dec, ra]
}
/**
 * initial_position: (ra, dec) both in radians
 * final_position: (ra, dec) both in radians
 */
pub fn angular_distance(initial_position: (f32, f32), final_position: (f32, f32)) -> f32 {
    let (i_ra, i_dec) = initial_position;
    let (f_ra, f_dec) = final_position;

    /*
    a = ?
    A = i_ra - f_ra
    b = PI/2 - i_dec
    c = PI/2 - f_dec
    cos(a) = cos(b)cos(c) + sin(b)sin(c)cos(A)
    */

    let b = PI / 2.0 - i_dec;
    let c = PI / 2.0 - f_dec;
    (b.cos() * c.cos() + b.sin() * c.sin() * (i_ra - f_ra).cos()).acos()

    // (i_dec.cos() * f_dec.cos() + i_dec.sin() * i_dec.sin() * (i_ra - f_ra).cos()).acos()
}
pub fn generate_random_point(rng: &mut ThreadRng) -> (f32, f32) {
    (rng.gen_range(0.0..360.0), rng.gen_range(-90.0..90.0))
}
pub fn ccw(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> bool {
    let (ax, ay) = a;
    let (bx, by) = b;
    let (cx, cy) = c;
    (cy - ay) * (bx - ax) > (by - ay) * (cx - ax)
}
pub fn intersect(a: (f32, f32), b: (f32, f32), c: (f32, f32), d: (f32, f32)) -> bool {
    ccw(a, c, d) != ccw(b, c, d) && ccw(a, b, c) != ccw(a, b, d)
}

pub fn is_inside_polygon(polygon: Vec<(f32, f32)>, point: (f32, f32), meridian_constellation: bool) -> bool {
    let (pra, pdec) = point;
    let mut crossed = 0;
    for i in 0..polygon.len() {
        let startpoint = polygon[i];
        let endpoint = polygon[(i + 1) % polygon.len()];
        let (ira, idec) = startpoint;
        let (fra, fdec) = endpoint;
        #[allow(clippy::collapsible_else_if)] // I believe this is more readable
        if meridian_constellation {
            if intersect(((ira + 180.0) % 360.0, idec), ((fra + 180.0) % 360.0, fdec), ((pra + 180.0) % 360.0, pdec), (0.0, 0.0)) {
                crossed += 1;
            }
        } else {
            if intersect((ira, idec), (fra, fdec), (pra, pdec), (0.0, 0.0)) {
                crossed += 1;
            }
        }
    }
    crossed % 2 == 1
}

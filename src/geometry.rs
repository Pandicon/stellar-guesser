use eframe::egui;
use nalgebra::{Matrix3, Vector2, Vector3};
use std::f32::consts::PI;

use crate::caspr::CellestialSphere;

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

	(
		egui::Pos2::new(point_coordinates[0] / screen_ratio + rect_size[0] / 2.0, point_coordinates[1] / screen_ratio + rect_size[1] / 2.0),
		// Is it within the bounds that we want to render in? //TODO: Use the geometry::is_in_rect function
		// TODO: Probably fix this - see how it is rendering into the top panel
		((rect_size[0] * screen_ratio / 2.0 > point_coordinates[0]) && (point_coordinates[0] > -rect_size[0] * screen_ratio / 2.0))
			|| ((rect_size[1] * screen_ratio / 2.0 > point_coordinates[1]) && (point_coordinates[1] > -rect_size[1] * screen_ratio / 2.0)),
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
			-(cellestial_sphere.get_zoom() * cellestial_sphere.get_zoom() - plane_coordinates[0] * plane_coordinates[0] * plane_coordinates[1] * plane_coordinates[1]) * cellestial_sphere.get_zoom()
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
		ra = 2.0 * PI + ra;
	}
	(dec, ra)
}
pub fn angular_distance(initial_position: (f32, f32), final_position: (f32, f32)) -> f32 {
	let (i_ra, i_dec) = initial_position;
	let (f_ra, f_dec) = final_position;

	(i_dec.cos() * f_dec.cos() + i_dec.sin() * i_dec.sin() * (i_ra - f_ra).cos()).acos()
}

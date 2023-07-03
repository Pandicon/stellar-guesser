use eframe::egui;
use nalgebra::{Matrix3, Vector2, Vector3};
use std::f32::consts::PI;

pub fn is_in_rect<T: PartialOrd>(point: [T; 2], rect: [[T; 2]; 2]) -> bool {
	let [upper_left, bottom_right] = rect;
	point[0] >= upper_left[0] && point[0] <= bottom_right[0] && point[1] >= upper_left[1] && point[1] <= bottom_right[1]
}

pub fn get_point_vector(ra: f32, dec: f32, rotation_matrix: Matrix3<f32>) -> Vector3<f32> {
	let (ra_s, ra_c) = ((-ra) * PI / 180.0).sin_cos();
	let (de_s, de_c) = ((90.0 - dec) * PI / 180.0).sin_cos();
	rotation_matrix * Vector3::new(de_s * ra_c, de_s * ra_s, de_c)
}

pub fn project_point(vector: &Vector3<f32>, zoom: f32, viewport_rect: egui::Rect) -> (egui::Pos2, bool) {
	let scale_factor = 1.0 - vector[2];

	let rect_size = Vector2::new(viewport_rect.max[0] - viewport_rect.min[0], viewport_rect.max[1] - viewport_rect.min[1]);

	let screen_ratio = 2.0 / (rect_size[0] * rect_size[0] + rect_size[1] * rect_size[1]).sqrt();

	let point_coordinates = Vector2::new(vector[0]*zoom / scale_factor, vector[1]*zoom / scale_factor);

	(
		egui::Pos2::new(point_coordinates[0] / screen_ratio + rect_size[0] / 2.0, point_coordinates[1] / screen_ratio + rect_size[1] / 2.0),
		// Is it within the bounds that we want to render in? //TODO: Use the geometry::is_in_rect function
		// TODO: Probably fix this - see how it is rendering into the top panel
		((rect_size[0] * screen_ratio / 2.0 > point_coordinates[0]) && (point_coordinates[0] > -rect_size[0] * screen_ratio / 2.0))
			|| ((rect_size[1] * screen_ratio / 2.0 > point_coordinates[1]) && (point_coordinates[1] > -rect_size[1] * screen_ratio / 2.0)),
	)
}

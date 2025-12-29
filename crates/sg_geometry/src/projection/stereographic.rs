use angle::Angle;

fn stereographic_scale(fov_full: angle::Rad<f32>, viewport: &egui::Rect) -> f32 {
    let r = viewport.width().min(viewport.height()) * 0.5; // R in pixels
    let theta_max = fov_full * 0.5; // half-angle
    r / ((theta_max / 2.0).tan())
}

pub fn stereographic_matrix(fov_full: angle::Rad<f32>, viewport: &egui::Rect) -> nalgebra::Matrix4<f32> {
    let s = stereographic_scale(fov_full, viewport);

    let far = 1000.0;
    let near = 0.1;
    let a = (far + near) / (far - near);
    let b = -(2.0 * far * near) / (far - near);

    #[rustfmt::skip]
    let m = nalgebra::Matrix4::new(
          s, 0.0, 0.0, 0.0,
        0.0,   s, 0.0, 0.0,
        0.0, 0.0,   a,   b,
        0.0, 0.0, 1.0, 1.0
    );

    m
}

pub fn cast_onto_sphere(viewport_rect: &egui::Rect, screen_position: &egui::Pos2, rotation: nalgebra::Rotation3<f32>, fov_full: angle::Rad<f32>) -> nalgebra::Vector3<f32> {
    let rect_size = nalgebra::Vector2::new(viewport_rect.max[0] - viewport_rect.min[0], viewport_rect.max[1] - viewport_rect.min[1]);

    let plane_coordinates = nalgebra::Vector2::new(screen_position[0] - rect_size[0] / 2.0, screen_position[1] - rect_size[1] / 2.0);

    cast_onto_sphere_plane_position(rotation, fov_full, plane_coordinates, viewport_rect)
}

fn inverse_stereographic(screen_pos: egui::Pos2, scale: f32) -> nalgebra::Vector3<f32> {
    let x = screen_pos.x;
    let y = screen_pos.y;

    // normalized stereographic coordinates
    let u = x / scale;
    let v = y / scale;

    let r2 = u * u + v * v;
    let denom = r2 + 1.0;

    nalgebra::Vector3::new(2.0 * u / denom, 2.0 * v / denom, (1.0 - r2) / denom)
}

pub fn cast_onto_sphere_plane_position(rotation: nalgebra::Rotation3<f32>, full_fov: angle::Rad<f32>, plane_coordinates: nalgebra::Vector2<f32>, viewport_rect: &egui::Rect) -> nalgebra::Vector3<f32> {
    let scale = stereographic_scale(full_fov, viewport_rect);
    let world_position = inverse_stereographic(egui::pos2(plane_coordinates.x, plane_coordinates.y), scale);

    rotation.inverse() * world_position
}

pub fn project_point_raw(vector: &nalgebra::Vector3<f32>, fov_full: angle::Rad<f32>, viewport_rect: egui::Rect) -> Option<egui::Pos2> {
    let proj = stereographic_matrix(fov_full, &viewport_rect);
    let v4 = nalgebra::Vector4::new(vector.x, vector.y, vector.z, 1.0);
    let clip = proj * v4;

    let w = clip.w;
    if w.abs() < 1e-6 {
        return None;
    } // near singularity

    let ndc_x = clip.x / w;
    let ndc_y = clip.y / w;

    let point_coordinates = nalgebra::Vector2::new(ndc_x, ndc_y);

    Some(egui::pos2(point_coordinates.x, point_coordinates.y))
}

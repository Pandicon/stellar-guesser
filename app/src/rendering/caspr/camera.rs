#[derive(Copy, Clone)]
pub struct Camera {
    pub zoom: f32,
    pub fov: angle::Rad<f32>,
    pub camera_z: f32,

    pub projection: sg_geometry::projection::Projection,
    pub rotation: nalgebra::Rotation3<f32>,

    pub viewport_rect: eframe::egui::Rect,

    pub changed: bool,
    pub changed_fov: bool,
    pub changed_rotation: bool,
    pub changed_projection: bool,
    pub changed_viewport_rect: bool,
}

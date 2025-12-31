use crate::rendering::caspr;

#[derive(Copy, Clone)]
pub struct Camera {
    zoom: f32,
    fov: angle::Rad<f32>,
    camera_z: f32,

    projection: sg_geometry::projection::Projection,
    rotation: nalgebra::Rotation3<f32>,

    viewport_rect: eframe::egui::Rect,

    changes_state: CameraChangesState,
}

impl Camera {
    pub fn new_with_fov(fov: angle::Rad<f32>, projection: sg_geometry::projection::Projection, rotation: nalgebra::Rotation3<f32>, viewport_rect: eframe::egui::Rect) -> Self {
        let mut camera = Self {
            zoom: 0.0,
            fov: fov - angle::Rad(0.2),
            camera_z: 0.0,

            projection,
            rotation,

            viewport_rect,

            changes_state: Default::default(),
        };
        camera.set_fov(fov);
        camera
    }

    pub fn set_fov(&mut self, fov: angle::Rad<f32>) {
        if fov != self.fov {
            self.fov = fov;
            self.zoom = caspr::renderer::CellestialSphere::fov_to_zoom(fov);
            self.camera_z = caspr::renderer::CellestialSphere::fov_to_camera_z(fov);

            self.changes_state.changed_fov = true;
            self.changes_state.changed = true;
        }
    }

    pub fn get_fov(&self) -> &angle::Rad<f32> {
        &self.fov
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        if zoom != self.zoom {
            self.zoom = zoom;
            self.fov = caspr::renderer::CellestialSphere::zoom_to_fov(self.zoom);
            self.camera_z = caspr::renderer::CellestialSphere::fov_to_camera_z(self.fov);

            self.changes_state.changed_fov = true;
            self.changes_state.changed = true;
        }
    }

    pub fn get_zoom(&self) -> &f32 {
        &self.zoom
    }

    pub fn get_camera_z(&self) -> &f32 {
        &self.camera_z
    }

    pub fn set_rotation(&mut self, rotation: nalgebra::Rotation3<f32>) {
        if rotation != self.rotation {
            self.rotation = rotation;

            self.changes_state.changed_rotation = true;
            self.changes_state.changed = true;
        }
    }

    pub fn get_rotation(&self) -> &nalgebra::Rotation3<f32> {
        &self.rotation
    }

    pub fn set_projection(&mut self, projection: sg_geometry::projection::Projection) {
        if projection != self.projection {
            self.projection = projection;

            self.changes_state.changed_projection = true;
            self.changes_state.changed = true;
        }
    }

    pub fn get_projection(&self) -> &sg_geometry::projection::Projection {
        &self.projection
    }

    pub fn set_viewport_rect(&mut self, viewport_rect: eframe::egui::Rect) {
        if viewport_rect != self.viewport_rect {
            self.viewport_rect = viewport_rect;

            self.changes_state.changed_viewport_rect = true;
            self.changes_state.changed = true;
        }
    }

    pub fn get_viewport_rect(&self) -> &eframe::egui::Rect {
        &self.viewport_rect
    }

    pub fn get_changes_state(&self) -> &CameraChangesState {
        &self.changes_state
    }

    pub fn reset_changes_state(&mut self) {
        self.changes_state = Default::default();
    }
}

#[derive(Copy, Clone)]
pub struct CameraChangesState {
    pub changed: bool,
    pub changed_fov: bool,
    pub changed_rotation: bool,
    pub changed_projection: bool,
    pub changed_viewport_rect: bool,
}

impl Default for CameraChangesState {
    fn default() -> Self {
        Self {
            changed: false,
            changed_fov: false,
            changed_rotation: false,
            changed_projection: false,
            changed_viewport_rect: false,
        }
    }
}

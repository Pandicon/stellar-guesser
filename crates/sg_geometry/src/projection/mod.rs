pub const VIEWPORT_OFFSET: f32 = 10.0;

mod stereographic;

pub enum Projection {
    Stereographic,
}

impl Projection {
    pub fn get_projection_matrix(&self, fov_full: angle::Rad<f32>, viewport: egui::Rect) -> nalgebra::Matrix4<f32> {
        match *self {
            Self::Stereographic => stereographic::stereographic_matrix(fov_full, &viewport),
        }
    }

    pub fn cast_onto_sphere(&self, viewport_rect: &egui::Rect, screen_position: &egui::Pos2, rotation: nalgebra::Rotation3<f32>, fov_full: angle::Rad<f32>) -> nalgebra::Vector3<f32> {
        match *self {
            Self::Stereographic => stereographic::cast_onto_sphere(viewport_rect, screen_position, rotation, fov_full),
        }
    }

    pub fn cast_onto_sphere_plane_position(
        &self,
        rotation: nalgebra::Rotation3<f32>,
        full_fov: angle::Rad<f32>,
        plane_coordinates: nalgebra::Vector2<f32>,
        viewport_rect: &egui::Rect,
    ) -> nalgebra::Vector3<f32> {
        match *self {
            Self::Stereographic => stereographic::cast_onto_sphere_plane_position(rotation, full_fov, plane_coordinates, viewport_rect),
        }
    }

    pub fn cast_onto_sphere_dec_ra(&self, viewport_rect: &egui::Rect, screen_position: &egui::Pos2, rotation: nalgebra::Rotation3<f32>, fov_full: angle::Rad<f32>) -> [angle::Rad<f32>; 2] {
        let sphere_position = self.cast_onto_sphere(viewport_rect, screen_position, rotation, fov_full);
        let (dec, ra) = crate::cartesian_to_spherical(sphere_position);
        [dec, ra]
    }

    pub fn project_point(&self, vector: &nalgebra::Vector3<f32>, fov_full: angle::Rad<f32>, viewport_rect: egui::Rect) -> (egui::Pos2, bool) {
        let projected_raw = match *self {
            Self::Stereographic => stereographic::project_point_raw(vector, fov_full, viewport_rect),
        };
        match projected_raw {
            Some(point_coordinates) => {
                let rect_size = nalgebra::Vector2::new(viewport_rect.max[0] - viewport_rect.min[0], viewport_rect.max[1] - viewport_rect.min[1]);

                let final_coordinates = egui::Pos2::new(point_coordinates[0] + rect_size[0] / 2.0, point_coordinates[1] + rect_size[1] / 2.0);

                (
                    final_coordinates,
                    crate::is_in_rect(
                        final_coordinates.into(),
                        [
                            [viewport_rect.min[0] - VIEWPORT_OFFSET, viewport_rect.min[1] - VIEWPORT_OFFSET],
                            [viewport_rect.max[0] + VIEWPORT_OFFSET, viewport_rect.max[1] + VIEWPORT_OFFSET],
                        ],
                    ),
                )
            }
            None => (egui::pos2(0.0, 0.0), false),
        }
    }
}

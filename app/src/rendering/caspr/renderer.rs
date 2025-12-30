use crate::{
    enums::{LightPollution, RendererCategory, StorageKeys},
    rendering::caspr,
    sky,
};
use angle::Angle;
use eframe::egui::{self, Align2, FontFamily, FontId};
use egui::epaint::Color32;
use nalgebra::{Rotation3, Vector3};
use sg_geometry::{intersections, LineSegment, Rectangle};
use std::collections::HashMap;

const ZOOM_CAP: f32 = 100.0;

pub const MAG_TO_LIGHT_POLLUTION_RAW: [(LightPollution, [Option<sky::star::MagnitudeToRadius>; sky::star::MAGNITUDE_TO_RADIUS_OPTIONS]); 4] = [
    (
        LightPollution::Default,
        [Some(sky::star::MagnitudeToRadius::defaults()[0]), Some(sky::star::MagnitudeToRadius::defaults()[1])],
    ),
    (
        LightPollution::PragueDark,
        [
            Some(sky::star::MagnitudeToRadius::Linear { mag_scale: 1.0, mag_offset: 4.3 }),
            Some(sky::star::MagnitudeToRadius::Exponential { r_0: 2.3, n: 3.5, o: 0.21 }),
        ],
    ),
    (
        LightPollution::Prague,
        [
            Some(sky::star::MagnitudeToRadius::Linear { mag_scale: 0.75, mag_offset: 3.75 }),
            Some(sky::star::MagnitudeToRadius::Exponential { r_0: 1.4, n: 3.5, o: 0.21 }),
        ],
    ),
    (
        LightPollution::AverageVillage,
        [
            Some(sky::star::MagnitudeToRadius::Linear { mag_scale: 0.7, mag_offset: 5.7 }),
            Some(sky::star::MagnitudeToRadius::Exponential { r_0: 2.6, n: 3.0, o: 0.17 }),
        ],
    ),
];

// use geometry::{cast_onto_sphere, project_point};

use super::deepsky;
use super::sky_settings;
use super::stars::StarRenderer;

pub struct CellestialSphere {
    pub sky_settings: sky_settings::SkySettings,
    star_renderers: HashMap<String, Vec<StarRenderer>>,
    line_renderers: HashMap<String, Vec<caspr::lines::LineRenderer>>,
    deepsky_renderers: HashMap<String, Vec<deepsky::DeepskyRenderer>>,
    marker_renderers: HashMap<String, Vec<caspr::markers::MarkerRenderer>>,

    pub camera: caspr::camera::Camera,

    pub textures: caspr::textures::Textures,
}

impl CellestialSphere {
    //Renders a circle based on its current normal (does NOT account for the rotation of the sphere)
    pub fn render_circle(&self, normal: &Vector3<f32>, radius: f32, color: egui::epaint::Color32, painter: &egui::Painter) {
        let (projected_point, is_within_bounds) = self.camera.projection.project_point(normal, self.camera.fov, self.camera.viewport_rect);

        if is_within_bounds {
            painter.circle_filled(projected_point, radius, color);
        }
    }

    pub fn render_line(&self, start: &Vector3<f32>, end: &Vector3<f32>, colour: Color32, width: f32, painter: &egui::Painter) {
        let (start_point, is_start_within_bounds) = self.camera.projection.project_point(start, self.camera.fov, self.camera.viewport_rect);
        let (end_point, is_end_within_bounds) = self.camera.projection.project_point(end, self.camera.fov, self.camera.viewport_rect);

        let screen_rect = Rectangle::from(self.camera.viewport_rect);

        // Allow the whole half sphere or what is within the FOV (whichever is greater)
        // This gets rid of lines on the other half of the sphere while also not removing lines that should be visible at large zooms
        let modified_camera_z = self.camera.camera_z.max(0.0);

        // Neither the starting point nor the ending point is visible AND either of them is behind the camera
        // This avoids lines from the part of the sky that is behind us (north pole when looking at the south pole) being drawn over the screen
        if !(is_start_within_bounds || is_end_within_bounds) && (modified_camera_z > start.z || modified_camera_z > end.z) {
            return;
        }
        // Neither the starting point nor the ending point is behind the camera OR either of them is on the screen (out of the FOV cone, but within the screen rectangle) -> the line should be drawn
        // TODO: Fix it when the line crosses a corner of the screen - both of the end points go out of the screen and behind the camera while a part of the line should still be visible
        if is_start_within_bounds || is_end_within_bounds || intersections::rect_segment(screen_rect, LineSegment::new(start_point, end_point)) {
            painter.line_segment([start_point, end_point], egui::Stroke::new(width, colour));
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render_marker(
        &self,
        centre_vector: &Vector3<f32>,
        other_vector: &Option<Vector3<f32>>,
        circle: bool,
        pixel_size: Option<f32>,
        colour: Color32,
        width: f32,
        painter: &egui::Painter,
        label: Option<String>,
    ) {
        let (centre_point, is_centre_within_bounds) = self.camera.projection.project_point(centre_vector, self.camera.fov, self.camera.viewport_rect);
        if !is_centre_within_bounds {
            return;
        }
        let size = if let Some(other_point_vec) = other_vector {
            let (other_point, _) = self.camera.projection.project_point(other_point_vec, self.camera.fov, self.camera.viewport_rect);
            let vec_to = other_point - centre_point;
            vec_to.length()
        } else if let Some(pixel_size) = pixel_size {
            pixel_size
        } else {
            return;
        };
        if circle {
            painter.circle(centre_point, size, Color32::TRANSPARENT, egui::Stroke::new(width, colour));
        } else {
            painter.line_segment(
                [egui::pos2(centre_point.x, centre_point.y - size), egui::pos2(centre_point.x, centre_point.y + size)],
                egui::Stroke::new(width, colour),
            );
            painter.line_segment(
                [egui::pos2(centre_point.x - size, centre_point.y), egui::pos2(centre_point.x + size, centre_point.y)],
                egui::Stroke::new(width, colour),
            );
        }
        if self.sky_settings.render_labels {
            if let Some(text) = label {
                _ = painter.text(
                    egui::pos2(centre_point.x + size + 0.1, centre_point.y + size + 0.1),
                    Align2::LEFT_TOP,
                    text,
                    FontId::new(10.0, FontFamily::Monospace),
                    colour,
                );
            }
        }
    }

    pub fn prepare_render(&mut self, sky: &sky::Sky) {
        if self.camera.changed_rotation || self.camera.changed_projection || self.camera.changed_viewport_rect {
            self.init_renderers(sky);
        } else if self.camera.changed_fov {
            self.reinit_renderer_category(sky, RendererCategory::Stars);
        }
        self.camera.changed = self.camera.changed_fov || self.camera.changed_projection || self.camera.changed_rotation || self.camera.changed_viewport_rect;
    }

    // Renders the entire sphere view
    pub fn render_sky(&self, painter: &egui::Painter, frame: &mut eframe::Frame) {
        let target_format = frame.wgpu_render_state().map(|state| state.target_format).unwrap_or(eframe::wgpu::TextureFormat::Bgra8Unorm);
        painter.add(eframe::egui_wgpu::Callback::new_paint_callback(
            self.camera.viewport_rect,
            caspr::clouds::renderer::CloudsCallback {
                camera_data: self.camera,
                clouds_settings: self.sky_settings.cloud_settings,
                clouds_texture_to_upload: self.textures.clouds_texture_to_upload.clone(),
                target_format,

                render: true,
            },
        ));
        for line_renderers in self.line_renderers.values() {
            for line_renderer in line_renderers {
                line_renderer.render(self, painter);
            }
        }
        for star_renderers in self.star_renderers.values() {
            for star_renderer in star_renderers {
                star_renderer.render(painter);
            }
        }
        for deepsky_renderers in self.deepsky_renderers.values() {
            for deepsky_renderer in deepsky_renderers {
                deepsky_renderer.render(self, painter);
            }
        }
        // Make sure the game markers are rendered last, so they are not obstructed
        let mut keys: Vec<&String> = self.marker_renderers.keys().collect();
        keys.sort_by(|a, b| match (a.as_str() == "game", b.as_str() == "game") {
            (true, true) => std::cmp::Ordering::Equal,
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            (false, false) => a.cmp(b),
        });
        for key in keys {
            if let Some(marker_renderers) = self.marker_renderers.get(key) {
                for marker_renderer in marker_renderers {
                    marker_renderer.render(self, painter);
                }
            }
        }
    }

    pub fn after_render(&mut self) {
        self.camera.changed = false;
        self.camera.changed_fov = false;
        self.camera.changed_projection = false;
        self.camera.changed_rotation = false;
        self.camera.changed_viewport_rect = false;

        self.textures.clouds_texture_to_upload = None;
    }

    pub fn load(storage: Option<&dyn eframe::Storage>) -> Self {
        let mut sky_settings = sky_settings::SkySettings::from_raw(&sky_settings::SkySettingsRaw::default());
        if let Some(storage) = storage {
            if let Some(sky_settings_raw_str) = storage.get_string(StorageKeys::SkySettings.as_ref()) {
                match serde_json::from_str(&sky_settings_raw_str) {
                    Ok(data) => sky_settings = sky_settings::SkySettings::from_raw(&data),
                    Err(err) => log::error!("Failed to deserialize sky settings: {:?}", err),
                }
            }
        }

        let viewport_rect = egui::Rect::from_two_pos(egui::pos2(0.0, 0.0), egui::pos2(0.0, 0.0));
        let zoom = 3.0_f32.sqrt();
        let fov = Self::zoom_to_fov(zoom);
        Self {
            sky_settings,
            star_renderers: HashMap::new(),
            line_renderers: HashMap::new(),
            deepsky_renderers: HashMap::new(),
            marker_renderers: HashMap::new(),

            camera: caspr::camera::Camera {
                zoom,
                fov,
                camera_z: Self::fov_to_camera_z(fov),

                rotation: Rotation3::new(Vector3::new(0.0, 0.0, 0.0)),
                projection: sg_geometry::projection::Projection::Stereographic,

                viewport_rect,

                changed: false,
                changed_fov: false,
                changed_projection: false,
                changed_rotation: false,
                changed_viewport_rect: false,
            },

            textures: caspr::textures::Textures::default(),
        }
    }

    // TODO: Make this always for example halve the FOV
    pub fn zoom(&mut self, velocity: f32) {
        if velocity == 0.0 {
            return;
        }
        let future_zoom = self.camera.zoom + velocity * self.camera.zoom;
        //A check is needed since negative zoom breaks everything
        if ZOOM_CAP > future_zoom && future_zoom > 0.0 {
            self.camera.zoom = future_zoom;
            self.camera.fov = Self::zoom_to_fov(self.camera.zoom);
            self.camera.camera_z = Self::fov_to_camera_z(self.camera.fov);

            self.camera.changed_fov = true;
        }
    }

    pub fn get_zoom(&self) -> f32 {
        self.camera.zoom
    }

    pub fn zoom_to_fov(zoom: f32) -> angle::Rad<f32> {
        angle::Rad(4.0 * (1.0 / zoom).atan())
    }

    pub fn fov_to_camera_z(fov_deg: angle::Rad<f32>) -> f32 {
        (fov_deg / 2.0).cos()
    }

    pub fn init(&mut self, sky: &mut sky::Sky) {
        let settings = sky.light_pollution_place_to_mag_settings(&sky.light_pollution_place, &self.sky_settings);
        self.sky_settings.mag_to_radius_settings[self.sky_settings.mag_to_radius_id] = settings;
        if self.sky_settings.cloud_settings.enabled {
            crate::rendering::caspr::clouds::apply_dimming(sky, self);
        }
        self.init_renderers(sky);
    }

    /// Preserves disabled renderers - will reinitialise them, but will also keep them disabled
    pub fn init_renderers(&mut self, sky: &sky::Sky) {
        {
            let mut old_renderers = HashMap::new();
            std::mem::swap(&mut self.star_renderers, &mut old_renderers);
            let mut active_star_groups = Vec::new();
            let mut all_disabled_renderers = std::collections::HashMap::new();
            for name in sky.stars.keys() {
                let active = self.sky_settings.stars_categories_active.entry(name.to_owned()).or_insert(true);
                if !*active {
                    continue;
                }
                active_star_groups.push(name.to_owned());

                let mut disabled_renderers = std::collections::HashSet::new();
                if let Some(renderers) = old_renderers.get(name) {
                    for renderer in renderers {
                        if renderer.disabled {
                            disabled_renderers.insert(renderer.object_id);
                        }
                    }
                }
                if !disabled_renderers.is_empty() {
                    all_disabled_renderers.insert(name.to_owned(), disabled_renderers);
                }
            }
            for name in active_star_groups {
                self.init_single_renderer_group(sky, RendererCategory::Stars, &name);
                if let Some(disabled_renderers) = all_disabled_renderers.get(&name) {
                    if let Some(renderers) = self.star_renderers.get_mut(&name) {
                        for renderer in renderers {
                            if disabled_renderers.contains(&renderer.object_id) {
                                renderer.disabled = true;
                            }
                        }
                    }
                }
            }
        }

        self.line_renderers = HashMap::new();
        let mut active_line_groups = Vec::new();
        for (name, lines) in &sky.lines {
            if !lines.active {
                continue;
            }
            active_line_groups.push(name.to_owned());
        }
        for name in active_line_groups {
            self.init_single_renderer_group(sky, RendererCategory::Lines, &name);
        }

        {
            let mut old_renderers = HashMap::new();
            std::mem::swap(&mut self.deepsky_renderers, &mut old_renderers);
            let mut active_deepsky_groups = Vec::new();
            let mut all_disabled_renderers = std::collections::HashMap::new();
            for name in sky.stars.keys() {
                let active = self.sky_settings.deepskies_categories_active.entry(name.to_owned()).or_insert(true);
                if !*active {
                    continue;
                }
                active_deepsky_groups.push(name.to_owned());

                let mut disabled_renderers = std::collections::HashSet::new();
                if let Some(renderers) = old_renderers.get(name) {
                    for renderer in renderers {
                        if renderer.disabled {
                            disabled_renderers.insert(renderer.object_id);
                        }
                    }
                }
                if !disabled_renderers.is_empty() {
                    all_disabled_renderers.insert(name.to_owned(), disabled_renderers);
                }
            }
            for name in active_deepsky_groups {
                self.init_single_renderer_group(sky, RendererCategory::Deepskies, &name);
                if let Some(disabled_renderers) = all_disabled_renderers.get(&name) {
                    if let Some(renderers) = self.deepsky_renderers.get_mut(&name) {
                        for renderer in renderers {
                            if disabled_renderers.contains(&renderer.object_id) {
                                renderer.disabled = true;
                            }
                        }
                    }
                }
            }
        }

        self.marker_renderers = HashMap::new();
        let mut active_markers_groups = Vec::new();
        for (name, markers) in &sky.markers {
            if !markers.active {
                continue;
            }
            active_markers_groups.push(name.to_owned());
        }
        for name in active_markers_groups {
            self.init_single_renderer_group(sky, RendererCategory::Markers, &name);
        }
        if sky.game_markers.active {
            self.init_single_renderer_group(sky, RendererCategory::Markers, "game");
        }
    }

    /// Preserves disabled renderers - will reinitialise them, but will also keep them disabled
    pub fn reinit_renderer_category(&mut self, sky: &sky::Sky, category: RendererCategory) {
        match category {
            RendererCategory::Stars => {
                let mut old_renderers = HashMap::new();
                std::mem::swap(&mut self.star_renderers, &mut old_renderers);
                let mut active_star_groups = Vec::new();
                let mut all_disabled_renderers = std::collections::HashMap::new();
                for name in sky.stars.keys() {
                    let active = self.sky_settings.stars_categories_active.entry(name.to_owned()).or_insert(true);
                    if !*active {
                        continue;
                    }
                    active_star_groups.push(name.to_owned());

                    let mut disabled_renderers = std::collections::HashSet::new();
                    if let Some(renderers) = old_renderers.get(name) {
                        for renderer in renderers {
                            if renderer.disabled {
                                disabled_renderers.insert(renderer.object_id);
                            }
                        }
                    }
                    all_disabled_renderers.insert(name.to_owned(), disabled_renderers);
                }
                for name in active_star_groups {
                    self.init_single_renderer_group(sky, RendererCategory::Stars, &name);
                    if let Some(disabled_renderers) = all_disabled_renderers.get(&name) {
                        if let Some(renderers) = self.star_renderers.get_mut(&name) {
                            for renderer in renderers {
                                if disabled_renderers.contains(&renderer.object_id) {
                                    renderer.disabled = true;
                                }
                            }
                        }
                    }
                }
            }
            RendererCategory::Lines => {
                self.line_renderers = HashMap::new();
                let mut active_line_groups = Vec::new();
                for (name, lines) in &sky.lines {
                    if !lines.active {
                        continue;
                    }
                    active_line_groups.push(name.to_owned());
                }
                for name in active_line_groups {
                    self.init_single_renderer_group(sky, RendererCategory::Lines, &name);
                }
            }
            RendererCategory::Deepskies => {
                let mut old_renderers = HashMap::new();
                std::mem::swap(&mut self.deepsky_renderers, &mut old_renderers);
                let mut active_deepsky_groups = Vec::new();
                let mut all_disabled_renderers = std::collections::HashMap::new();
                for name in sky.stars.keys() {
                    let active = self.sky_settings.deepskies_categories_active.entry(name.to_owned()).or_insert(true);
                    if !*active {
                        continue;
                    }
                    active_deepsky_groups.push(name.to_owned());

                    let mut disabled_renderers = std::collections::HashSet::new();
                    if let Some(renderers) = old_renderers.get(name) {
                        for renderer in renderers {
                            if renderer.disabled {
                                disabled_renderers.insert(renderer.object_id);
                            }
                        }
                    }
                    if !disabled_renderers.is_empty() {
                        all_disabled_renderers.insert(name.to_owned(), disabled_renderers);
                    }
                }
                for name in active_deepsky_groups {
                    self.init_single_renderer_group(sky, RendererCategory::Deepskies, &name);
                    if let Some(disabled_renderers) = all_disabled_renderers.get(&name) {
                        if let Some(renderers) = self.deepsky_renderers.get_mut(&name) {
                            for renderer in renderers {
                                if disabled_renderers.contains(&renderer.object_id) {
                                    renderer.disabled = true;
                                }
                            }
                        }
                    }
                }
            }
            RendererCategory::Markers => {
                self.marker_renderers = HashMap::new();
                let mut active_markers_groups = Vec::new();
                for (name, markers) in &sky.markers {
                    if !markers.active {
                        continue;
                    }
                    active_markers_groups.push(name.to_owned());
                }
                for name in active_markers_groups {
                    self.init_single_renderer_group(sky, RendererCategory::Markers, &name);
                }
                if sky.game_markers.active {
                    self.init_single_renderer_group(sky, RendererCategory::Markers, "game");
                }
            }
        }
    }

    pub fn init_single_renderer_group(&mut self, sky: &sky::Sky, category: RendererCategory, name: &str) {
        match category {
            RendererCategory::Stars => {
                if let Some(stars) = sky.stars.get(name) {
                    self.star_renderers.insert(
                        name.to_string(),
                        stars
                            .iter()
                            .map(|star| {
                                star.get_renderer(
                                    &self.camera.projection,
                                    self.camera.rotation.matrix(),
                                    self.sky_settings.mag_to_radius_settings[self.sky_settings.mag_to_radius_id],
                                    self.camera.fov.to_deg(),
                                    self.camera.viewport_rect,
                                )
                            })
                            .collect(),
                    );
                }
            }
            RendererCategory::Lines => {
                if let Some(lines) = sky.lines.get(name) {
                    self.line_renderers.insert(
                        name.to_string(),
                        lines.lines.iter().map(|line| line.get_renderer(self.camera.rotation.matrix(), lines.colour)).collect(),
                    );
                }
            }
            RendererCategory::Deepskies => {
                if let Some(deepskies) = sky.deepskies.get(name) {
                    let mut disabled_renderers = std::collections::HashSet::new();
                    if let Some(renderers) = self.deepsky_renderers.get(name) {
                        for renderer in renderers {
                            if renderer.disabled {
                                disabled_renderers.insert(renderer.object_id);
                            }
                        }
                    }
                    self.deepsky_renderers.insert(
                        name.to_string(),
                        deepskies
                            .deepskies
                            .iter()
                            .map(|deepsky| {
                                let mut renderer = deepsky.get_renderer(self.camera.rotation.matrix(), deepskies.colour);
                                if disabled_renderers.contains(&deepsky.object_id) {
                                    renderer.disabled = true;
                                }
                                renderer
                            })
                            .collect(),
                    );
                }
            }
            RendererCategory::Markers => {
                if name == "game" {
                    self.marker_renderers.insert(
                        name.to_string(),
                        sky.game_markers.markers.iter().filter_map(|marker| marker.get_renderer(self.camera.rotation.matrix())).collect(),
                    );
                } else if let Some(markers) = sky.markers.get(name) {
                    self.marker_renderers.insert(
                        name.to_string(),
                        markers.markers.iter().filter_map(|marker| marker.get_renderer(self.camera.rotation.matrix(), markers.colour)).collect(),
                    );
                }
            }
        }
    }

    pub fn deinit_single_renderer_group(&mut self, category: RendererCategory, name: &str) {
        match category {
            RendererCategory::Stars => {
                self.star_renderers.insert(name.to_string(), Vec::new());
            }
            RendererCategory::Lines => {
                self.line_renderers.insert(name.to_string(), Vec::new());
            }
            RendererCategory::Deepskies => {
                self.deepsky_renderers.insert(name.to_string(), Vec::new());
            }
            RendererCategory::Markers => {
                self.marker_renderers.insert(name.to_string(), Vec::new());
            }
        }
    }

    pub fn enable_single_renderer(&mut self, object_id: u64) {
        for renderer_group in self.star_renderers.values_mut() {
            for renderer in renderer_group {
                if renderer.object_id == object_id {
                    renderer.disabled = false;
                }
            }
        }
        for renderer_group in self.deepsky_renderers.values_mut() {
            for renderer in renderer_group {
                if renderer.object_id == object_id {
                    renderer.disabled = false;
                }
            }
        }
    }

    pub fn disable_single_renderer(&mut self, object_id: u64) {
        for renderer_group in self.star_renderers.values_mut() {
            for renderer in renderer_group {
                if renderer.object_id == object_id {
                    renderer.disabled = true;
                }
            }
        }
        for renderer_group in self.deepsky_renderers.values_mut() {
            for renderer in renderer_group {
                if renderer.object_id == object_id {
                    renderer.disabled = true;
                }
            }
        }
    }

    /*pub fn mag_to_radius(&self, vmag: f32) -> f32 {
        let mag = self.sky_settings.mag_scale * (self.sky_settings.mag_offset - vmag) + 0.5;
        if mag < 0.35 {
            0.0
        } else {
            mag
        }
    }*/

    pub fn project_screen_pos(&self, screen_pos: egui::Pos2) -> Vector3<f32> {
        self.camera.projection.cast_onto_sphere(&self.camera.viewport_rect, &screen_pos, self.camera.rotation, self.camera.fov)
    }

    pub fn rotate_between_points(&mut self, initial_pos: &Vector3<f32>, final_pos: &Vector3<f32>) -> Option<()> {
        if initial_pos == final_pos {
            return None;
        }
        if let Some(rotation_matrix) = Rotation3::rotation_between(initial_pos, final_pos) {
            if rotation_matrix.matrix()[0].is_nan() {
                return None;
            }
            self.camera.rotation *= rotation_matrix;
            self.camera.changed_rotation = true;
        } else {
            return None;
        }
        Some(())
    }

    /// Rotates the view to look at the point. It has to be taken without rotations.
    pub fn look_at_point(&mut self, point: &Vector3<f32>) -> Option<()> {
        let z_axis = Vector3::new(0.0, 0.0, -1.0);
        let y_axis = Vector3::new(0.0, -1.0, 0.0);
        let axis = if point.cross(&z_axis).magnitude_squared() < 0.01 { y_axis } else { z_axis };
        let rotation_matrix = Rotation3::look_at_rh(&(-point), &axis);
        if rotation_matrix.matrix()[0].is_nan() {
            return None;
        }
        self.camera.rotation = rotation_matrix;
        self.camera.changed_rotation = true;
        Some(())
    }
}

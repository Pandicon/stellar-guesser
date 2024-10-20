use egui::Color32;
use nalgebra::Matrix3;

use crate::{geometry::get_point_vector, rendering::themes::GameMarkersColours};

use super::{Marker, MarkerRenderer};

pub struct GameMarkers {
    pub active: bool,
    pub markers: Vec<GameMarker>,
}

pub struct GameMarker {
    pub marker_type: GameMarkerType,
    pub colour: Color32,

    pub ra: angle::Deg<f32>,
    pub dec: angle::Deg<f32>,
    pub line_width: f32,
    pub angular_radius: Option<angle::Deg<f32>>,
    pub pixel_radius: Option<f32>,
    pub angular_width: Option<angle::Deg<f32>>,
    pub pixel_width: Option<f32>,
}

impl GameMarker {
    /**
     * ra - the right ascension (in degrees)
     * dec - the declination (in degrees)
     * colour - the colour of the marker
     * line_width - the width of the line of the marker
     * half_size - the distance from the centre to the edge of the marker (radius for circular markers)
     * circular - if the marker is circular or not, if not then it is a cross
     * angular_size - if the half_size is in degrees or in pixels
     */
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        marker_type: GameMarkerType,
        ra: angle::Deg<f32>,
        dec: angle::Deg<f32>,
        line_width: f32,
        half_size: f32,
        circular: bool,
        angular_size: bool,
        game_markers_colours: &GameMarkersColours,
    ) -> Self {
        #[allow(clippy::collapsible_else_if)]
        let [angular_radius, pixel_radius, angular_width, pixel_width] = if circular {
            if angular_size {
                [Some(half_size), None, None, None]
            } else {
                [None, Some(half_size), None, None]
            }
        } else {
            if angular_size {
                [None, None, Some(half_size), None]
            } else {
                [None, None, None, Some(half_size)]
            }
        };
        Self {
            marker_type,
            colour: Self::get_colour(marker_type, game_markers_colours),

            ra,
            dec,
            line_width,
            angular_radius: angular_radius.map(angle::Deg),
            pixel_radius,
            angular_width: angular_width.map(angle::Deg),
            pixel_width,
        }
    }

    pub fn get_renderer(&self, rotation_matrix: &Matrix3<f32>) -> Option<MarkerRenderer> {
        if self.angular_radius.is_none() && self.pixel_radius.is_none() && self.angular_width.is_none() && self.pixel_width.is_none() {
            return None;
        }
        let other_vec = if let Some(angular_radius) = self.angular_radius {
            Some(get_point_vector(
                self.ra,
                if self.dec + angular_radius <= angle::Deg(90.0) {
                    self.dec + angular_radius
                } else {
                    self.dec - angular_radius
                },
                rotation_matrix,
            ))
        } else {
            self.angular_width.map(|angular_width| {
                get_point_vector(
                    self.ra,
                    if self.dec + angular_width <= angle::Deg(90.0) {
                        self.dec + angular_width
                    } else {
                        self.dec - angular_width
                    },
                    rotation_matrix,
                )
            })
        };
        Some(MarkerRenderer::new(
            get_point_vector(self.ra, self.dec, rotation_matrix),
            other_vec,
            &self.to_general_marker(),
            self.colour,
        ))
    }

    pub fn to_general_marker(&self) -> Marker {
        Marker {
            ra: self.ra,
            dec: self.dec,
            line_width: self.line_width,
            angular_radius: self.angular_radius,
            pixel_radius: self.pixel_radius,
            angular_width: self.angular_width,
            pixel_width: self.pixel_width,
        }
    }

    pub fn get_colour(marker_type: GameMarkerType, game_markers_colours: &GameMarkersColours) -> Color32 {
        match marker_type {
            GameMarkerType::Exact => game_markers_colours.exact,
            GameMarkerType::Tolerance => game_markers_colours.tolerance,
            GameMarkerType::Task => game_markers_colours.task,
            GameMarkerType::CorrectAnswer => game_markers_colours.correct_answer,
        }
    }
}

#[derive(Clone, Copy)]
pub enum GameMarkerType {
    /// A marker showing the exact chosen location
    Exact,
    /// A marker indicating the tolerance of a guess
    Tolerance,
    /// A marker marking the task (an object to guess etc.)
    Task,
    /// A marker marking the correct answer
    CorrectAnswer,
}

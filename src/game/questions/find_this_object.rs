use crate::game::game_handler;
use crate::game::game_handler::GameHandler;
use crate::geometry;
use crate::renderer::CellestialSphere;
use crate::rendering::caspr::markers::game_markers::{GameMarker, GameMarkerType};
use crate::rendering::themes::Theme;
use angle::{Angle, Deg};
use rand::Rng;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Settings {
    pub rotate_to_correct_point: bool,
    pub show_messiers: bool,
    pub show_caldwells: bool,
    pub show_ngcs: bool,
    pub show_ics: bool,
    pub show_bayer: bool,
    pub show_starnames: bool,
    pub magnitude_cutoff: f32,
    pub correctness_threshold: angle::Deg<f32>,
    pub replay_incorrect: bool,
    pub show: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            rotate_to_correct_point: true,
            show_messiers: true,
            show_caldwells: true,
            show_ngcs: true,
            show_ics: true,
            show_bayer: true,
            show_starnames: true,
            magnitude_cutoff: 6.0,
            correctness_threshold: angle::Deg(0.2),
            replay_incorrect: true,
            show: true,
        }
    }
}

#[derive(Clone)]
pub struct Question {
    pub name: String,
    pub ra: angle::Deg<f32>,
    pub dec: angle::Deg<f32>,
    pub is_messier: bool,
    pub is_caldwell: bool,
    pub is_ngc: bool,
    pub is_ic: bool,
    pub is_bayer: bool,
    pub is_starname: bool,
    pub magnitude: Option<f32>,
    pub object_type: String,
    pub constellation_abbreviation: String,
    pub images: Vec<crate::structs::image_info::ImageInfo>,
}

impl crate::game::game_handler::Question for Question {
    fn check_answer(&self, game_handler: &mut GameHandler, cellestial_sphere: &mut CellestialSphere, theme: &Theme) {
        let markers = &mut cellestial_sphere.game_markers.markers;
        game_handler.increment_possible_score(3);
        let mut correct = false;
        if !self.images.is_empty() {
            game_handler.answer_image = Some(self.images[rand::thread_rng().gen_range(0..self.images.len())].clone());
        }
        let (answer_dec_text, answer_ra_text, distance, answer_review_text_heading) = if !markers.is_empty() {
            let answer_dec = markers[0].dec;
            let answer_ra = markers[0].ra;
            let distance = geometry::angular_distance((self.ra.to_rad(), self.dec.to_rad()), (answer_ra.to_rad(), answer_dec.to_rad())).to_deg();
            if game_handler.game_settings.is_scored_mode {
                game_handler.score += GameHandler::evaluate_score(distance);
            }
            (
                answer_dec.value().to_string(),
                answer_ra.value().to_string(),
                distance.value().to_string(),
                if distance < game_handler.questions_settings.find_this_object.correctness_threshold {
                    correct = true;
                    String::from("Correct!")
                } else {
                    format!("You were {} degrees away from {} !", (distance.value() * 100.0).round() / 100.0, self.name)
                },
            )
        } else {
            (String::from("-"), String::from("-"), String::from("-"), format!("You didn't guess where {} is", self.name))
        };
        game_handler.answer_review_text_heading = answer_review_text_heading;
        game_handler.answer_review_text = format!(
            "Your coordinates: [dec = {}°; ra = {}°]\nCorrect coordinates: [dec = {}°; ra = {}°]\nFully precise distance: {}°\nYou can see the correct place marked with a new {}.\nObject type: {}",
            answer_dec_text,
            answer_ra_text,
            self.dec.value(),
            self.ra.value(),
            distance,
            if self.is_bayer || self.is_starname { "circle" } else { "cross" },
            self.object_type
        );
        markers.push(GameMarker::new(
            GameMarkerType::CorrectAnswer,
            self.ra,
            self.dec,
            2.0,
            5.0,
            self.is_bayer || self.is_starname,
            false,
            &theme.game_visuals.game_markers_colours,
        ));
        if !game_handler.questions_settings.find_this_object.replay_incorrect || correct {
            game_handler.use_up_current_question();
        } else {
            game_handler.question_number += 1;
        }
        if game_handler.questions_settings.find_this_object.rotate_to_correct_point {
            let final_vector = geometry::get_point_vector(self.ra, self.dec, &nalgebra::Matrix3::<f32>::identity());
            cellestial_sphere.look_at_point(&final_vector);
            cellestial_sphere.init_renderers();
        }
    }

    fn can_choose_as_next(&self, game_handler: &mut GameHandler) -> bool {
        let mag = (self.magnitude).unwrap_or(-1.0); // TODO: Shouldn't a default magnitude be something else?
        game_handler.questions_settings.find_this_object.show
            && ((game_handler.questions_settings.find_this_object.show_messiers && self.is_messier)
                || (game_handler.questions_settings.find_this_object.show_caldwells && self.is_caldwell)
                || (game_handler.questions_settings.find_this_object.show_ngcs && self.is_ngc)
                || (game_handler.questions_settings.find_this_object.show_ics && self.is_ic)
                || (game_handler.questions_settings.find_this_object.show_bayer && self.is_bayer)
                || (game_handler.questions_settings.find_this_object.show_starnames && self.is_starname))
            && ((!self.is_bayer && !self.is_starname) || mag < game_handler.questions_settings.find_this_object.magnitude_cutoff)
            && *game_handler.active_constellations.entry(self.constellation_abbreviation.to_lowercase()).or_insert(true)
    }

    fn reset(self) -> Box<dyn game_handler::Question> {
        Box::new(self)
    }

    fn show_tolerance_marker(&self) -> bool {
        true
    }

    fn show_circle_marker(&self) -> bool {
        self.is_bayer || self.is_starname
    }

    fn get_question_distance_tolerance(&self, game_handler: &GameHandler) -> Deg<f32> {
        game_handler.questions_settings.find_this_object.correctness_threshold
    }

    fn allow_multiple_player_markers(&self) -> bool {
        false
    }

    fn add_marker_on_click(&self) -> bool {
        true
    }

    fn should_display_input(&self) -> bool {
        false
    }

    fn start_question(&self, _game_handler: &mut GameHandler, cellestial_sphere: &mut CellestialSphere, _theme: &Theme) {
        cellestial_sphere.game_markers.markers = Vec::new();
    }

    fn get_display_question(&self) -> String {
        format!("Find {}.", self.name)
    }

    fn clone_box(&self) -> Box<dyn game_handler::Question> {
        Box::new(self.clone())
    }
}

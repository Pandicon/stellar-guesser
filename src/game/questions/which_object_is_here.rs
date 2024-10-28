use crate::game::game_handler;
use crate::game::game_handler::GameHandler;
use crate::geometry;
use crate::renderer::CellestialSphere;
use crate::rendering::caspr::markers::game_markers::{GameMarker, GameMarkerType};
use crate::rendering::themes::Theme;
use angle::Deg;
use rand::Rng;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Settings {
    pub rotate_to_point: bool,
    pub show_messiers: bool,
    pub show_caldwells: bool,
    pub show_ngcs: bool,
    pub show_ics: bool,
    pub show_bayer: bool,
    pub show_starnames: bool,
    pub magnitude_cutoff: f32,
    pub correctness_threshold: f32,
    pub replay_incorrect: bool,
    pub show: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            rotate_to_point: true,
            show_messiers: true,
            show_caldwells: true,
            show_ngcs: true,
            show_ics: true,
            show_bayer: true,
            show_starnames: true,
            magnitude_cutoff: 6.0,
            correctness_threshold: 0.2,
            replay_incorrect: true,
            show: true,
        }
    }
}

#[derive(Clone, Default)]
pub struct State {
    answer: String,
}

#[derive(Clone)]
pub struct Question {
    pub possible_names: Vec<String>,
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

    pub state: State,
}

impl crate::game::game_handler::Question for Question {
    fn check_answer(&self, game_handler: &mut GameHandler, _cellestial_sphere: &mut CellestialSphere, _theme: &Theme) {
        if !self.images.is_empty() {
            game_handler.answer_image = Some(self.images[rand::thread_rng().gen_range(0..self.images.len())].clone());
        }
        let possible_names_edited = self.possible_names.iter().map(|name| name.replace(' ', "").to_lowercase()).collect::<Vec<String>>();
        let correct = possible_names_edited.contains(&self.state.answer.replace(' ', "").to_lowercase());
        game_handler.answer_review_text_heading = format!(
            "{}orrect!",
            if correct {
                game_handler.score += 1;
                "C"
            } else {
                "Inc"
            }
        );
        game_handler.answer_review_text = format!(
            "Your answer was: {}\nPossible answers: {}\nObject type: {}",
            self.state.answer,
            self.possible_names.join(", "),
            self.object_type
        );
        game_handler.increment_possible_score(1);
        if !game_handler.questions_settings.what_is_this_object.replay_incorrect || correct {
            game_handler.use_up_current_question();
        } else {
            game_handler.question_number += 1;
        }
    }

    fn can_choose_as_next(&self, game_handler: &mut GameHandler) -> bool {
        let mag = (self.magnitude).unwrap_or(-1.0);
        game_handler.questions_settings.what_is_this_object.show
            && ((game_handler.questions_settings.what_is_this_object.show_messiers && self.is_messier)
                || (game_handler.questions_settings.what_is_this_object.show_caldwells && self.is_caldwell)
                || (game_handler.questions_settings.what_is_this_object.show_ngcs && self.is_ngc)
                || (game_handler.questions_settings.what_is_this_object.show_ics && self.is_ic)
                || (game_handler.questions_settings.what_is_this_object.show_bayer && self.is_bayer)
                || (game_handler.questions_settings.what_is_this_object.show_starnames && self.is_starname))
            && ((!self.is_bayer && !self.is_starname) || mag < game_handler.questions_settings.what_is_this_object.magnitude_cutoff)
            && *game_handler.active_constellations.entry(self.constellation_abbreviation.to_lowercase()).or_insert(true)
    }

    fn reset(self) -> Box<dyn game_handler::Question> {
        Box::new(Self {
            possible_names: self.possible_names,
            ra: self.ra,
            dec: self.dec,
            is_messier: self.is_messier,
            is_caldwell: self.is_caldwell,
            is_ngc: self.is_ngc,
            is_ic: self.is_ic,
            is_bayer: self.is_bayer,
            is_starname: self.is_starname,
            magnitude: self.magnitude,
            object_type: self.object_type,
            constellation_abbreviation: self.constellation_abbreviation,
            images: self.images,

            state: State::default(),
        })
    }

    fn show_tolerance_marker(&self) -> bool {
        false
    }

    fn show_circle_marker(&self) -> bool {
        self.is_bayer || self.is_starname
    }

    fn get_question_distance_tolerance(&self, _game_handler: &GameHandler) -> Deg<f32> {
        angle::Deg(0.0)
    }

    fn allow_multiple_player_markers(&self) -> bool {
        false
    }

    fn add_marker_on_click(&self) -> bool {
        false
    }

    fn should_display_input(&self) -> bool {
        true
    }

    fn start_question(&self, game_handler: &mut GameHandler, cellestial_sphere: &mut CellestialSphere, theme: &Theme) {
        cellestial_sphere.game_markers.markers = if self.is_bayer || self.is_starname {
            vec![GameMarker::new(
                GameMarkerType::Task,
                self.ra,
                self.dec,
                2.0,
                5.0,
                true,
                false,
                &theme.game_visuals.game_markers_colours,
            )]
        } else {
            vec![GameMarker::new(
                GameMarkerType::Task,
                self.ra,
                self.dec,
                2.0,
                5.0,
                false,
                false,
                &theme.game_visuals.game_markers_colours,
            )]
        };
        if game_handler.questions_settings.what_is_this_object.rotate_to_point {
            let final_vector = geometry::get_point_vector(self.ra, self.dec, &nalgebra::Matrix3::<f32>::identity());
            cellestial_sphere.look_at_point(&final_vector);
            cellestial_sphere.init_renderers();
        }
    }

    fn get_display_question(&self) -> String {
        String::from("What is this object?")
    }

    fn clone_box(&self) -> Box<dyn game_handler::Question> {
        Box::new(self.clone())
    }
}

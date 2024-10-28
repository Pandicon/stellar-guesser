use crate::game::game_handler;
use crate::game::game_handler::GameHandler;
use crate::geometry;
use crate::renderer::CellestialSphere;
use crate::rendering::caspr::markers::game_markers::{GameMarker, GameMarkerType};
use crate::rendering::themes::Theme;
use angle::{Angle, Deg};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Settings {
    pub show: bool,
    pub rotate_to_point: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { show: true, rotate_to_point: true }
    }
}

#[derive(Clone, Default)]
pub struct State {
    answer: String,
}

#[derive(Clone)]
pub struct Question {
    pub ra: angle::Deg<f32>,
    pub dec: angle::Deg<f32>,

    pub state: State,
}

impl Question {
    pub fn new_random() -> Self {
        let (ra, dec) = geometry::generate_random_point(&mut rand::thread_rng());
        Self { ra, dec, state: State::default() }
    }
}

impl crate::game::game_handler::Question for Question {
    fn check_answer(&self, game_handler: &mut GameHandler, cellestial_sphere: &mut CellestialSphere, _theme: &Theme) {
        let possible_abbrevs = cellestial_sphere.determine_constellation((self.ra.to_rad(), self.dec.to_rad()));
        let mut possible_constellation_names = Vec::new();
        for abbrev in possible_abbrevs {
            if let Some(constellation) = cellestial_sphere.constellations.get(&abbrev) {
                possible_constellation_names.extend(constellation.possible_names.iter().map(|name| name.replace(' ', "").to_lowercase()));
            };
        }
        let correct = possible_constellation_names.contains(&self.state.answer.replace(' ', "").to_lowercase());
        game_handler.answer_review_text_heading = format!(
            "{}orrect!",
            if correct {
                game_handler.score += 1;
                "C"
            } else {
                "Inc"
            }
        );
        game_handler.increment_possible_score(1);
        game_handler.answer_review_text = format!("Your answer was: {}\nThe right answers were: {}", self.state.answer, possible_constellation_names.join(", "));
        game_handler.use_up_current_question();
    }

    fn can_choose_as_next(&self, game_handler: &mut GameHandler) -> bool {
        game_handler.questions_settings.what_constellation_is_this_point_in.show
    }

    fn reset(self) -> Box<dyn game_handler::Question> {
        Box::new(Self::new_random())
    }

    fn show_tolerance_marker(&self) -> bool {
        false
    }

    fn show_circle_marker(&self) -> bool {
        false
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
        cellestial_sphere.game_markers.markers = vec![GameMarker::new(
            GameMarkerType::Task,
            self.ra,
            self.dec,
            2.0,
            5.0,
            false,
            false,
            &theme.game_visuals.game_markers_colours,
        )];
        if game_handler.questions_settings.what_constellation_is_this_point_in.rotate_to_point {
            let final_vector = geometry::get_point_vector(self.ra, self.dec, &nalgebra::Matrix3::<f32>::identity());
            cellestial_sphere.look_at_point(&final_vector);
            cellestial_sphere.init_renderers();
        }
    }

    fn get_display_question(&self) -> String {
        String::from("What constellation does this point lie in?")
    }

    fn clone_box(&self) -> Box<dyn game_handler::Question> {
        Box::new(self.clone())
    }
}

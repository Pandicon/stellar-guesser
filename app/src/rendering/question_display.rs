use crate::game::game_handler::QuestionWindowData;
use crate::{enums::GameStage, Application};
use eframe::egui;

impl Application {
    pub fn render_question_window(&mut self, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>> {
        match self.game_handler.stage {
            GameStage::NotStartedYet => egui::Window::new("Question").open(&mut self.state.windows.game_question.opened).show(ctx, |ui| {
                ui.heading("Welcome!");
                if ui.button("Start").clicked() {
                    self.game_handler.stage = GameStage::Checked;
                    self.game_handler.next_question(&mut self.cellestial_sphere, &self.theme)
                }
            }),
            GameStage::NoMoreQuestions => egui::Window::new("Question").open(&mut self.state.windows.game_question.opened).show(ctx, |ui| {
                if self.game_handler.question_catalog.is_empty() {
                    ui.heading("Question pack is empty");
                    ui.label("There are no questions to be chosen from as this question pack is empty. You have to choose a different one from the game settings.");
                } else {
                    ui.heading("No more questions left");
                    ui.label("There are no more questions to be chosen from. You can either choose a different question pack from the game settings, or return to the questions you already went through by clicking 'Reset'.");
                }
                ui.horizontal(|ui| {
                    if !self.game_handler.question_catalog.is_empty() {
                        if ui.button("Reset").clicked() {
                            self.game_handler.reset_used_questions(&mut self.cellestial_sphere);
                            self.game_handler.next_question(&mut self.cellestial_sphere, &self.theme);
                        }
                    }                   
                    if ui.button("Choose a different question pack").clicked() {
                        self.state.windows.settings.opened = true;
                        self.state.windows.settings.subwindow = crate::structs::state::windows::settings::SettingsSubWindow::Game;
                        self.state.windows.settings.game_settings.subwindow = crate::structs::state::windows::settings::GameSettingsSubWindow::Questions;
                    }
                });

                ui.label(&self.game_handler.question_number_text);
            }),
            GameStage::ScoredModeFinished => egui::Window::new("Question").open(&mut self.state.windows.game_question.opened).show(ctx, |ui| {
                ui.heading("Game over!");
                let percentage = (self.game_handler.score as f32) / (self.game_handler.get_possible_score() as f32) * 100.0;
                ui.label(format!(
                    "Game over! Your score was {}/{}, that is {:.1}% of the maximum. Click 'Reset' if you want to play a new game!",
                    self.game_handler.score, self.game_handler.get_possible_score(), percentage
                ));
                ui.horizontal(|ui| {
                    if ui.button("Reset").clicked() {
                        self.game_handler.reset_used_questions(&mut self.cellestial_sphere);
                        self.game_handler.next_question(&mut self.cellestial_sphere, &self.theme);
                    }
                });

                ui.label(&self.game_handler.question_number_text);
            }),
            GameStage::Guessing | GameStage::Checked => {
                let data = QuestionWindowData {
                    cellestial_sphere: &mut self.cellestial_sphere,
                    theme: &self.theme,
                    game_question_opened: &mut self.state.windows.game_question.opened,
                    request_input_focus: &mut self.game_handler.request_input_focus,
                    add_marker_on_click: &mut self.game_handler.add_marker_on_click,
                    question_number_text: &self.game_handler.question_number_text,
                    game_stage: &mut self.game_handler.stage,
                    ctx,
                    start_next_question: &mut self.game_handler.switch_to_next_question,
                    score: &mut self.game_handler.score,
                    possible_score: &mut self.game_handler.possible_score,
                    is_scored_mode: self.game_handler.game_settings.is_scored_mode,
                    current_question: self.game_handler.current_question,
                    used_questions: &mut self.game_handler.used_questions,
                    questions_settings: &self.game_handler.questions_settings,
                    question_number: &mut self.game_handler.question_number
                };
                self.game_handler.question_catalog[self.game_handler.current_question].render_window(data)
            }
        }
    }
}

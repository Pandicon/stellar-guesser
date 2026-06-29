use crate::game::questions;
use crate::{enums::GameStage, Application};
use eframe::egui;

impl Application {
    pub fn render_question_window(&mut self, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>> {
        match self.game_handler.stage {
            GameStage::NotStartedYet => egui::Window::new("Question").open(&mut self.state.windows.game_question.opened).show(ctx, |ui| {
                ui.heading("Welcome!");
                if ui.button("Start").clicked() {
                    self.game_handler.stage = GameStage::Checked;
                    self.game_handler.next_question(&self.theme, &mut self.actions)
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
                    if !self.game_handler.question_catalog.is_empty() && ui.button("Reset").clicked() {
                        self.game_handler.reset_used_questions();
                        self.game_handler.next_question(&self.theme, &mut self.actions);
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
                        self.game_handler.reset_used_questions();
                        self.game_handler.next_question(&self.theme, &mut self.actions);
                    }
                });

                ui.label(&self.game_handler.question_number_text);
            }),
            GameStage::Guessing | GameStage::Checked => {
                match &mut self.game_handler.active_question {
                    None => {
                        None
                    }
                    Some(questions::ActiveQuestion::AngularSeparation(active_question)) => active_question.render_window(ctx, self.game_handler.stage, self.state.windows.game_question.opened, self.game_handler.request_input_focus, &self.game_handler.question_number_text, &mut self.actions),
                    Some(questions::ActiveQuestion::FindThisObject(active_question)) => active_question.render_window(ctx, self.game_handler.stage, self.state.windows.game_question.opened, &self.game_handler.question_number_text, &mut self.actions),
                    Some(questions::ActiveQuestion::GuessDec(active_question)) => active_question.render_window(ctx, self.game_handler.stage, self.state.windows.game_question.opened, self.game_handler.request_input_focus, &self.game_handler.question_number_text, &mut self.actions),
                    Some(questions::ActiveQuestion::GuessRa(active_question)) => active_question.render_window(ctx, self.game_handler.stage, self.state.windows.game_question.opened, self.game_handler.request_input_focus, &self.game_handler.question_number_text, &mut self.actions),
                    Some(questions::ActiveQuestion::GuessTheMagnitude(active_question)) => active_question.render_window(ctx, self.game_handler.stage, self.state.windows.game_question.opened, self.game_handler.request_input_focus, &self.game_handler.question_number_text, &mut self.actions),
                    Some(questions::ActiveQuestion::MarkMissingObject(active_question)) => active_question.render_window(ctx, self.game_handler.stage, self.state.windows.game_question.opened, &self.game_handler.question_number_text, &mut self.actions),
                    Some(questions::ActiveQuestion::WhatIsThisObject(active_question)) => active_question.render_window(ctx, self.game_handler.stage, self.state.windows.game_question.opened, self.game_handler.request_input_focus, &self.game_handler.question_number_text, &mut self.actions),
                    Some(questions::ActiveQuestion::WhichConstellationIsThisPointIn(active_question)) => active_question.render_window(ctx, self.game_handler.stage, self.state.windows.game_question.opened, self.game_handler.request_input_focus, &self.game_handler.question_number_text, &mut self.actions),
                    Some(questions::ActiveQuestion::WhichObjectIsMissing(active_question)) => active_question.render_window(ctx, self.game_handler.stage, self.state.windows.game_question.opened, self.game_handler.request_input_focus, &self.game_handler.question_number_text, &mut self.actions),
                }
            }
        }
    }
}

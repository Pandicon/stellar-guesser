use eframe::egui;
use egui::epaint::Pos2;
use std::collections::HashMap;

use crate::game::game_handler::QuestionCheckingData;
use crate::{
    enums::{self, GameStage, PointerPosition, RendererCategory},
    Application,
};
const KEY_COMBINATIONS: [&str; 6] = ["alt+shift+g", "alt+shift+i", "alt+shift+o", "alt+shift+s", "mouse-middle", "space"];

impl Application {
    pub fn handle_input(&mut self, cursor_within_central_panel: bool, ctx: &egui::Context) {
        self.input.handle(cursor_within_central_panel, ctx);
        for input_to_handle in &self.input.to_handle {
            match input_to_handle {
                enums::Inputs::AltShiftI => self.state.windows.app_info.opened = !self.state.windows.app_info.opened,
                enums::Inputs::AltShiftO => self.state.windows.settings.opened = !self.state.windows.settings.opened,
                enums::Inputs::AltShiftS => self.state.windows.stats.opened = !self.state.windows.stats.opened,
                enums::Inputs::Space | enums::Inputs::MouseMiddle => {
                    if matches!(*input_to_handle, enums::Inputs::Space) && ctx.wants_keyboard_input() {
                        continue;
                    }
                    if !self.game_handler.no_more_questions() {
                        match self.game_handler.stage {
                            GameStage::Guessing | GameStage::Checked => {
                                if (self.game_handler.stage == GameStage::Guessing && !self.game_handler.should_display_input()) || self.game_handler.stage == GameStage::Checked {
                                    self.game_handler.question_catalog[self.game_handler.current_question].generic_to_next_part(QuestionCheckingData {
                                        cellestial_sphere: &mut self.cellestial_sphere,
                                        theme: &self.theme,
                                        game_stage: &mut self.game_handler.stage,
                                        score: &mut self.game_handler.score,
                                        possible_score: &mut self.game_handler.possible_score,
                                        is_scored_mode: self.game_handler.game_settings.is_scored_mode,
                                        current_question: self.game_handler.current_question,
                                        used_questions: &mut self.game_handler.used_questions,
                                        add_marker_on_click: &mut self.game_handler.add_marker_on_click,
                                        questions_settings: &self.game_handler.questions_settings,
                                        question_number: &mut self.game_handler.question_number,
                                        start_next_question: &mut self.game_handler.switch_to_next_question,
                                    });
                                }
                            }
                            GameStage::NotStartedYet => unimplemented!(),
                            GameStage::NoMoreQuestions | GameStage::ScoredModeFinished => {}
                        }
                    }
                }
            }
        }
        let reinitialise_stars = self.cellestial_sphere.zoom(self.input.zoom);

        let pointer_position: Pos2 = match self.input.pointer_position {
            PointerPosition::OnScreen(position) => position,
            PointerPosition::OffScreen => return,
        };
        let all_reinitialised = if cursor_within_central_panel {
            let mut all_reinitialised = false;
            if self.game_handler.add_marker_on_click && self.input.primary_released && !self.input.primary_dragging_last_frame {
                /*let sphere_position = geometry::cast_onto_sphere(&self.cellestial_sphere, &pointer_position);
                let (dec, ra) = geometry::cartesian_to_spherical(sphere_position);*/
                let marker_pos = sg_geometry::cast_onto_sphere_dec_ra(
                    &self.cellestial_sphere.viewport_rect,
                    &pointer_position,
                    self.cellestial_sphere.rotation,
                    self.cellestial_sphere.get_zoom(),
                );
                if self.game_handler.allow_multiple_player_marker() {
                    self.game_handler.guess_marker_positions.push(marker_pos);
                } else {
                    self.game_handler.guess_marker_positions = vec![marker_pos];
                }
                let new_markers = self.game_handler.generate_player_markers(&self.game_handler.guess_marker_positions, &self.theme);
                self.cellestial_sphere.game_markers.markers = new_markers; // vec![Marker::new(ra / PI * 180.0, dec / PI * 180.0, Color32::RED, 2.0, 5.0, self.game_handler.show_circle_marker(), false)];
                self.cellestial_sphere.init_single_renderer(RendererCategory::Markers, "game");
            }
            let initial_vector = self.cellestial_sphere.project_screen_pos(pointer_position - self.input.dragged);
            let final_vector = self.cellestial_sphere.project_screen_pos(pointer_position);

            if initial_vector != final_vector {
                // Some rotation this frame

                self.cellestial_sphere.rotate_between_points(&initial_vector, &final_vector);
                self.cellestial_sphere.init_renderers();
                all_reinitialised = true;
            }
            all_reinitialised
        } else {
            false
        };
        if !all_reinitialised && reinitialise_stars {
            self.cellestial_sphere.reinit_renderer_category(RendererCategory::Stars);
        }
    }
}

pub struct Input {
    pub dragged: egui::Vec2,
    pub pointer_position: PointerPosition,
    pub to_handle: Vec<enums::Inputs>,
    pub zoom: f32,
    pub secondary_released: bool,
    pub primary_clicked: bool,
    pub primary_down: bool,
    pub primary_released: bool,
    pub primary_dragging: bool,
    pub primary_dragging_last_frame: bool,

    pointer_down_outside_subwindow: bool,
    currently_held: HashMap<&'static str, bool>,

    pub text_from_keys: String,
    pub input_field_has_focus: bool,
    pub input_field_had_focus_last_frame: bool,
}

impl Default for Input {
    fn default() -> Self {
        let mut currently_held: HashMap<&str, bool> = HashMap::new();
        for combination in KEY_COMBINATIONS {
            currently_held.insert(combination, false);
        }
        Self {
            dragged: egui::Vec2::new(0.0, 0.0),
            pointer_position: PointerPosition::OnScreen(egui::Pos2::new(0.0, 0.0)),
            to_handle: Vec::new(),
            zoom: 1.0,

            pointer_down_outside_subwindow: false,
            currently_held,
            secondary_released: false,
            primary_clicked: false,
            primary_down: false,
            primary_released: false,
            primary_dragging: false,
            primary_dragging_last_frame: false,

            text_from_keys: String::new(),
            input_field_has_focus: false,
            input_field_had_focus_last_frame: false,
        }
    }
}

impl Input {
    pub fn handle(&mut self, cursor_within_central_panel: bool, ctx: &egui::Context) {
        self.text_from_keys = String::new();
        let input_events = ctx.input(|i| i.events.clone());
        let shift_held = ctx.input(|i| i.modifiers.shift);
        let drag_x = ctx.input(|i: &egui::InputState| i.pointer.delta().x);
        let drag_y = ctx.input(|i| i.pointer.delta().y);
        let primary_down = ctx.input(|i| i.pointer.primary_down());
        self.primary_clicked = ctx.input(|i| i.pointer.primary_clicked());
        self.primary_released = ctx.input(|i| i.pointer.primary_released());
        self.secondary_released = ctx.input(|i| i.pointer.secondary_released());
        self.primary_down = primary_down;
        self.primary_dragging_last_frame = self.primary_dragging;
        self.primary_dragging = primary_down && ctx.input(|i| i.pointer.is_decidedly_dragging());
        if ctx.is_pointer_over_area() {
            self.pointer_position = PointerPosition::OnScreen(ctx.input(|i| i.pointer.hover_pos().unwrap_or(egui::pos2(0.0, 0.0))));
        } else {
            self.pointer_position = PointerPosition::OffScreen;
        }
        if self.pointer_down_outside_subwindow && primary_down && ctx.input(|i| i.pointer.is_decidedly_dragging()) {
            // Ignore drags that started in a subwindow
            if shift_held {
                if drag_x.abs() >= drag_y.abs() {
                    self.dragged.x = drag_x;
                    self.dragged.y = 0.0;
                } else {
                    self.dragged.x = 0.0;
                    self.dragged.y = drag_y;
                }
            } else {
                self.dragged.x = drag_x;
                self.dragged.y = drag_y;
            }
        } else {
            self.dragged.x = 0.0;
            self.dragged.y = 0.0;
        }
        if cursor_within_central_panel || !primary_down {
            self.pointer_down_outside_subwindow = primary_down;
        }
        let mut to_handle: Vec<enums::Inputs> = Vec::new();
        let middle_down = ctx.input(|i| i.pointer.middle_down());

        if let Some(pressed) = self.currently_held.get("mouse-middle") {
            let pressed = *pressed;
            let held = self.currently_held.entry("mouse-middle").or_insert(true);
            if !pressed && middle_down {
                *held = true;
                to_handle.push(enums::Inputs::MouseMiddle);
            } else if pressed && !middle_down {
                *held = false;
            }
        } else {
            println!("The mouse-middle combination was not in the 'currently_held' hashmap");
        }
        self.zoom = 0.0;
        let mut tap_position = Pos2::new(0.0, 0.0);
        let mut tap_released = false;
        let mut touch_detected = false;
        for event in &input_events {
            match event {
                egui::Event::Zoom(zoom) => {
                    if *zoom > 1.0 {
                        self.zoom = 0.2;
                    } else if *zoom < 1.0 {
                        self.zoom = -0.2;
                    }
                }
                egui::Event::MouseWheel { delta, unit, .. } => {
                    let divider = match unit {
                        egui::MouseWheelUnit::Point => 500.0,
                        egui::MouseWheelUnit::Line => 25.0,
                        egui::MouseWheelUnit::Page => 0.5,
                    };
                    self.zoom = delta.y / divider;
                }
                egui::Event::Touch {
                    device_id: _,
                    id: _,
                    phase,
                    pos,
                    force: _,
                } => {
                    touch_detected = true;
                    if phase == &egui::TouchPhase::End {
                        tap_released = true;
                    }
                    tap_position = *pos;
                }
                // Press of Shift + UpArrow
                egui::Event::Key {
                    key: egui::Key::ArrowUp,
                    physical_key: _,
                    pressed: true,
                    repeat: false,
                    modifiers:
                        egui::Modifiers {
                            alt: _,
                            ctrl: _,
                            shift: true,
                            mac_cmd: _,
                            command: _,
                        },
                } => {
                    self.zoom = 1.28;
                }
                // Press of Shift + DownArrow
                egui::Event::Key {
                    key: egui::Key::ArrowDown,
                    physical_key: _,
                    pressed: true,
                    repeat: false,
                    modifiers:
                        egui::Modifiers {
                            alt: _,
                            ctrl: _,
                            shift: true,
                            mac_cmd: _,
                            command: _,
                        },
                } => {
                    self.zoom = 0.75;
                }
                // Press of LeftArrow
                egui::Event::Key {
                    key: egui::Key::ArrowLeft,
                    physical_key: _,
                    pressed: true,
                    repeat: _,
                    modifiers:
                        egui::Modifiers {
                            alt: _,
                            ctrl: _,
                            shift: _,
                            mac_cmd: _,
                            command: _,
                        },
                } => {
                    self.dragged.x -= 1.0;
                }
                // Press of RightArrow
                egui::Event::Key {
                    key: egui::Key::ArrowRight,
                    physical_key: _,
                    pressed: true,
                    repeat: _,
                    modifiers:
                        egui::Modifiers {
                            alt: _,
                            ctrl: _,
                            shift: _,
                            mac_cmd: _,
                            command: _,
                        },
                } => {
                    self.dragged.x += 1.0;
                }
                // Press of UpArrow
                egui::Event::Key {
                    key: egui::Key::ArrowUp,
                    physical_key: _,
                    pressed: true,
                    repeat: _,
                    modifiers:
                        egui::Modifiers {
                            alt: _,
                            ctrl: _,
                            shift: _,
                            mac_cmd: _,
                            command: _,
                        },
                } => {
                    self.dragged.y -= 1.0;
                }
                // Press of DownArrow
                egui::Event::Key {
                    key: egui::Key::ArrowDown,
                    physical_key: _,
                    pressed: true,
                    repeat: _,
                    modifiers:
                        egui::Modifiers {
                            alt: _,
                            ctrl: _,
                            shift: _,
                            mac_cmd: _,
                            command: _,
                        },
                } => {
                    self.dragged.y += 1.0;
                }
                // Press of Alt + Shift + I
                egui::Event::Key {
                    key: egui::Key::I,
                    physical_key: _,
                    pressed: true,
                    repeat: false,
                    modifiers:
                        egui::Modifiers {
                            alt: true,
                            ctrl: _,
                            shift: true,
                            mac_cmd: _,
                            command: _,
                        },
                } => {
                    if let Some(pressed) = self.currently_held.get("alt+shift+i") {
                        if !pressed {
                            let held = self.currently_held.entry("alt+shift+i").or_insert(true);
                            *held = true;
                            to_handle.push(enums::Inputs::AltShiftI);
                        }
                    } else {
                        println!("The alt+shift+i combination was not in the 'currently_held' hashmap");
                    }
                }
                // Press of Alt + Shift + O
                egui::Event::Key {
                    key: egui::Key::O,
                    physical_key: _,
                    pressed: true,
                    repeat: false,
                    modifiers:
                        egui::Modifiers {
                            alt: true,
                            ctrl: _,
                            shift: true,
                            mac_cmd: _,
                            command: _,
                        },
                } => {
                    if let Some(pressed) = self.currently_held.get("alt+shift+o") {
                        if !pressed {
                            let held = self.currently_held.entry("alt+shift+o").or_insert(true);
                            *held = true;
                            to_handle.push(enums::Inputs::AltShiftO);
                        }
                    } else {
                        println!("The alt+shift+o combination was not in the 'currently_held' hashmap");
                    }
                }
                // Press of Alt + Shift + S
                egui::Event::Key {
                    key: egui::Key::S,
                    physical_key: _,
                    pressed: true,
                    repeat: false,
                    modifiers:
                        egui::Modifiers {
                            alt: true,
                            ctrl: _,
                            shift: true,
                            mac_cmd: _,
                            command: _,
                        },
                } => {
                    if let Some(pressed) = self.currently_held.get("alt+shift+s") {
                        if !pressed {
                            let held = self.currently_held.entry("alt+shift+s").or_insert(true);
                            *held = true;
                            to_handle.push(enums::Inputs::AltShiftS);
                        }
                    } else {
                        println!("The alt+shift+s combination was not in the 'currently_held' hashmap");
                    }
                }
                // All unhandled, unmodified keys - construct the text edit string by hand
                #[cfg(any(target_os = "ios", target_os = "android"))]
                egui::Event::Key {
                    key,
                    physical_key: _,
                    pressed: true,
                    repeat: false,
                    modifiers:
                        egui::Modifiers {
                            alt: false,
                            ctrl: false,
                            shift,
                            mac_cmd: false,
                            command: false,
                        },
                } => {
                    let mut character = match *key {
                        egui::Key::Enter => "\n",
                        egui::Key::Space => " ",
                        egui::Key::Minus => "-",
                        egui::Key::Plus => "+",
                        egui::Key::Equals => "=",
                        egui::Key::Comma => ",",
                        egui::Key::Period => ".",
                        egui::Key::Colon => ":",
                        egui::Key::Semicolon => ";",
                        egui::Key::Questionmark => "?",
                        egui::Key::Backtick => "`",
                        egui::Key::Quote => "'",
                        egui::Key::Pipe => "|",
                        egui::Key::Slash => "/",
                        egui::Key::Backslash => "\\",
                        egui::Key::OpenBracket => "(",
                        egui::Key::CloseBracket => ")",
                        egui::Key::Num0 => "0",
                        egui::Key::Num1 => "1",
                        egui::Key::Num2 => "2",
                        egui::Key::Num3 => "3",
                        egui::Key::Num4 => "4",
                        egui::Key::Num5 => "5",
                        egui::Key::Num6 => "6",
                        egui::Key::Num7 => "7",
                        egui::Key::Num8 => "8",
                        egui::Key::Num9 => "9",
                        egui::Key::A => "a",
                        egui::Key::B => "b",
                        egui::Key::C => "c",
                        egui::Key::D => "d",
                        egui::Key::E => "e",
                        egui::Key::F => "f",
                        egui::Key::G => "g",
                        egui::Key::H => "h",
                        egui::Key::I => "i",
                        egui::Key::J => "j",
                        egui::Key::K => "k",
                        egui::Key::L => "l",
                        egui::Key::M => "m",
                        egui::Key::N => "n",
                        egui::Key::O => "o",
                        egui::Key::P => "p",
                        egui::Key::Q => "q",
                        egui::Key::R => "r",
                        egui::Key::S => "s",
                        egui::Key::T => "t",
                        egui::Key::U => "u",
                        egui::Key::V => "v",
                        egui::Key::W => "w",
                        egui::Key::X => "x",
                        egui::Key::Y => "y",
                        egui::Key::Z => "z",
                        egui::Key::ArrowDown
                        | egui::Key::ArrowLeft
                        | egui::Key::ArrowRight
                        | egui::Key::ArrowUp
                        | egui::Key::Escape
                        | egui::Key::Cut
                        | egui::Key::Copy
                        | egui::Key::Paste
                        | egui::Key::Tab
                        | egui::Key::Backspace
                        | egui::Key::Insert
                        | egui::Key::Delete
                        | egui::Key::Home
                        | egui::Key::End
                        | egui::Key::PageUp
                        | egui::Key::PageDown
                        | egui::Key::F1
                        | egui::Key::F2
                        | egui::Key::F3
                        | egui::Key::F4
                        | egui::Key::F5
                        | egui::Key::F6
                        | egui::Key::F7
                        | egui::Key::F8
                        | egui::Key::F9
                        | egui::Key::F10
                        | egui::Key::F11
                        | egui::Key::F12
                        | egui::Key::F13
                        | egui::Key::F14
                        | egui::Key::F15
                        | egui::Key::F16
                        | egui::Key::F17
                        | egui::Key::F18
                        | egui::Key::F19
                        | egui::Key::F20
                        | egui::Key::F21
                        | egui::Key::F22
                        | egui::Key::F23
                        | egui::Key::F24
                        | egui::Key::F25
                        | egui::Key::F26
                        | egui::Key::F27
                        | egui::Key::F28
                        | egui::Key::F29
                        | egui::Key::F30
                        | egui::Key::F31
                        | egui::Key::F32
                        | egui::Key::F33
                        | egui::Key::F34
                        | egui::Key::F35 => "",
                    };
                    // Could probably have a bit more fun with it, but having it doubled does work well enough...
                    if *key == egui::Key::Space && !self.input_field_has_focus {
                        // If you are typing, then you don't want this to fire
                        if let Some(pressed) = self.currently_held.get("space") {
                            if !pressed {
                                let held = self.currently_held.entry("space").or_insert(true);
                                *held = true;
                                to_handle.push(enums::Inputs::Space);
                            }
                        } else {
                            println!("The space combination was not in the 'currently_held' hashmap");
                        }
                    }
                    let c = character.to_uppercase();
                    if *shift {
                        character = &c;
                    }
                    self.text_from_keys += character;
                }
                // Press of Space
                egui::Event::Key {
                    key: egui::Key::Space,
                    physical_key: _,
                    pressed: true,
                    repeat: _,
                    modifiers:
                        egui::Modifiers {
                            alt: _,
                            ctrl: _,
                            shift: _,
                            mac_cmd: _,
                            command: _,
                        },
                } => {
                    if let Some(pressed) = self.currently_held.get("space") {
                        if !pressed {
                            let held = self.currently_held.entry("space").or_insert(true);
                            *held = true;
                            to_handle.push(enums::Inputs::Space);
                        }
                    } else {
                        println!("The space combination was not in the 'currently_held' hashmap");
                    }
                }
                // Release of G
                egui::Event::Key {
                    key: egui::Key::G,
                    physical_key: _,
                    pressed: false,
                    repeat: _,
                    modifiers:
                        egui::Modifiers {
                            alt: _,
                            ctrl: _,
                            shift: _,
                            mac_cmd: _,
                            command: _,
                        },
                } => {
                    if let Some(&pressed) = self.currently_held.get("alt+shift+g") {
                        if pressed {
                            let held = self.currently_held.entry("alt+shift+g").or_insert(false);
                            *held = false;
                        }
                    } else {
                        println!("The alt+shift+g combination was not in the 'currently_held' hashmap");
                    }
                }
                // Release of I
                egui::Event::Key {
                    key: egui::Key::I,
                    physical_key: _,
                    pressed: false,
                    repeat: _,
                    modifiers:
                        egui::Modifiers {
                            alt: _,
                            ctrl: _,
                            shift: _,
                            mac_cmd: _,
                            command: _,
                        },
                } => {
                    if let Some(&pressed) = self.currently_held.get("alt+shift+i") {
                        if pressed {
                            let held = self.currently_held.entry("alt+shift+i").or_insert(false);
                            *held = false;
                        }
                    } else {
                        println!("The alt+shift+i combination was not in the 'currently_held' hashmap");
                    }
                }
                // Release of O
                egui::Event::Key {
                    key: egui::Key::O,
                    physical_key: _,
                    pressed: false,
                    repeat: _,
                    modifiers:
                        egui::Modifiers {
                            alt: _,
                            ctrl: _,
                            shift: _,
                            mac_cmd: _,
                            command: _,
                        },
                } => {
                    if let Some(&pressed) = self.currently_held.get("alt+shift+o") {
                        if pressed {
                            let held = self.currently_held.entry("alt+shift+o").or_insert(false);
                            *held = false;
                        }
                    } else {
                        println!("The alt+shift+o combination was not in the 'currently_held' hashmap");
                    }
                }
                // Release of S
                egui::Event::Key {
                    key: egui::Key::S,
                    physical_key: _,
                    pressed: false,
                    repeat: _,
                    modifiers:
                        egui::Modifiers {
                            alt: _,
                            ctrl: _,
                            shift: _,
                            mac_cmd: _,
                            command: _,
                        },
                } => {
                    if let Some(&pressed) = self.currently_held.get("alt+shift+s") {
                        if pressed {
                            let held = self.currently_held.entry("alt+shift+s").or_insert(false);
                            *held = false;
                        }
                    } else {
                        println!("The alt+shift+s combination was not in the 'currently_held' hashmap");
                    }
                }
                // Release of Space
                egui::Event::Key {
                    key: egui::Key::Space,
                    physical_key: _,
                    pressed: false,
                    repeat: _,
                    modifiers:
                        egui::Modifiers {
                            alt: _,
                            ctrl: _,
                            shift: _,
                            mac_cmd: _,
                            command: _,
                        },
                } => {
                    if let Some(&pressed) = self.currently_held.get("space") {
                        if pressed {
                            let held = self.currently_held.entry("space").or_insert(false);
                            *held = false;
                        }
                    } else {
                        println!("The space combination was not in the 'currently_held' hashmap");
                    }
                }
                _ => {}
            }
        }
        if self.zoom == 0.0 {
            self.zoom = ctx.input(|i| i.zoom_delta()) - 1.0;
        }
        self.to_handle = to_handle;
        if touch_detected && self.zoom == 0.0 {
            self.pointer_position = PointerPosition::OnScreen(tap_position);
            self.primary_released |= tap_released;
        }
    }
}

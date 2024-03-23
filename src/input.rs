use eframe::{
	egui,
	epaint::{Color32, Pos2},
};
use nalgebra::Rotation3;
use std::{collections::HashMap, f32::consts::PI};

use crate::markers::Marker;
use crate::{
	enums::{self, PointerPosition},
	Application,
};

mod geometry;
const KEY_COMBINATIONS: [&str; 6] = ["alt+shift+g", "alt+shift+i", "alt+shift+o", "alt+shift+s", "mouse-middle", "space"];

impl Application {
	pub fn handle_input(&mut self, cursor_within_central_panel: bool, ctx: &egui::Context) {
		self.input.handle(cursor_within_central_panel, ctx);
		for input_to_handle in &self.input.to_handle {
			match input_to_handle {
				enums::Inputs::AltShiftG => self.state.windows.game_settings.opened = !self.state.windows.game_settings.opened,
				enums::Inputs::AltShiftI => self.state.windows.app_info.opened = !self.state.windows.app_info.opened,
				enums::Inputs::AltShiftO => self.state.windows.graphics_settings.opened = !self.state.windows.graphics_settings.opened,
				enums::Inputs::AltShiftS => self.state.windows.stats.opened = !self.state.windows.stats.opened,
				enums::Inputs::Space | enums::Inputs::MouseMiddle => {
					if !self.game_handler.no_more_questions() {
						if self.game_handler.stage == 0 {
							if !self.game_handler.should_display_input() {
								self.game_handler.check_answer(&mut self.cellestial_sphere);
							}
						} else if self.game_handler.stage == 1 {
							self.game_handler.next_question(&mut self.cellestial_sphere);
						} else {
							unimplemented!();
						}
					}
				}
			}
		}
		self.cellestial_sphere.zoom(self.input.zoom);

		let pointer_position: Pos2;

		match self.input.pointer_position {
			PointerPosition::OnScreen(position) => pointer_position = position,
			PointerPosition::OffScreen => return,
		}
		if cursor_within_central_panel {
			if self.game_handler.add_marker_on_click && self.input.primary_released && !self.input.primary_dragging_last_frame {
				let sphere_position = geometry::cast_onto_sphere(&self.cellestial_sphere, &pointer_position);
				let (dec, ra) = geometry::cartesian_to_spherical(sphere_position);
				let entry = self.cellestial_sphere.markers.entry("game".to_string()).or_default();
				*entry = vec![Marker::new(ra / PI * 180.0, dec / PI * 180.0, Color32::RED, 2.0, 5.0, self.game_handler.show_circle_marker(), false)];
				self.cellestial_sphere.init_single_renderer("markers", "game");
			}
	
			let initial_vector = self.cellestial_sphere.project_screen_pos(pointer_position - self.input.dragged);
			let final_vector = self.cellestial_sphere.project_screen_pos(pointer_position);
		
			if initial_vector != final_vector {
				// Some rotation this frame

				let rotation_matrix = Rotation3::rotation_between(&initial_vector, &final_vector).expect("FUCKIN FUCK");
				if !rotation_matrix.matrix()[0].is_nan(){
					self.cellestial_sphere.rotation *= rotation_matrix
				}
				// if self.input.secondary_released {}
				self.cellestial_sphere.init_renderers();
			}
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
		}
	}
}

impl Input {
	pub fn handle(&mut self, cursor_within_central_panel: bool, ctx: &egui::Context) {
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
		for event in &input_events {
			match event {
				egui::Event::Zoom(zoom) => {
					if *zoom > 1.0 {
						self.zoom = 0.2;
					} else if *zoom < 1.0 {
						self.zoom = -0.2;
					}
				}
				egui::Event::Scroll(egui::Vec2 { y, .. }) => {
					self.zoom = *y / 500.0;
				}
				// Press of Shift + UpArrow
				egui::Event::Key {
					key: egui::Key::ArrowUp,
					pressed: true,
					repeat: false,
					modifiers: egui::Modifiers {
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
					pressed: true,
					repeat: false,
					modifiers: egui::Modifiers {
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
					pressed: true,
					repeat: _,
					modifiers: egui::Modifiers {
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
					pressed: true,
					repeat: _,
					modifiers: egui::Modifiers {
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
					pressed: true,
					repeat: _,
					modifiers: egui::Modifiers {
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
					pressed: true,
					repeat: _,
					modifiers: egui::Modifiers {
						alt: _,
						ctrl: _,
						shift: _,
						mac_cmd: _,
						command: _,
					},
				} => {
					self.dragged.y += 1.0;
				}
				// Press of Alt + Shift + G
				egui::Event::Key {
					key: egui::Key::G,
					pressed: true,
					repeat: false,
					modifiers: egui::Modifiers {
						alt: true,
						ctrl: _,
						shift: true,
						mac_cmd: _,
						command: _,
					},
				} => {
					if let Some(pressed) = self.currently_held.get("alt+shift+g") {
						if !pressed {
							let held = self.currently_held.entry("alt+shift+g").or_insert(true);
							*held = true;
							to_handle.push(enums::Inputs::AltShiftG);
						}
					} else {
						println!("The alt+shift+g combination was not in the 'currently_held' hashmap");
					}
				}
				// Press of Alt + Shift + I
				egui::Event::Key {
					key: egui::Key::I,
					pressed: true,
					repeat: false,
					modifiers: egui::Modifiers {
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
					pressed: true,
					repeat: false,
					modifiers: egui::Modifiers {
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
					pressed: true,
					repeat: false,
					modifiers: egui::Modifiers {
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
				// Press of Space
				egui::Event::Key {
					key: egui::Key::Space,
					pressed: true,
					repeat: _,
					modifiers: egui::Modifiers {
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
					pressed: false,
					repeat: _,
					modifiers: egui::Modifiers {
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
					pressed: false,
					repeat: _,
					modifiers: egui::Modifiers {
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
					pressed: false,
					repeat: _,
					modifiers: egui::Modifiers {
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
					pressed: false,
					repeat: _,
					modifiers: egui::Modifiers {
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
					pressed: false,
					repeat: _,
					modifiers: egui::Modifiers {
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
		self.to_handle = to_handle;
	}
}

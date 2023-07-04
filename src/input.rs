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
const KEY_COMBINATIONS: [&str; 4] = ["alt+shift+g", "alt+shift+i", "alt+shift+o", "alt+shift+s"];

impl Application {
	pub fn handle_input(&mut self, cursor_within_central_panel: bool, ctx: &egui::Context) {
		self.input.handle(cursor_within_central_panel, ctx);
		for input_to_handle in &self.input.to_handle {
			match input_to_handle {
				enums::Inputs::AltShiftG => self.state.windows.game_settings.opened = !self.state.windows.game_settings.opened,
				enums::Inputs::AltShiftI => self.state.windows.app_info.opened = !self.state.windows.app_info.opened,
				enums::Inputs::AltShiftO => self.state.windows.graphics_settings.opened = !self.state.windows.graphics_settings.opened,
				enums::Inputs::AltShiftS => self.state.windows.stats.opened = !self.state.windows.stats.opened,
			}
		}
		self.cellestial_sphere.zoom(self.input.zoom / 500.0);

		let pointer_position: Pos2;

		match self.input.pointer_position {
			PointerPosition::OnScreen(position) => pointer_position = position,
			PointerPosition::OffScreen => return,
		}
		if self.input.primary_clicked {
			let (ra, dec) = geometry::cartesian_to_spherical(geometry::cast_onto_sphere(&self.cellestial_sphere, &pointer_position));
			let entry = self.cellestial_sphere.markers.entry("game".to_string()).or_default();
			*entry = vec![Marker::new(ra / PI * 180.0, dec / PI * 180.0, Color32::RED, 2.0, 5.0, false, false)];
		}

		let initial_vector = self.cellestial_sphere.project_screen_pos(pointer_position - self.input.dragged);
		let final_vector = self.cellestial_sphere.project_screen_pos(pointer_position);

		// println!("{}",final_vector)

		self.cellestial_sphere.rotation *= Rotation3::rotation_between(&initial_vector, &final_vector).expect("FUCKIN FUCK");

		if self.input.secondary_released {}
		self.cellestial_sphere.init_renderers(); // TODO: Maybe don't reinitialize them if the rotation hasn't changed
	}
}

pub struct Input {
	pub dragged: egui::Vec2,
	pub pointer_position: PointerPosition,
	pub to_handle: Vec<enums::Inputs>,
	pub zoom: f32,
	pub secondary_released: bool,
	pub primary_clicked: bool,

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
		self.secondary_released = ctx.input(|i| i.pointer.secondary_clicked());
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
		self.zoom = 0.0;
		for event in &input_events {
			match event {
				/*egui::Event::Zoom(zoom) => {

				}*/
				egui::Event::Scroll(egui::Vec2 { y, .. }) => {
					self.zoom = *y;
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
				_ => {}
			}
		}
		self.to_handle = to_handle;
	}
}

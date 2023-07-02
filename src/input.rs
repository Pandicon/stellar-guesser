use eframe::egui;
use std::collections::HashMap;

use crate::enums;

const KEY_COMBINATIONS: [&str; 2] = ["alt+shift+i", "alt+shift+s"];

pub struct Input {
	pub dragged: egui::Vec2,
	pub pointer_position: egui::Pos2,
	pub to_handle: Vec<enums::Inputs>,
	pub zoom: f32,

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
			pointer_position: egui::Pos2::new(0.0, 0.0),
			to_handle: Vec::new(),
			zoom: 1.0,

			pointer_down_outside_subwindow: false,
			currently_held,
		}
	}
}

impl Input {
	pub fn handle(&mut self, cursor_within_central_panel: bool, ctx: &egui::Context) {
		let input_events = ctx.input(|i| i.events.clone());
		let shift_held = ctx.input(|i| i.modifiers.shift);
		let drag_x = ctx.input(|i| i.pointer.delta().x);
		let drag_y = ctx.input(|i| i.pointer.delta().y);
		let primary_down = ctx.input(|i| i.pointer.primary_down());
		self.pointer_position = ctx.input(|i| i.pointer.hover_pos().unwrap_or(egui::pos2(0.0, 0.0)));
		if self.pointer_down_outside_subwindow && primary_down && ctx.input(|i| i.pointer.is_decidedly_dragging()) { // Ignore drags that started in a subwindow
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
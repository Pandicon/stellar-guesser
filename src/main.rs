#[path = "./tests/root.rs"]
mod tests;

pub mod application;
#[path = "./rendering/rendering.rs"]
mod rendering;

pub use application::Application;
use eframe::IconData;
use std::fs::read;
#[path = "./rendering/caspr/caspr.rs"]
pub mod caspr;
pub mod enums;
#[path = "./rendering/caspr/markers.rs"]
pub mod markers;
mod public_constants;
pub use public_constants::*;
pub mod structs;

pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
	let mut authors_split = AUTHORS.split(':').collect::<Vec<&str>>();
	let authors = if authors_split.len() > 2 {
		let last = authors_split.pop().unwrap();
		format!("{}, and {}", authors_split.join(", "), last)
	} else {
		authors_split.join(" and ")
	};
	let native_options = eframe::NativeOptions {
		app_id: Some(PROJECT_NAME.to_string()),
		maximized: true,
		resizable: true,
		icon_data: Some(IconData::try_from_png_bytes(&read("./ico.png").expect("File not found!")).expect("File not png!")),
		..Default::default()
	};

	eframe::run_native(
		"Stellar guesser",
		native_options,
		Box::new(|cc| Box::new(application::Application::new(cc, authors, VERSION.to_string()))),
	)
	.expect("Failed to start the application");
}

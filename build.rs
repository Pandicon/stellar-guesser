use chrono::{Datelike, Timelike, Utc};
use const_gen::*;
use std::{env, fs, path::Path};

const DEEPSKIES_FOLDER: &str = "./sphere/deepsky";
const LINES_FOLDER: &str = "./sphere/lines";
const MARKERS_FOLDER: &str = "./sphere/markers";
const STARS_FOLDER: &str = "./sphere/stars";
const STAR_NAMES_FOLDER: &str = "./sphere/named-stars";
const CONSTELLATION_VERTICES: &str = "./data/constellation_vertices.csv";
const CONSTELLATION_NAMES: &str = "./data/constellations.csv";

fn zero_nothing(num: i64) -> String {
	String::from(if num < 10 { "0" } else { "" })
}

fn main() {
	let out_dir = env::var_os("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("const_gen.rs");

	let curr_time = Utc::now();
	let date: Vec<String> = vec![
		format!("{}", curr_time.year()),
		format!("{}{}", zero_nothing(curr_time.month() as i64), curr_time.month()),
		format!("{}{}", zero_nothing(curr_time.day() as i64), curr_time.day()),
		format!("{}{}", zero_nothing(curr_time.hour() as i64), curr_time.hour()),
		format!("{}{}", zero_nothing(curr_time.minute() as i64), curr_time.minute()),
		format!("{}{}", zero_nothing(curr_time.second() as i64), curr_time.second()),
		format!("{}", curr_time.timestamp_millis()),
	];

	let target_os = std::env::var_os("CARGO_CFG_TARGET_OS").unwrap_or("_".into());
	let mut const_declarations_intermediate = if target_os == "android" || target_os == "ios" {
		let content_folder = [
			["deepskies", DEEPSKIES_FOLDER],
			["lines", LINES_FOLDER],
			["markers", MARKERS_FOLDER],
			["stars", STARS_FOLDER],
			["star names", STAR_NAMES_FOLDER],
		];
		let mut sky_data = Vec::new();

		for (i, d) in content_folder.iter().enumerate() {
			let id = d[0];
			let folder = d[1];
			sky_data.push((id, Vec::new()));
			let files = fs::read_dir(folder);
			if let Ok(files) = files {
				for file in files.flatten() {
					let path = file.path();
					let file_name = path.file_name();
					if file_name.is_none() {
						continue;
					}
					let file_name = file_name.unwrap().to_str();
					if file_name.is_none() {
						continue;
					}
					let file_name = file_name.unwrap().to_string();
					let file_content = fs::read_to_string(path);
					if let Ok(file_content) = file_content {
						sky_data[i].1.push([file_name, file_content.replace("\"", "\\\"")]);
					}
				}
			}
		}
		let mut other_sky_data = Vec::new();
		if let Ok(file_content) = fs::read_to_string(CONSTELLATION_NAMES) {
			other_sky_data.push([String::from("constellation names"), file_content.replace("\"", "\\\"")])
		};
		if let Ok(file_content) = fs::read_to_string(CONSTELLATION_VERTICES) {
			other_sky_data.push([String::from("constellation vertices"), file_content.replace("\"", "\\\"")])
		};

		vec![const_declaration!(pub SKY_DATA_LISTS = sky_data), const_declaration!(pub SKY_DATA_FILES = other_sky_data)]
	} else {
		vec![]
	};
	const_declarations_intermediate.push(const_declaration!(pub BUILD_DATE = date));
	fs::write(dest_path, const_declarations_intermediate.join("\n")).unwrap();

	if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
		let mut res = winres::WindowsResource::new();
		res.set_icon("./ico.ico");
		res.compile().unwrap();
	}
}

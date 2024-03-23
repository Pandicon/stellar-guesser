use chrono::{Datelike, Timelike, Utc};
use const_gen::*;
use std::{env, fs, path::Path};

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

	let const_declarations = vec![const_declaration!(BUILD_DATE = date)].join("\n");
	fs::write(dest_path, const_declarations).unwrap();
	if cfg!(target_os = "windows") {
		let mut res = winres::WindowsResource::new();
		res.set_icon("./ico.ico");
		res.compile().unwrap();
	}
}

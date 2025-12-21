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

    let _target_os = std::env::var_os("CARGO_CFG_TARGET_OS").unwrap_or("_".into());
    let _target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let mut const_declarations_intermediate = Vec::new();
    const_declarations_intermediate.push(const_declaration!(pub BUILD_DATE = date));
    const_declarations_intermediate.push(const_declaration!(pub BUILD_PROFILE = std::env::var("PROFILE").expect("The 'PROFILE' environment variable is missing")));
    fs::write(dest_path, const_declarations_intermediate.join("\n")).expect("Failed to save the const declarations");

    {
        let folder_to_embed = "data";
        let out_dir = env::var_os("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("embedded_loader.rs");
        let code = format!("pub static EMBEDDED_DATA: include_dir::Dir = include_dir::include_dir!(\"$CARGO_MANIFEST_DIR/{}\");", folder_to_embed);
        fs::write(&dest_path, code).unwrap();
    }

    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        let mut res = winres::WindowsResource::new();
        res.set_icon("./ico.ico");
        res.compile().unwrap();
    }
}

// Most of the code here comes from the https://github.com/rust-mobile/rust-android-examples template

#![allow(clippy::redundant_static_lifetimes)] // Comes from const_gen

use crate::application;

include!(concat!(env!("OUT_DIR"), "/const_gen.rs"));

fn _main(options: eframe::NativeOptions) {
    if let Err(err) = dotenvy::dotenv() {
        log::error!("Failed to initialise dotenvy: {}", err);
    };
    let _main_server_url = &crate::CONFIG.main_server_url; // Force the config to load at the start

    eframe::run_native(
        crate::PROJECT_NAME,
        options,
        Box::new(|cc| Ok(Box::new(application::Application::new(cc, crate::entry_point::generate_authors(), crate::VERSION.to_string())))),
    )
    .unwrap();
}

pub fn main() {
    env_logger::builder().filter_level(crate::config::LOGGING_FILTER_LEVEL).parse_default_env().init();

    let mut options = eframe::NativeOptions {
        viewport: eframe::egui::viewport::ViewportBuilder::default().with_maximized(true),
        persist_window: false,
        ..Default::default()
    };

    {
        let default_path = {
            let mut def_path = directories_next::ProjectDirs::from("", "", crate::DESKTOP_PACKAGE_NAME)
                .map(|proj_dirs| proj_dirs.data_dir().to_path_buf())
                .unwrap_or(".".into());
            def_path.push("save.ron");
            def_path
        };
        options.persistence_path = Some(default_path);

        let icon_data = {
            match image::open(crate::ICON_PATH) {
                Ok(dynamic_image) => {
                    let image = dynamic_image.into_rgba8();
                    let (width, height) = image.dimensions();
                    let rgba = image.into_raw();
                    let icon_data = eframe::egui::viewport::IconData { rgba, width, height };
                    Some(std::sync::Arc::new(icon_data))
                }
                Err(err) => {
                    log::error!("Failed to open icon path: {:?}", err);
                    None
                }
            }
        };
        options.viewport.icon = icon_data;
    }
    _main(options);
}

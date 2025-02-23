// Most of the code here comes from the https://github.com/rust-mobile/rust-android-examples template

#![allow(clippy::redundant_static_lifetimes)] // Comes from const_gen

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

pub use application::Application;
pub use public_constants::*;
pub use rendering::caspr::renderer;

pub mod application;
pub mod config;
pub mod enums;
pub mod files;
pub mod game;
pub mod graphics;
pub mod input;
mod public_constants;
pub mod rendering;
pub mod server_communication;
pub mod structs;

pub const MINIMUM_CIRCLE_RADIUS_TO_RENDER: f32 = 0.5;

pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
const ICON_PATH: &str = "./ico.png";

#[cfg(target_os = "android")]
pub const PLATFORM: &str = "android";
#[cfg(target_os = "windows")]
pub const PLATFORM: &str = "windows";
#[cfg(target_os = "linux")]
pub const PLATFORM: &str = "linux";

#[cfg(target_os = "android")]
pub const MOBILE: bool = true;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub const MOBILE: bool = false;

pub static CONFIG: once_cell::sync::Lazy<config::Config> = once_cell::sync::Lazy::new(config::get_config);
pub static CREDITS: once_cell::sync::Lazy<Vec<sg_credits::Credits>> = once_cell::sync::Lazy::new(sg_credits::get_credits);

include!(concat!(env!("OUT_DIR"), "/const_gen.rs"));

fn _main(options: eframe::NativeOptions) {
    if let Err(err) = dotenvy::dotenv() {
        log::error!("Failed to initialise dotenvy: {}", err);
    };
    let _main_server_url = &CONFIG.main_server_url; // Force the config to load at the start

    let mut authors_split = AUTHORS.split(':').collect::<Vec<&str>>();
    let authors = if authors_split.len() > 2 {
        let last = authors_split.pop().unwrap();
        format!("{}, and {}", authors_split.join(", "), last)
    } else {
        authors_split.join(" and ")
    };

    eframe::run_native(PROJECT_NAME, options, Box::new(|cc| Ok(Box::new(application::Application::new(cc, authors, VERSION.to_string()))))).unwrap();
}

#[cfg(any(target_os = "ios", target_os = "android"))]
fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("attempt to unwind out of `rust` with err: {:?}", err);
            std::process::abort()
        }
    }
}

#[cfg(target_os = "ios")]
fn _start_app() {
    stop_unwind(|| main());
}

#[no_mangle]
#[inline(never)]
#[cfg(target_os = "ios")]
pub extern "C" fn start_app() {
    _start_app();
}

#[cfg(not(target_os = "android"))]
pub fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Warn).parse_default_env().init();

    let mut options = eframe::NativeOptions {
        viewport: eframe::egui::viewport::ViewportBuilder::default().with_maximized(true),
        persist_window: false,
        ..Default::default()
    };

    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    {
        let default_path = {
            let mut def_path = directories_next::ProjectDirs::from("", "", DESKTOP_PACKAGE_NAME)
                .map(|proj_dirs| proj_dirs.data_dir().to_path_buf())
                .unwrap_or(".".into());
            def_path.push("save.ron");
            def_path
        };
        options.persistence_path = Some(default_path);

        let icon_data = {
            match image::open(ICON_PATH) {
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

#[allow(dead_code)]
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Warn));

    let mut options = eframe::NativeOptions {
        persist_window: false,
        ..Default::default()
    };
    options.android_app = Some(app);
    /*options.event_loop_builder = Some(Box::new(move |event_loop| {
        event_loop.with_android_app(app);
    }));*/
    // Android paths:
    // - <package_name> is the package name, for example
    // - /data/data/<packagename>/files/<path> (for example /data/data/<packagename>/files/id.txt) is a sandboxed piece of storage that no other app can access (and also the user can not access it without a rooted device)
    // - /storage/emulated/0/Android/data/<packagename>/files/<path> (for example /storage/emulated/0/Android/data/<packagename>/files/id.txt) is a storage accessible by the user, but only from a computer as of newer Android versions
    // - /storage/emulated/0/Documents/<path> (for example /storage/emulated/0/Documents/id.txt) is a completely public piece of storage in the Documents directory
    let default_path = format!("/storage/emulated/0/Android/data/{ANDROID_PACKAGE_NAME}/files/save.ron");
    options.persistence_path = Some(default_path.into());

    stop_unwind(|| _main(options));
}

#[cfg(target_os = "android")]
pub fn show_soft_input(show: bool) -> bool {
    let ctx = ndk_context::android_context();

    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
    let env = vm.attach_current_thread().unwrap();

    let class_ctx = env.find_class("android/content/Context").unwrap();
    let ime = env.get_static_field(class_ctx, "INPUT_METHOD_SERVICE", "Ljava/lang/String;").unwrap();
    let ime_manager = env
        .call_method(ctx.context() as jni::sys::jobject, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[ime])
        .unwrap()
        .l()
        .unwrap();

    let jni_window = env.call_method(ctx.context() as jni::sys::jobject, "getWindow", "()Landroid/view/Window;", &[]).unwrap().l().unwrap();
    let view = env.call_method(jni_window, "getDecorView", "()Landroid/view/View;", &[]).unwrap().l().unwrap();

    if show {
        let result = env
            .call_method(ime_manager, "showSoftInput", "(Landroid/view/View;I)Z", &[view.into(), 0i32.into()])
            .unwrap()
            .z()
            .unwrap();
        log::info!("show input: {}", result);
        result
    } else {
        let window_token = env.call_method(view, "getWindowToken", "()Landroid/os/IBinder;", &[]).unwrap().l().unwrap();
        let result = env
            .call_method(ime_manager, "hideSoftInputFromWindow", "(Landroid/os/IBinder;I)Z", &[window_token.into(), 0i32.into()])
            .unwrap()
            .z()
            .unwrap();
        log::info!("hide input: {}", result);
        result
    }
}

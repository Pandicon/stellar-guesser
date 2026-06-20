// Most of the code here comes from the https://github.com/rust-mobile/rust-android-examples template

#![allow(clippy::redundant_static_lifetimes)] // Comes from const_gen

pub use application::Application;

pub use config::{ANDROID_PACKAGE_NAME, DESKTOP_PACKAGE_NAME};
pub use entry_point::*;

pub mod action;
pub mod application;
pub mod config;
pub mod entry_point;
pub mod enums;
pub mod files_handling;
pub mod game;
pub mod graphics;
pub mod input;
pub mod rendering;
pub mod server_communication;
pub mod sky;
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
#[cfg(target_arch = "wasm32")]
pub const PLATFORM: &str = "web";

#[cfg(target_os = "android")]
pub const MOBILE: bool = true;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub const MOBILE: bool = false;
#[cfg(target_arch = "wasm32")]
pub const MOBILE: bool = false;

pub static CONFIG: once_cell::sync::Lazy<config::Config> = once_cell::sync::Lazy::new(config::get_config);
pub static CREDITS: once_cell::sync::Lazy<Vec<sg_credits::Credits>> = once_cell::sync::Lazy::new(sg_credits::get_credits);

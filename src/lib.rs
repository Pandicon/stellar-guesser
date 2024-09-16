// Most of the code here comes from the https://github.com/rust-mobile/rust-android-examples template

#![allow(clippy::redundant_static_lifetimes)] // Comes from const_gen

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

use winit::event::Event::*;
use winit::event_loop::{EventLoop, EventLoopBuilder, EventLoopWindowTarget};

use egui_wgpu::winit::Painter;
use egui_winit::State;

const INITIAL_WIDTH: u32 = 1920;
const INITIAL_HEIGHT: u32 = 1080;

pub use application::Application;
pub use public_constants::*;
pub use rendering::caspr::renderer;

pub mod application;
pub mod config;
pub mod enums;
pub mod files;
pub mod game;
pub mod geometry;
pub mod graphics;
pub mod input;
mod public_constants;
pub mod rendering;
pub mod server_communication;
pub mod storage;
pub mod structs;
mod tests;

pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(target_os = "windows")]
const ICON_PATH: &str = "./ico.png";

#[cfg(target_os = "android")]
pub const PLATFORM: &str = "android";
#[cfg(target_os = "windows")]
pub const PLATFORM: &str = "windows";

pub static CONFIG: once_cell::sync::Lazy<config::Config> = once_cell::sync::Lazy::new(config::get_config);

include!(concat!(env!("OUT_DIR"), "/const_gen.rs"));

/// A custom event type for the winit app.
enum Event {
    RequestRedraw,
}

/// Enable egui to request redraws via a custom Winit event...
#[derive(Clone)]
struct RepaintSignal(std::sync::Arc<std::sync::Mutex<winit::event_loop::EventLoopProxy<Event>>>);

fn create_window<T>(event_loop: &EventLoopWindowTarget<T>, state: &mut State, painter: &mut Painter) -> winit::window::Window {
    #[cfg(target_os = "windows")]
    let window_icon = {
        let icon_data = {
            if let Ok(dynamic_image) = image::open(ICON_PATH) {
                let image = dynamic_image.into_rgba8();
                let (width, height) = image.dimensions();
                let rgba = image.into_raw();
                Some((rgba, width, height))
            } else {
                println!("Failed to open icon path");
                None
            }
        };
        if let Some((icon_rgba, icon_width, icon_height)) = icon_data {
            if let Ok(icon) = winit::window::Icon::from_rgba(icon_rgba, icon_width, icon_height) {
                Some(icon)
            } else {
                println!("Failed to open the icon");
                None
            }
        } else {
            None
        }
    };
    #[cfg(not(target_os = "windows"))]
    let window_icon = None;
    let window = winit::window::WindowBuilder::new()
        .with_decorations(true)
        .with_resizable(true)
        .with_transparent(false)
        .with_title("Stellar guesser")
        .with_inner_size(winit::dpi::PhysicalSize {
            width: INITIAL_WIDTH,
            height: INITIAL_HEIGHT,
        })
        .with_maximized(true)
        .with_window_icon(window_icon)
        .build(event_loop)
        .unwrap();

    pollster::block_on(painter.set_window(state.egui_ctx().viewport_id(), Some(&window))).unwrap();

    // NB: calling set_window will lazily initialize render state which
    // means we will be able to query the maximum supported texture
    // dimensions
    if let Some(max_size) = painter.max_texture_side() {
        state.set_max_texture_side(max_size);
    }

    window.request_redraw();

    window
}

fn _main(event_loop: EventLoop<Event>) {
    let _main_server_url = &CONFIG.main_server_url; // Force the config to load at the start
    let ctx = egui::Context::default();
    let repaint_signal = RepaintSignal(std::sync::Arc::new(std::sync::Mutex::new(event_loop.create_proxy())));
    ctx.set_request_repaint_callback(move |_| {
        repaint_signal.0.lock().unwrap().send_event(Event::RequestRedraw).ok();
    });

    let mut painter = Painter::new(
        egui_wgpu::WgpuConfiguration::default(),
        1, // msaa samples
        None,
        false,
    );
    let mut state = State::new(ctx.clone(), ctx.viewport_id(), &event_loop, ctx.native_pixels_per_point(), painter.max_texture_side());
    let mut window: Option<winit::window::Window> = None;

    let mut authors_split = AUTHORS.split(':').collect::<Vec<&str>>();
    let authors = if authors_split.len() > 2 {
        let last = authors_split.pop().unwrap();
        format!("{}, and {}", authors_split.join(", "), last)
    } else {
        authors_split.join(" and ")
    };
    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos", target_os = "android"))]
    let mut storage = Some(storage::Storage::new());
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos", target_os = "android")))]
    let mut storage = None; // Maybe implement iOS and other platforms storage?
    let mut application = application::Application::new(&ctx, authors, VERSION.to_string(), &mut storage);

    let run_res = event_loop.run(move |event, event_loop| match event {
        Resumed => match window {
            None => {
                window = Some(create_window(event_loop, &mut state, &mut painter));
            }
            Some(ref window) => {
                pollster::block_on(painter.set_window(ctx.viewport_id(), Some(window))).unwrap();
                window.request_redraw();
            }
        },
        Suspended => {
            // Save application state
            if let Some(storage) = &mut storage {
                application.save(storage);
                storage.save();
            }
            window = None;
        }
        UserEvent(Event::RequestRedraw) => {
            if let Some(window) = window.as_ref() {
                window.request_redraw();
            }
        }
        WindowEvent { event, .. } => {
            match event {
                winit::event::WindowEvent::RedrawRequested => {
                    if let Some(window) = window.as_ref() {
                        let raw_input = state.take_egui_input(window);

                        let full_output = ctx.run(raw_input, |ctx| {
                            application.update(ctx);

                            // Save application state
                            if let Some(storage) = &mut storage {
                                let now = std::time::Instant::now();
                                if now - application.last_state_save > application.state_save_interval {
                                    application.save(storage);
                                    application.last_state_save = now;
                                }
                            }

                            // toggle software keyboard
                            #[cfg(target_os = "android")]
                            if application.input.input_field_has_focus && !application.input.input_field_had_focus_last_frame {
                                // There was no focus on any text input field last frame, but there is this frame -> show the keyboard
                                show_soft_input(true);
                            } else if !application.input.input_field_has_focus && application.input.input_field_had_focus_last_frame {
                                // There was focus on some text input field last frame, but there is not this frame -> hide the keyboard
                                show_soft_input(false);
                            }
                        });
                        state.handle_platform_output(window, full_output.platform_output);

                        let pixels_per_point = egui_winit::pixels_per_point(&ctx, window);
                        painter.paint_and_update_textures(
                            ctx.viewport_id(),
                            pixels_per_point,
                            egui::Rgba::default().to_array(),
                            &ctx.tessellate(full_output.shapes, pixels_per_point),
                            &full_output.textures_delta,
                            false,
                        );

                        if let Some(viewport_output) = full_output.viewport_output.get(&egui::ViewportId::ROOT) {
                            if viewport_output.repaint_delay.is_zero() {
                                window.request_redraw();
                            }
                        } else {
                            log::error!("No root viewport output");
                        }
                    }
                }
                winit::event::WindowEvent::Resized(size) => {
                    if size.width > 0 && size.height > 0 {
                        painter.on_window_resized(
                            state.egui_ctx().viewport_id(),
                            std::num::NonZeroU32::new(size.width).unwrap(),
                            std::num::NonZeroU32::new(size.height).unwrap(),
                        );
                    }
                }
                winit::event::WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                _ => {}
            }

            if let Some(window) = window.as_ref() {
                let response = state.on_window_event(window, &event);
                if response.repaint {
                    window.request_redraw();
                }
            }
        }
        LoopExiting => {
            // Save application state
            if let Some(storage) = &mut storage {
                application.save(storage);
                storage.save();
            }
        }
        _ => (),
    });
    match run_res {
        Ok(_) => {}
        Err(err) => log::error!("Error while running the event loop: {err:?}"),
    }
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

    let event_loop = EventLoopBuilder::with_user_event().build();
    match event_loop {
        Ok(event_loop) => {
            _main(event_loop);
        }
        Err(err) => {
            log::error!("Failed to create event loop: {err:?}");
        }
    }
}

#[allow(dead_code)]
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Warn));

    let event_loop = EventLoopBuilder::with_user_event().with_android_app(app).build();
    match event_loop {
        Ok(event_loop) => {
            stop_unwind(|| _main(event_loop));
        }
        Err(err) => {
            log::error!("Failed to create event loop: {err:?}");
        }
    }
}

#[cfg(target_os = "android")]
fn show_soft_input(show: bool) -> bool {
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

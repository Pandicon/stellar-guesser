// Most of the code here comes from the https://github.com/rust-mobile/rust-android-examples template

#![allow(clippy::redundant_static_lifetimes)] // Comes from const_gen

use winit::platform::android::activity::AndroidApp;

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

fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("attempt to unwind out of `rust` with err: {:?}", err);
            std::process::abort()
        }
    }
}

#[allow(dead_code)]
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
    let default_path = format!("/storage/emulated/0/Android/data/{}/files/save.ron", crate::ANDROID_PACKAGE_NAME);
    options.persistence_path = Some(default_path.into());

    stop_unwind(|| _main(options));
}

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

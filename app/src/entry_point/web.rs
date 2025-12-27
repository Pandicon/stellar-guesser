use wasm_bindgen::prelude::*;

use crate::application;

include!(concat!(env!("OUT_DIR"), "/const_gen.rs"));

#[wasm_bindgen(start)]
fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(crate::config::LOGGING_FILTER_LEVEL).ok();

    let web_options = eframe::WebOptions::default();
    log::warn!("AAAA");

    wasm_bindgen_futures::spawn_local(async {
        log::warn!("BBBB");
        let document = web_sys::window().expect("No window").document().expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(application::Application::new(cc, crate::entry_point::generate_authors(), crate::VERSION.to_string())))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    log::warn!("Removing loading text");
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html("<p> The app has crashed. See the developer console for details. </p>");
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

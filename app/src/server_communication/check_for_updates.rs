use crate::structs::state::threads_communication;

#[derive(serde::Deserialize)]
pub struct UpdateExistsResponse {
    pub newer_version_exists: bool,
    pub newest_version: String,
    pub newest_found_version: String,
}

async fn run_update_check(url: String, sender: std::sync::mpsc::Sender<Result<UpdateExistsResponse, String>>) {
    match reqwest::get(url).await {
        Ok(response) => {
            if let Ok(body) = response.text().await {
                match serde_json::from_str(&body) {
                    Ok(res) => {
                        match sender.send(Ok(res)) {
                            Ok(_) => {}
                            Err(err) => log::error!("Failed to send the response: {}", err),
                        }
                        return;
                    }
                    Err(err) => {
                        log::error!("Failed to serialize the response: {}\nError: {}", body, err)
                    }
                }
            }
        }
        Err(err) => {
            log::error!("Failed to fetch the updates: {}", err);
        }
    }
    match sender.send(Err(String::from("Failed to fetch updates"))) {
        Ok(_) => {}
        Err(err) => log::error!("Failed to send the response: {}", err),
    }
}

pub fn check_for_updates(app_threads_communication: &mut threads_communication::ThreadsCommunication, platform: &str, current_version: &str, show_popup: threads_communication::CheckUpdatesShowPopup) {
    let (sender, receiver) = std::sync::mpsc::channel();
    let config = threads_communication::CheckUpdates { show_popup, receiver };
    app_threads_communication.check_updates = Some(config);
    let url = format!("{}/update_exists?platform={}&current_version={}", crate::CONFIG.main_server_url.clone(), platform, current_version);

    #[cfg(not(target_arch = "wasm32"))]
    {
        std::thread::spawn(move || match tokio::runtime::Runtime::new() {
            Ok(runtime) => {
                runtime.block_on(async {
                    run_update_check(url, sender).await;
                });
            }
            Err(err) => {
                log::error!("Failed to create runtime: {}", err);
            }
        });
    }
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(async move {
            run_update_check(url, sender).await;
        });
    }
}

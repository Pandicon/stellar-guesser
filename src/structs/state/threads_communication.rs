use crate::Application;

#[derive(Default)]
pub struct ThreadsCommunication {
    pub check_updates: Option<CheckUpdates>,
}

pub enum CheckUpdatesShowPopup {
    OnFoundUpdate,
    IfInfoWindowClosed,
}

pub struct CheckUpdates {
    pub show_popup: CheckUpdatesShowPopup,
    pub receiver: std::sync::mpsc::Receiver<Result<crate::server_communication::check_for_updates::UpdateExistsResponse, String>>,
}

impl Application {
    pub fn receive_threads_messages(&mut self) {
        if let Some(check_updates_setup) = &self.threads_communication.check_updates {
            if let Ok(response_res) = check_updates_setup.receiver.try_recv() {
                match response_res {
                    Ok(response) => {
                        match check_updates_setup.show_popup {
                            CheckUpdatesShowPopup::OnFoundUpdate => {
                                if response.newer_version_exists {
                                    self.toasts
                                        .info(format!("An update to version {} is available!", response.newest_found_version))
                                        .duration(Some(std::time::Duration::from_secs(15)));
                                }
                            }
                            CheckUpdatesShowPopup::IfInfoWindowClosed => {
                                if !self.state.windows.app_info.opened {
                                    if response.newer_version_exists {
                                        self.toasts
                                            .info(format!("An update to version {} is available!", response.newest_found_version))
                                            .duration(Some(std::time::Duration::from_secs(15)));
                                    } else {
                                        self.toasts.info("No updates found.").duration(Some(std::time::Duration::from_secs(15)));
                                    }
                                }
                            }
                        }

                        self.version.update_available = Some(response.newer_version_exists);
                        self.version.latest_version = Some(response.newest_version);
                        self.version.latest_released_version = Some(response.newest_found_version);

                        self.threads_communication.check_updates = None;
                    }
                    Err(err) => {
                        log::error!("Failed to fetch update information: {}", err);
                        self.toasts.error("Failed to fetch update information").duration(Some(std::time::Duration::from_secs(15)));

                        self.threads_communication.check_updates = None;
                    }
                }
            }
        }
    }
}

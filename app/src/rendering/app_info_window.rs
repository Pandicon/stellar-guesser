use chrono::{Datelike, TimeZone, Timelike};
use eframe::egui;

use crate::{Application, BUILD_DATE};

fn zero_nothing_prefix(num: i64) -> String {
    format!("{}{}", if num < 10 { "0" } else { "" }, num)
}

impl Application {
    pub fn render_application_info_window(&mut self, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>> {
        egui::Window::new("Application info").open(&mut self.state.windows.app_info.opened).show(ctx, |ui| {
            ui.label(format!("Authors: {}", self.authors));
            ui.label(format!("Version: {} (built in {} mode)", self.version.current_version, crate::BUILD_PROFILE));
            if let Some(update_exists) = self.version.update_available {
                if update_exists {
                    if let Some(newest_version) = &self.version.latest_released_version {
                        ui.label(format!("There is an update available, newest version is {}", newest_version));
                    } else {
                        ui.label("There is an update available");
                    }
                } else {
                    ui.label("You are running the newest version");
                }
            } else {
                ui.label("Update information has not been loaded yet.");
            }
            ui.add_enabled_ui(self.threads_communication.check_updates.is_none(), |ui| {
                if ui.button("Check for updates").clicked() {
                    crate::server_communication::check_for_updates::check_for_updates(
                        &mut self.threads_communication,
                        crate::PLATFORM,
                        &self.version.current_version,
                        crate::structs::state::threads_communication::CheckUpdatesShowPopup::IfInfoWindowClosed,
                    );
                };
            });
            let date = {
                if let Ok(timestamp) = BUILD_DATE[6].parse::<i64>() {
                    if let chrono::MappedLocalTime::Single(datetime_utc) = chrono::Utc.timestamp_millis_opt(timestamp) {
                        let datetime_local = datetime_utc.with_timezone(&chrono::Local);
                        Some(datetime_local)
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
            let date_string = match date {
                Some(date) => {
                    format!(
                        "{}.{}.{}, {}:{}:{} ({}.{}.{}, {}:{}:{} UTC)",
                        zero_nothing_prefix(date.day() as i64),
                        zero_nothing_prefix(date.month() as i64),
                        zero_nothing_prefix(date.year() as i64),
                        zero_nothing_prefix(date.hour() as i64),
                        zero_nothing_prefix(date.minute() as i64),
                        zero_nothing_prefix(date.second() as i64),
                        BUILD_DATE[2],
                        BUILD_DATE[1],
                        BUILD_DATE[0],
                        BUILD_DATE[3],
                        BUILD_DATE[4],
                        BUILD_DATE[5]
                    )
                }
                None => format!("{}.{}.{}, {}:{}:{} UTC", BUILD_DATE[2], BUILD_DATE[1], BUILD_DATE[0], BUILD_DATE[3], BUILD_DATE[4], BUILD_DATE[5]),
            };
            ui.label(format!("Build date and time: {}", date_string));
        })
    }
}

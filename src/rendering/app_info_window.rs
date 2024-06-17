use chrono::{Datelike, TimeZone, Timelike};

use crate::{Application, BUILD_DATE};

fn zero_nothing_prefix(num: i64) -> String {
    format!("{}{}", if num < 10 { "0" } else { "" }, num)
}

impl Application {
    pub fn render_application_info_window(&mut self, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>> {
        egui::Window::new("Application info").open(&mut self.state.windows.app_info.opened).show(ctx, |ui| {
            ui.label(format!("Authors: {}", self.authors));
            ui.label(format!("Version: {} (built in {} mode)", self.version, crate::BUILD_PROFILE));
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

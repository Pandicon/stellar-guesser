use std::collections::HashMap;

use egui::Color32;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Theme {
    pub name: String,
    pub game_visuals: Visuals,
    pub egui_visuals: egui::Visuals,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            name: "Dark".into(),
            game_visuals: Visuals {
                default_colour: Color32::WHITE,
                default_star_colour: Color32::WHITE,
                use_default_star_colour: false,
                lines_colours: HashMap::new(),
            },
            egui_visuals: egui::Visuals::dark(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Visuals {
    pub default_colour: Color32,
    pub default_star_colour: Color32,
    pub use_default_star_colour: bool,
    pub lines_colours: HashMap<String, Color32>,
}

pub fn default_themes() -> HashMap<String, Theme> {
    let mut themes = HashMap::new();
    let dark_theme = Theme::dark();
    themes.insert(dark_theme.name.clone(), dark_theme);
    themes.insert(
        "Light".into(),
        Theme {
            name: "Light".into(),
            game_visuals: Visuals {
                default_colour: Color32::BLACK,
                default_star_colour: Color32::BLACK,
                use_default_star_colour: true,
                lines_colours: HashMap::new(),
            },
            egui_visuals: egui::Visuals::light(),
        },
    );
    let mut egui_visuals = egui::Visuals::light();
    egui_visuals.panel_fill = Color32::WHITE;
    egui_visuals.window_fill = Color32::WHITE;
    themes.insert(
        "Printing".into(),
        Theme {
            name: "Printing".into(),
            game_visuals: Visuals {
                default_colour: Color32::BLACK,
                default_star_colour: Color32::BLACK,
                use_default_star_colour: true,
                lines_colours: HashMap::new(),
            },
            egui_visuals,
        },
    );
    themes
}

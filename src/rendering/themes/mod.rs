use std::collections::HashMap;

use egui::Color32;

use crate::Application;

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

pub struct ThemesHandler {
    data: HashMap<String, Theme>,
}

impl ThemesHandler {
    pub fn insert(&mut self, name: String, theme: Theme) -> Option<Theme> {
        self.data.insert(name, theme)
    }

    pub fn get(&self, name: &str) -> Option<&Theme> {
        self.data.get(name)
    }

    pub fn themes_names(&self) -> std::collections::hash_map::Keys<String, Theme> {
        self.data.keys()
    }

    pub fn add_theme_str(&mut self, data_str: &str) -> Result<Option<Theme>, serde_json::Error> {
        let theme: Theme = serde_json::from_str(data_str)?;
        Ok(self.insert(theme.name.clone(), theme))
    }

    pub fn from_hash_map(data: HashMap<String, Theme>) -> Self {
        Self { data }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Visuals {
    pub default_colour: Color32,
    pub default_star_colour: Color32,
    pub use_default_star_colour: bool,
    pub lines_colours: HashMap<String, Color32>,
}

pub fn default_themes() -> ThemesHandler {
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
    ThemesHandler::from_hash_map(themes)
}

impl Application {
    pub fn apply_theme(&mut self, ctx: &egui::Context, theme: Theme) {
        self.theme = theme;
        let mut lines_to_reinit = Vec::new();
        for (name, lines) in &mut self.cellestial_sphere.lines {
            match self.theme.game_visuals.lines_colours.get(name) {
                Some(colour) => {
                    lines.colour = *colour;
                    if lines.active {
                        lines_to_reinit.push(name.clone());
                    }
                }
                None => {
                    self.theme.game_visuals.lines_colours.insert(name.clone(), lines.colour);
                }
            }
        }
        for name in lines_to_reinit {
            self.cellestial_sphere.init_single_renderer("lines", &name);
        }
        ctx.set_visuals(self.theme.egui_visuals.clone());
    }
}

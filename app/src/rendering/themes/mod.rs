use std::collections::HashMap;

use eframe::egui;
use egui::Color32;

use crate::{enums::RendererCategory, Application};

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
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
                override_star_colour: Color32::WHITE,
                use_overriden_star_colour: false,
                lines_colours: HashMap::from(
                    [
                        ("celestial-lines-of-latitude.csv", [217, 98, 13, 255]),
                        ("celestial-meridians.csv", [217, 98, 13, 255]),
                        ("asterisms.csv", [107, 119, 255, 255]),
                        ("constellation-borders.csv", [135, 197, 255, 255]),
                        ("ecliptic.csv", [107, 255, 107, 255]),
                        ("celestial-equator.csv", [217, 13, 13, 255]),
                        ("galactic-equator.csv", [166, 107, 255, 255]),
                        ("constellation-connections.csv", [107, 119, 255, 255]),
                        ("prime-meridian.csv", [217, 13, 13, 255]),
                    ]
                    .map(|(n, c)| (n.to_string(), Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]))),
                ),
                markers_colours: HashMap::from(
                    [
                        ("galactic_poles.csv", [166, 107, 255, 255]),
                        ("first_point_of_aries.csv", [255, 251, 0, 255]),
                        ("galactic_centre_anticentre.csv", [146, 81, 245, 255]),
                        ("celestial_poles.csv", [217, 13, 13, 255]),
                        ("ecliptic_poles.csv", [107, 255, 107, 255]),
                    ]
                    .map(|(n, c)| (n.to_string(), Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]))),
                ),
                game_markers_colours: GameMarkersColours::default(),
                deepskies_colours: HashMap::from(
                    [("messier-catalogue.csv", [107, 238, 255, 255]), ("caldwell-catalogue.csv", [107, 255, 191, 255])]
                        .map(|(n, c)| (n.to_string(), Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]))),
                ),
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

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct Visuals {
    pub default_colour: Color32,
    pub override_star_colour: Color32,
    pub use_overriden_star_colour: bool,
    #[serde(default)]
    pub lines_colours: HashMap<String, Color32>,
    #[serde(default)]
    pub markers_colours: HashMap<String, Color32>,
    #[serde(default)]
    pub game_markers_colours: GameMarkersColours,
    #[serde(default)]
    pub deepskies_colours: HashMap<String, Color32>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug)]
pub struct GameMarkersColours {
    pub exact: Color32,
    pub tolerance: Color32,
    pub task: Color32,
    pub correct_answer: Color32,
}

impl Default for GameMarkersColours {
    fn default() -> Self {
        Self {
            exact: Color32::RED,
            tolerance: Color32::LIGHT_RED,
            task: Color32::YELLOW,
            correct_answer: Color32::YELLOW,
        }
    }
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
                override_star_colour: Color32::BLACK,
                use_overriden_star_colour: true,
                lines_colours: HashMap::from(
                    [
                        ("celestial-lines-of-latitude.csv", [217, 98, 13, 255]),
                        ("celestial-meridians.csv", [217, 98, 13, 255]),
                        ("asterisms.csv", [107, 119, 255, 255]),
                        ("constellation-borders.csv", [135, 197, 255, 255]),
                        ("ecliptic.csv", [107, 255, 107, 255]),
                        ("celestial-equator.csv", [217, 13, 13, 255]),
                        ("galactic-equator.csv", [166, 107, 255, 255]),
                        ("constellation-connections.csv", [107, 119, 255, 255]),
                        ("prime-meridian.csv", [217, 13, 13, 255]),
                    ]
                    .map(|(n, c)| (n.to_string(), Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]))),
                ),
                markers_colours: HashMap::from(
                    [
                        ("galactic_poles.csv", [166, 107, 255, 255]),
                        ("first_point_of_aries.csv", [255, 155, 0, 255]),
                        ("galactic_centre_anticentre.csv", [146, 81, 245, 255]),
                        ("celestial_poles.csv", [217, 13, 13, 255]),
                        ("ecliptic_poles.csv", [107, 255, 107, 255]),
                    ]
                    .map(|(n, c)| (n.to_string(), Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]))),
                ),
                game_markers_colours: GameMarkersColours {
                    task: Color32::from_rgba_unmultiplied(255, 155, 0, 255),
                    correct_answer: Color32::from_rgba_unmultiplied(255, 155, 0, 255),
                    ..Default::default()
                },
                deepskies_colours: HashMap::from(
                    [("messier-catalogue.csv", [62, 211, 228, 255]), ("caldwell-catalogue.csv", [75, 227, 165, 255])]
                        .map(|(n, c)| (n.to_string(), Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]))),
                ),
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
                override_star_colour: Color32::BLACK,
                use_overriden_star_colour: true,
                lines_colours: HashMap::from(
                    [
                        ("celestial-lines-of-latitude.csv", [217, 98, 13, 255]),
                        ("celestial-meridians.csv", [217, 98, 13, 255]),
                        ("asterisms.csv", [107, 119, 255, 255]),
                        ("constellation-borders.csv", [135, 197, 255, 255]),
                        ("ecliptic.csv", [107, 255, 107, 255]),
                        ("celestial-equator.csv", [217, 13, 13, 255]),
                        ("galactic-equator.csv", [166, 107, 255, 255]),
                        ("constellation-connections.csv", [107, 119, 255, 255]),
                        ("prime-meridian.csv", [217, 13, 13, 255]),
                    ]
                    .map(|(n, c)| (n.to_string(), Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]))),
                ),
                markers_colours: HashMap::from(
                    [
                        ("galactic_poles.csv", [166, 107, 255, 255]),
                        ("first_point_of_aries.csv", [255, 155, 0, 255]),
                        ("galactic_centre_anticentre.csv", [146, 81, 245, 255]),
                        ("celestial_poles.csv", [217, 13, 13, 255]),
                        ("ecliptic_poles.csv", [107, 255, 107, 255]),
                    ]
                    .map(|(n, c)| (n.to_string(), Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]))),
                ),
                game_markers_colours: GameMarkersColours {
                    task: Color32::from_rgba_unmultiplied(255, 155, 0, 255),
                    correct_answer: Color32::from_rgba_unmultiplied(255, 155, 0, 255),
                    ..Default::default()
                },
                deepskies_colours: HashMap::from(
                    [("messier-catalogue.csv", [62, 211, 228, 255]), ("caldwell-catalogue.csv", [75, 227, 165, 255])]
                        .map(|(n, c)| (n.to_string(), Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]))),
                ),
            },
            egui_visuals,
        },
    );
    ThemesHandler::from_hash_map(themes)
}

impl Application {
    pub fn apply_theme(&mut self, ctx: &egui::Context, theme: Theme) {
        self.theme = theme;
        let mut deepskies_to_reinit = Vec::new();
        for (name, deepskies) in &mut self.cellestial_sphere.deepskies {
            match self.theme.game_visuals.deepskies_colours.get(name) {
                Some(colour) => {
                    deepskies.colour = *colour;
                    if deepskies.active {
                        deepskies_to_reinit.push(name.clone());
                    }
                }
                None => {
                    self.theme.game_visuals.deepskies_colours.insert(name.clone(), deepskies.colour);
                }
            }
        }
        for name in deepskies_to_reinit {
            self.cellestial_sphere.init_single_renderer(RendererCategory::Deepskies, &name);
        }
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
            self.cellestial_sphere.init_single_renderer(RendererCategory::Lines, &name);
        }
        let mut markers_to_reinit = Vec::new();
        for (name, markers) in &mut self.cellestial_sphere.markers {
            match self.theme.game_visuals.markers_colours.get(name) {
                Some(colour) => {
                    markers.colour = *colour;
                    if markers.active {
                        markers_to_reinit.push(name.clone());
                    }
                }
                None => {
                    self.theme.game_visuals.markers_colours.insert(name.clone(), markers.colour);
                }
            }
        }
        for name in markers_to_reinit {
            self.cellestial_sphere.init_single_renderer(RendererCategory::Markers, &name);
        }
        ctx.set_visuals(self.theme.egui_visuals.clone());
    }
}

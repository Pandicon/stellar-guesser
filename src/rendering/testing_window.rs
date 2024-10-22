use crate::Application;
use angle::Angle;
use eframe::egui;

impl Application {
    pub fn render_testing_window(&mut self, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>> {
        egui::Window::new("Testing").open(&mut self.state.windows.testing.opened).show(ctx, |ui| {
            let prev_selected_constellation = self.testing_settings.highlight_stars_in_constellation.clone();
            ui.horizontal(|ui| {
                ui.label("Highlight stars from constellation: ");
                egui::ComboBox::from_id_salt("Highlight stars from constellation: ")
                    .selected_text(self.testing_settings.highlight_stars_in_constellation.to_string())
                    .show_ui(ui, |ui: &mut egui::Ui| {
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                        let mut keys = self.cellestial_sphere.constellations.keys().map(|k| k.to_lowercase()).collect::<Vec<String>>();
                        keys.sort();
                        for key in keys {
                            ui.selectable_value(&mut self.testing_settings.highlight_stars_in_constellation, key.clone(), &key);
                        }
                    });
            });
            if prev_selected_constellation != self.testing_settings.highlight_stars_in_constellation {
                match self.cellestial_sphere.constellations.get(&self.testing_settings.highlight_stars_in_constellation) {
                    Some(constellation) => {
                        for category in self.cellestial_sphere.stars.values_mut() {
                            println!("Stars in category: {:?}", category.len());
                            'stars: for star in category {
                                // println!("{} {}", star.ra, star.dec);
                                let pos = spherical_geometry::SphericalPoint::new(star.ra.to_rad().value(), star.dec.to_rad().value());
                                for polygon in &constellation.polygons {
                                    match polygon.contains_point(&pos) {
                                        Ok(v) => {
                                            println!("Determined the constellation for star at dec={} ra={}", star.dec, star.ra);
                                            // println!("{}", v);
                                            if v {
                                                star.colour = egui::Color32::GREEN;
                                                continue 'stars;
                                            } else {
                                                star.colour = egui::Color32::WHITE;
                                            }
                                        }
                                        Err(_) => {
                                            println!("Could not determine the constellation for star at dec={} ra={}", star.dec, star.ra);
                                            star.colour = egui::Color32::RED;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        println!("Invalid constellation selected: {}", self.testing_settings.highlight_stars_in_constellation);
                    }
                }
            }

            let prev_selected_constellation = self.testing_settings.highlight_stars_in_constellation_precomputed.clone();
            ui.horizontal(|ui| {
                ui.label("Highlight stars from constellation, using precomputed values: ");
                egui::ComboBox::from_id_salt("Highlight stars from constellation, using precomputed values: ")
                    .selected_text(self.testing_settings.highlight_stars_in_constellation_precomputed.to_string())
                    .show_ui(ui, |ui: &mut egui::Ui| {
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                        let mut keys = self.cellestial_sphere.constellations.keys().map(|k| k.to_lowercase()).collect::<Vec<String>>();
                        keys.sort();
                        for key in keys {
                            ui.selectable_value(&mut self.testing_settings.highlight_stars_in_constellation_precomputed, key.clone(), &key);
                        }
                    });
            });

            if prev_selected_constellation != self.testing_settings.highlight_stars_in_constellation_precomputed {
                for category in self.cellestial_sphere.stars.values_mut() {
                    println!("Stars in category: {:?}", category.len());
                    for star in category {
                        if star
                            .constellations_abbreviations
                            .iter()
                            .any(|abbrev| abbrev.to_uppercase() == self.testing_settings.highlight_stars_in_constellation_precomputed.to_uppercase())
                        {
                            star.colour = egui::Color32::LIGHT_RED;
                        } else {
                            star.colour = egui::Color32::WHITE;
                        }
                    }
                }
            }
        })
    }
}

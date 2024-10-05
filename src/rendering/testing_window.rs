use crate::Application;
use std::f32::consts::PI;

impl Application {
    pub fn render_testing_window(&mut self, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>> {
        egui::Window::new("Testing").open(&mut self.state.windows.testing.opened).show(ctx, |ui| {
            let prev_selected_constellation = self.testing_settings.highlight_stars_in_constellation.clone();
            ui.horizontal(|ui| {
                ui.label("Highlight stars from constellation: ");
                egui::ComboBox::from_id_source("Highlight stars from constellation: ")
                    .selected_text(self.testing_settings.highlight_stars_in_constellation.to_string())
                    .show_ui(ui, |ui: &mut egui::Ui| {
                        ui.style_mut().wrap = Some(false);
                        let mut keys = self.cellestial_sphere.constellations.keys().map(|k| k.to_lowercase()).collect::<Vec<String>>();
                        keys.sort();
                        for key in keys {
                            ui.selectable_value(&mut self.testing_settings.highlight_stars_in_constellation, key.clone(), &key);
                        }
                    });
            });
            if prev_selected_constellation != self.testing_settings.highlight_stars_in_constellation {
                match self.cellestial_sphere.constellations.get_mut(&self.testing_settings.highlight_stars_in_constellation) {
                    Some(constellation) => {
                        let constellation_vertices = constellation
                            .vertices
                            .iter()
                            .map(|v| {
                                println!("{} {}", v.ra(), v.dec());
                                spherical_geometry::SphericalPoint::new(v.ra() * PI / 180.0, v.dec() * PI / 180.0)
                            })
                            .collect::<Vec<spherical_geometry::SphericalPoint>>();
                        match spherical_geometry::Polygon::new(constellation_vertices, spherical_geometry::EdgeDirection::CounterClockwise) {
                            Ok(polygon) => {
                                println!("{} polygon acquired: {:#?}", self.testing_settings.highlight_stars_in_constellation, polygon.vertices());
                                for category in self.cellestial_sphere.stars.values_mut() {
                                    println!("Stars in category: {:?}", category.len());
                                    for star in category {
                                        // println!("{} {}", star.ra, star.dec);
                                        let pos = spherical_geometry::SphericalPoint::new(star.ra * PI / 180.0, star.dec * PI / 180.0);
                                        match polygon.contains_point(&pos) {
                                            Ok(v) => {
                                                // println!("{}", v);
                                                if v {
                                                    star.colour = egui::Color32::GREEN;
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
                            Err(_) => {
                                println!("Failed to create the polygon for the {} constellation", self.testing_settings.highlight_stars_in_constellation);
                            }
                        }
                    }
                    None => {
                        println!("Invalid constellation selected: {}", self.testing_settings.highlight_stars_in_constellation);
                    }
                }
            }
        })
    }
}

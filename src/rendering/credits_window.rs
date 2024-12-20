use eframe::egui;

use crate::Application;

impl Application {
    pub fn render_credits_window(&mut self, ctx: &egui::Context) -> Option<egui::InnerResponse<Option<()>>> {
        egui::Window::new("Credits").open(&mut self.state.windows.credits.opened).show(ctx, |ui| {
            ui.label("This window contains credits for the different resources used by the application.");
            ui.separator();
            egui::ScrollArea::vertical().auto_shrink([false, true]).show(ui, |ui| {
                for (i, credits) in crate::CREDITS.iter().enumerate() {
                    ui.heading(&credits.name);

                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Copyright notice:"));
                        ui.label(&credits.copyright_notice);
                    });

                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Description:"));
                        ui.label(&credits.description);
                    });

                    if let Some(source_link) = &credits.source_link {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Source link:"));
                            ui.add(egui::Hyperlink::new(source_link).open_in_new_tab(true));
                        });
                    }

                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("License:"));
                        ui.label(&credits.license);
                    });

                    if let Some(license_link) = &credits.license_link {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("License link:"));
                            ui.add(egui::Hyperlink::new(license_link).open_in_new_tab(true));
                        });
                    }

                    if let Some(license_text) = &credits.license_text {
                        egui::CollapsingHeader::new("License text").id_salt(&credits.name).show(ui, |ui| {
                            ui.label(license_text);
                        });
                    }

                    if i != crate::CREDITS.len() - 1 {
                        ui.separator();
                    }
                }
            });
        })
    }
}

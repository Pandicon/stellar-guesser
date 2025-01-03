use std::collections::HashMap;

pub fn render_constellations_settings_subwindow(
    ui: &mut eframe::egui::Ui,
    state: &mut crate::GameConstellationsState,
    settings: &mut crate::GameConstellations,
    abbrev_to_name: HashMap<String, String>,
) {
    let prev_active_group = state.active_group.clone();
    ui.horizontal(|ui| {
        eframe::egui::ComboBox::from_label("Select group").selected_text(&state.active_group).show_ui(ui, |ui| {
            ui.style_mut().wrap_mode = Some(eframe::egui::TextWrapMode::Extend);
            let mut groups = settings.constellation_groups.keys().cloned().collect::<Vec<String>>();
            groups.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
            for group_name in groups {
                ui.selectable_value(&mut state.active_group, group_name.clone(), &group_name);
            }
        });
        let removed_group = ui
            .add_enabled_ui(settings.constellation_groups.keys().len() != 0, |ui| {
                if ui.button("Remove group").clicked() {
                    settings.constellation_groups.remove(&state.active_group);
                    state.active_group = String::new();
                    true
                } else {
                    false
                }
            })
            .inner;
        if !removed_group && state.active_group != prev_active_group {
            for (cons, toggle) in settings.constellation_groups.get(&state.active_group).unwrap() {
                settings.active_constellations.insert(cons.to_owned(), *toggle);
            }
            state.new_name = state.active_group.clone();
        }
    });
    ui.label(format!("Here you can customise the selected constellation group. You may rename it which will allow you to create a new group out of it.\nThe name may not contain any of the following characters: '{}', '{}'", crate::CONSTELLATIONS_SEPARATOR, crate::GROUPS_SEPARATOR));
    ui.horizontal(|ui| {
        ui.label("Group name");
        ui.text_edit_singleline(&mut state.new_name);
        state.new_name = crate::GameConstellations::sanitise_group_name(&state.new_name);
        let button = if settings.constellation_groups.contains_key(&state.new_name) {
            ui.button("Save group settings")
        } else {
            ui.button("Create a new group")
        };
        if button.clicked() {
            settings.constellation_groups.insert(state.new_name.clone(), settings.active_constellations.clone());
            state.active_group = state.new_name.clone();
        }
    });

    let mut constellations = settings
        .active_constellations
        .keys()
        .cloned()
        .filter_map(|abbrev| if let Some(name) = abbrev_to_name.get(&abbrev) { Some((abbrev, name.clone())) } else { None })
        .collect::<Vec<(String, String)>>();
    constellations.sort_by(|a, b| a.1.cmp(&b.1));
    ui.separator();
    ui.label("You can toggle all constellations using the buttons below.");
    ui.horizontal(|ui| {
        if ui.button("Select all").clicked() {
            for toggle in settings.active_constellations.values_mut() {
                *toggle = true;
            }
        }
        if ui.button("Unselect all").clicked() {
            for toggle in settings.active_constellations.values_mut() {
                *toggle = false;
            }
        }
    });
    ui.label(format!(
        "You can toggle specific constellations using the input below. Input all constellation abbreviations to toggle separated by '{}'.",
        crate::CONSTELLATIONS_SEPARATOR
    ));
    ui.horizontal(|ui| {
        ui.label("Constellations to toggle");
        ui.text_edit_singleline(&mut state.toggle_constellations);
        let (mut toggled, mut new_state) = if ui.button("Select").clicked() { (true, true) } else { (false, false) };
        if ui.button("Unselect").clicked() {
            toggled = true;
            new_state = false;
        }
        if toggled {
            for constellation_raw in state.toggle_constellations.split(crate::CONSTELLATIONS_SEPARATOR) {
                if let Some(constellation) = settings.active_constellations.keys().find(|con| con.to_lowercase() == constellation_raw.to_lowercase()) {
                    settings.active_constellations.insert(constellation.to_owned(), new_state);
                } else {
                    log::debug!("Invalid constellation entered: {}", constellation_raw);
                }
            }
        }
    });
    eframe::egui::CollapsingHeader::new("Advanced").default_open(false).show(ui, |ui| {
        let mut active = settings
            .active_constellations
            .iter()
            .filter_map(|(abbrev, t)| if *t { Some(abbrev.to_owned()) } else { None })
            .collect::<Vec<String>>();
        active.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        ui.label(format!("Currently active constellations: {}", active.join(&crate::CONSTELLATIONS_SEPARATOR.to_string())));
    });
    ui.separator();
    eframe::egui::ScrollArea::vertical().auto_shrink([false, true]).show(ui, |ui| {
        for (abbrev, name) in constellations {
            let text = format!("{name} ({abbrev})");
            let entry = settings.active_constellations.entry(abbrev.clone()).or_insert(true);
            ui.checkbox(entry, text).changed();
        }
    });
}

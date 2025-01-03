use std::collections::HashMap;

pub fn render_constellations_settings_subwindow(
    ui: &mut eframe::egui::Ui,
    state: &mut crate::GameConstellationsState,
    settings: &mut crate::GameConstellations,
    abbrev_to_name: HashMap<String, String>,
) {
    let prev_active_group = state.active_group.clone();
    eframe::egui::ComboBox::from_label("Select group").selected_text(&state.active_group).show_ui(ui, |ui| {
        ui.style_mut().wrap_mode = Some(eframe::egui::TextWrapMode::Extend);
        let mut groups = settings.constellation_groups.keys().cloned().collect::<Vec<String>>();
        groups.sort();
        for group_name in groups {
            ui.selectable_value(&mut state.active_group, group_name.clone(), &group_name);
        }
    });
    if state.active_group != prev_active_group {
        for (cons, toggle) in settings.constellation_groups.get(&state.active_group).unwrap() {
            settings.active_constellations.insert(cons.to_owned(), *toggle);
        }
        state.new_name = state.active_group.clone();
    }
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
    for (abbrev, name) in constellations {
        let text = format!("{name} ({abbrev})");
        let entry = settings.active_constellations.entry(abbrev.clone()).or_insert(true);
        ui.checkbox(entry, text).changed();
    }
}

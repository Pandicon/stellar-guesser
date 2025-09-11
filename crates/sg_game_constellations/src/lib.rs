pub mod ui;

use std::collections::HashMap;

const CONSTELLATIONS_STORAGE_KEY: &str = "active_constellations";
const GROUPS_STORAGE_KEY: &str = "constellation_groups";
const CONSTELLATIONS_SEPARATOR: char = '|';
const GROUPS_SEPARATOR: char = ';';

pub struct GameConstellations {
    pub active_constellations: HashMap<String, bool>,
    pub constellation_groups: HashMap<String, HashMap<String, bool>>,
}

impl GameConstellations {
    pub fn load_from_storage(storage: Option<&dyn eframe::Storage>, constellations: &Vec<String>) -> Self {
        let mut active_constellations = HashMap::new();
        let mut constellation_groups = HashMap::new();
        let mut all_active = HashMap::new();
        for constellation in constellations {
            all_active.insert(constellation.to_owned(), true);
        }
        if let Some(storage) = storage {
            for constellation in constellations {
                active_constellations.insert(constellation.to_owned(), false);
            }
            if let Some(active_constellations_str) = storage.get_string(CONSTELLATIONS_STORAGE_KEY) {
                let constellations_save = active_constellations_str.split(CONSTELLATIONS_SEPARATOR);
                for constellation_raw in constellations_save {
                    let constellation = if let Some(constellation) = constellations.iter().find(|s| s.to_lowercase() == constellation_raw.to_lowercase()) {
                        constellation.to_string()
                    } else {
                        log::warn!("Unknown constellation: {constellation_raw}");
                        continue;
                    };
                    active_constellations.insert(constellation, true);
                }
            }

            if let Some(groups_str) = storage.get_string(GROUPS_STORAGE_KEY) {
                if !groups_str.is_empty() {
                    let groups = groups_str.split(GROUPS_SEPARATOR).collect::<Vec<&str>>();
                    for group_raw in &groups {
                        let mut group_spl = group_raw.split(CONSTELLATIONS_SEPARATOR);
                        let group_name = group_spl.next();
                        if group_name.is_none() {
                            log::warn!("An empty constellation group was found");
                            continue;
                        }
                        let group_name = group_name.unwrap();

                        let mut active = HashMap::new();
                        for constellation in constellations {
                            active.insert(constellation.to_owned(), false);
                        }
                        for constellation_raw in group_spl {
                            let constellation = if let Some(constellation) = constellations.iter().find(|s| s.to_lowercase() == constellation_raw.to_lowercase()) {
                                constellation.to_string()
                            } else {
                                log::warn!("Unknown constellation: {constellation_raw}");
                                continue;
                            };
                            active.insert(constellation, true);
                        }
                        constellation_groups.insert(group_name.to_string(), active);
                    }
                }
            }
        } else {
            active_constellations = all_active.clone();
        }
        if constellation_groups.is_empty() {
            constellation_groups.insert(String::from("Not started"), all_active);
        }
        Self {
            active_constellations,
            constellation_groups,
        }
    }

    pub fn save_to_storage(&self, storage: &mut dyn eframe::Storage) {
        fn hashmap_to_string_parts(hashmap: &HashMap<String, bool>) -> Vec<String> {
            hashmap.iter().filter_map(|(c, t)| if *t { Some(c.to_owned()) } else { None }).collect::<Vec<String>>()
        }
        let active_constellations = hashmap_to_string_parts(&self.active_constellations).join(&CONSTELLATIONS_SEPARATOR.to_string());
        storage.set_string(CONSTELLATIONS_STORAGE_KEY, active_constellations);

        let mut groups_strs = Vec::new();
        for (group_name, group_data) in &self.constellation_groups {
            let mut active_constellations_vec = hashmap_to_string_parts(group_data);
            active_constellations_vec.insert(0, Self::sanitise_group_name(group_name));
            groups_strs.push(active_constellations_vec.join(&CONSTELLATIONS_SEPARATOR.to_string()));
        }
        storage.set_string(GROUPS_STORAGE_KEY, groups_strs.join(&GROUPS_SEPARATOR.to_string()));
    }

    pub fn sanitise_group_name(name: &str) -> String {
        name.replace([CONSTELLATIONS_SEPARATOR, GROUPS_SEPARATOR], "")
    }
}

pub struct GameConstellationsState {
    pub active_group: String,

    pub new_name: String,
    pub toggle_constellations: String,
}

impl Default for GameConstellationsState {
    fn default() -> Self {
        Self {
            active_group: String::new(),

            new_name: String::from("Custom"),
            toggle_constellations: String::new(),
        }
    }
}

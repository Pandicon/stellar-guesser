static LICENSES_DIR: include_dir::Dir = include_dir::include_dir!("$CARGO_MANIFEST_DIR/../../licenses_generic");

#[derive(serde::Deserialize)]
pub struct CreditsRaw {
    pub name: String,
    pub description: String,
    pub source_link: Option<String>,
    pub copyright_notice: String,
    pub license: String,
    pub license_link: Option<String>,
    pub license_text: Option<String>,
    pub license_file: Option<String>, // Relative to './licenses_generic', will get inserted into 'license_text'
}

pub struct Credits {
    pub name: String,
    pub description: String,
    pub source_link: Option<String>,
    pub copyright_notice: String,
    pub license: String,
    pub license_link: Option<String>,
    pub license_text: Option<String>,
}

impl Credits {
    fn from_raw(raw: CreditsRaw) -> Self {
        let text = if raw.license_text.is_some() {
            raw.license_text
        } else if let Some(license_file) = raw.license_file {
            if let Some(file) = LICENSES_DIR.get_file(&license_file) {
                if let Some(contents) = file.contents_utf8().map(|s| s.to_owned()) {
                    Some(contents)
                } else {
                    log::warn!("The license file for {} ({}) could not be read as UTF-8.", raw.name, license_file);
                    None
                }
            } else {
                log::warn!("The license file for {} ({}) was not found.", raw.name, license_file);
                None
            }
        } else {
            None
        };
        Self {
            name: raw.name,
            description: raw.description,
            source_link: raw.source_link,
            copyright_notice: raw.copyright_notice,
            license: raw.license,
            license_link: raw.license_link,
            license_text: text,
        }
    }

    fn non_empty_license(&self) -> bool {
        if let Some(link) = &self.license_link {
            if !link.trim().is_empty() {
                return true;
            }
        }
        if let Some(text) = &self.license_text {
            if !text.trim().is_empty() {
                return true;
            }
        }
        false
    }
}

pub fn get_credits() -> Vec<Credits> {
    let data = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../credits.json"));
    let res_raw: Vec<CreditsRaw> = serde_json::from_str(data).expect("Unable to parse the credits file.");
    log::info!("Successfully loaded the credits file");
    res_raw
        .into_iter()
        .map(|raw| {
            let credits = Credits::from_raw(raw);
            if !credits.non_empty_license() {
                log::warn!("The credits for {} are missing a license.", credits.name);
            };
            credits
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn all_credits_have_some_license() {
        let credits_list = super::get_credits();
        for credits in credits_list {
            assert!(credits.non_empty_license());
        }
    }
}

pub struct VersionInformation {
    pub current_version: String,
    pub update_available: Option<bool>,
    /// This is the latest version for the platform that has been released OR the user is running. So if the user runs version 1.3.1, but the latest release is 1.3.0, this field will hold 1.3.1
    pub latest_version: Option<String>,
    /// This is the latest version for the platform that has been released. So if the user runs version 1.3.1, but the latest release is 1.3.0, this field will hold 1.3.0
    pub latest_released_version: Option<String>,
}

impl VersionInformation {
    pub fn only_current(current_version: String) -> Self {
        Self {
            current_version,
            update_available: None,
            latest_version: None,
            latest_released_version: None,
        }
    }
}

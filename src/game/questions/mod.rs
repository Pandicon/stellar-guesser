mod angular_separation;
mod find_this_object;
mod guess_ra_dec;
mod guess_the_magnitude;
mod which_constellation_is_point_in;
mod which_object_is_here;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Settings {
    pub angular_separation: angular_separation::Settings,
    pub find_this_object: find_this_object::Settings,
    pub guess_rad_dec: guess_ra_dec::Settings,
    pub guess_the_magnitude: guess_the_magnitude::Settings,
    pub what_constellation_is_this_point_in: which_constellation_is_point_in::Settings,
    pub what_is_this_object: which_object_is_here::Settings,
}

#[allow(clippy::derivable_impls)]
impl Default for Settings {
    fn default() -> Self {
        Self {
            angular_separation: angular_separation::Settings::default(),
            find_this_object: find_this_object::Settings::default(),
            guess_rad_dec: guess_ra_dec::Settings::default(),
            guess_the_magnitude: guess_the_magnitude::Settings::default(),
            what_constellation_is_this_point_in: which_constellation_is_point_in::Settings::default(),
            what_is_this_object: which_object_is_here::Settings::default(),
        }
    }
}

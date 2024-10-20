use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct StarName {
    pub ra: angle::Deg<f32>,
    pub dec: angle::Deg<f32>,
    pub mag: f32,
    pub name: String,
    pub con: String,
    pub id: Option<String>,
    pub id_greek: Option<String>,
    pub hip: f32,
}

#[derive(Clone, Deserialize)]
pub struct StarNameRaw {
    #[allow(dead_code)]
    name_ascii: String,
    name_diacritics: String,
    #[allow(dead_code)]
    designation: String,
    id: Option<String>,
    id_greek: Option<String>,
    con: String,
    #[allow(dead_code)]
    num: Option<String>,
    #[allow(dead_code)]
    wsd_j: Option<String>,
    mag: Option<f32>,
    #[allow(dead_code)]
    bnd: Option<String>,
    hip: Option<String>,
    #[allow(dead_code)]
    hd: Option<String>,
    ra: angle::Deg<f32>,
    dec: angle::Deg<f32>,
    #[allow(dead_code)]
    date: String,
}

impl StarName {
    pub fn from_raw(raw_star: StarNameRaw) -> Option<Self> {
        match raw_star.hip {
            Some(hipstr) => {
                let hip = hipstr.parse().expect("Invalid HIP number!");
                match raw_star.mag {
                    Some(mag) => Some(Self {
                        ra: raw_star.ra,
                        dec: raw_star.dec,
                        name: raw_star.name_diacritics,
                        con: raw_star.con,
                        id: raw_star.id,
                        id_greek: raw_star.id_greek,
                        hip,
                        mag,
                    }),
                    None => None,
                }
            }
            None => None,
        }
    }
}

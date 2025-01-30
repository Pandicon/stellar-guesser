use angle::Angle;

pub mod parser;

// TODO: Store all objects in the same struct and have this function for that struct
// Only put it into the Star and Deepsky structs for rendering
pub fn check_star(expression: &parser::Keyword, star: &crate::rendering::caspr::stars::Star) -> bool {
    match expression {
        parser::Keyword::And(expressions) => expressions.iter().all(|expression| check_star(expression, star)),
        parser::Keyword::Or(expressions) => expressions.iter().any(|expression| check_star(expression, star)),
        parser::Keyword::Not(expression) => !check_star(&expression, star),
        &parser::Keyword::Dec(min, max) => (min..=max).contains(star.dec.as_value()),
        &parser::Keyword::Ra(min, max) => (min..=max).contains(&(star.ra.as_value() / 15.0)),
        &parser::Keyword::RaDeg(min, max) => (min..=max).contains(star.ra.as_value()),
        parser::Keyword::Constellation(constellations) => constellations.iter().any(|constellation| {
            star.constellations_abbreviations
                .iter()
                .map(|abbrev| abbrev.to_lowercase())
                .collect::<Vec<String>>()
                .contains(&constellation.to_lowercase())
        }),
        parser::Keyword::Catalogue(catalogues) => catalogues.iter().any(|catalogue| match catalogue {
            &parser::Catalogue::Caldwell | &parser::Catalogue::Messier | &parser::Catalogue::Ngc | &parser::Catalogue::Hd | &parser::Catalogue::Hip | &parser::Catalogue::ProperName => false, // TODO: Make this work :D
        }),
    }
}

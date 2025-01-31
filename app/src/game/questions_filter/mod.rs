use crate::game::QuestionObject;
use angle::Angle;

pub mod parser;

pub fn check(expression: &parser::Keyword, object: &QuestionObject) -> bool {
    match expression {
        parser::Keyword::And(expressions) => expressions.iter().all(|expression| check(expression, object)),
        parser::Keyword::Or(expressions) => expressions.iter().any(|expression| check(expression, object)),
        parser::Keyword::Not(expression) => !check(&expression, object),
        &parser::Keyword::Dec(min, max) => (min..=max).contains(object.dec.as_value()),
        &parser::Keyword::Ra(min, max) => (min..=max).contains(&(object.ra.as_value() / 15.0)),
        &parser::Keyword::RaDeg(min, max) => (min..=max).contains(object.ra.as_value()),
        parser::Keyword::Constellation(constellations) => constellations.iter().any(|constellation| {
            object
                .constellations_abbreviations
                .iter()
                .map(|abbrev| abbrev.to_lowercase())
                .collect::<Vec<String>>()
                .contains(&constellation.to_lowercase())
        }),
        parser::Keyword::Catalogue(catalogues) => catalogues.iter().any(|catalogue| match catalogue {
            &parser::Catalogue::Bayer => object.bayer_designation.is_some(),
            &parser::Catalogue::Flamsteed => object.flamsteed_designation.is_some(),
            &parser::Catalogue::Caldwell => object.caldwell_number.is_some(),
            &parser::Catalogue::Messier => object.messier_number.is_some(),
            &parser::Catalogue::Ngc => object.ngc_number.is_some(),
            &parser::Catalogue::Hd => object.hd_number.is_some(),
            &parser::Catalogue::Hip => object.hipparcos_number.is_some(),
            &parser::Catalogue::ProperName => !object.proper_names.is_empty(),
        }),
    }
}

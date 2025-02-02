use crate::game::QuestionObject;
use angle::Angle;

pub mod parser;

pub struct QuestionPack {
    pub query: String,
    pub question_objects: Vec<(crate::game::questions::QuestionType, Vec<u64>)>,
}

pub fn check(expression: &parser::Keyword, object: &QuestionObject) -> bool {
    match expression {
        parser::Keyword::And(expressions) => expressions.iter().all(|expression| check(expression, object)),
        parser::Keyword::Or(expressions) => expressions.iter().any(|expression| check(expression, object)),
        parser::Keyword::Not(expression) => !check(expression, object),
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
        parser::Keyword::Catalogue(catalogues) => catalogues.iter().any(|catalogue| match *catalogue {
            parser::Catalogue::Bayer => object.bayer_designation.is_some(),
            parser::Catalogue::Flamsteed => object.flamsteed_designation.is_some(),
            parser::Catalogue::Caldwell => object.caldwell_number.is_some(),
            parser::Catalogue::Messier => object.messier_number.is_some(),
            parser::Catalogue::Ngc => object.ngc_number.is_some(),
            parser::Catalogue::Hd => object.hd_number.is_some(),
            parser::Catalogue::Hip => object.hipparcos_number.is_some(),
            parser::Catalogue::ProperName => !object.proper_names.is_empty(),
        }),
        parser::Keyword::CatalogueDesignation(catalogue_designations) => catalogue_designations.iter().any(|(catalogue, designation)| match *catalogue {
            parser::Catalogue::Bayer => object.bayer_designation.as_ref() == Some(designation),
            parser::Catalogue::Flamsteed => object.flamsteed_designation.as_ref() == Some(designation),
            parser::Catalogue::Caldwell => object.caldwell_number == designation.parse().ok(),
            parser::Catalogue::Messier => object.messier_number == designation.parse().ok(),
            parser::Catalogue::Ngc => object.ngc_number == designation.parse().ok(),
            parser::Catalogue::Hd => object.hd_number == designation.parse().ok(),
            parser::Catalogue::Hip => object.hipparcos_number == designation.parse().ok(),
            parser::Catalogue::ProperName => object.proper_names.iter().any(|s| s.trim().to_lowercase() == designation.trim().to_lowercase()),
        }),
        parser::Keyword::Type(object_types) => object_types.iter().any(|object_type| match *object_type {
            crate::game::ObjectType::Star(crate::game::StarType::Any) => matches!(object.object_type, crate::game::ObjectType::Star(_)),
            crate::game::ObjectType::Deepsky(crate::game::DeepskyType::Any) => matches!(object.object_type, crate::game::ObjectType::Deepsky(_)),

            crate::game::ObjectType::Star(crate::game::StarType::Single) => matches!(object.object_type, crate::game::ObjectType::Star(crate::game::StarType::Single)),
            crate::game::ObjectType::Star(crate::game::StarType::Double) => matches!(object.object_type, crate::game::ObjectType::Star(crate::game::StarType::Double)),
            crate::game::ObjectType::Star(crate::game::StarType::Multiple) => matches!(object.object_type, crate::game::ObjectType::Star(crate::game::StarType::Multiple)),
            crate::game::ObjectType::Star(crate::game::StarType::Unknown) => matches!(object.object_type, crate::game::ObjectType::Star(crate::game::StarType::Unknown)),

            crate::game::ObjectType::Deepsky(crate::game::DeepskyType::DarkNebula) => matches!(object.object_type, crate::game::ObjectType::Deepsky(crate::game::DeepskyType::DarkNebula)),
            crate::game::ObjectType::Deepsky(crate::game::DeepskyType::DiffuseNebula) => matches!(object.object_type, crate::game::ObjectType::Deepsky(crate::game::DeepskyType::DiffuseNebula)),
            crate::game::ObjectType::Deepsky(crate::game::DeepskyType::Nebula) => matches!(object.object_type, crate::game::ObjectType::Deepsky(crate::game::DeepskyType::Nebula)),
            crate::game::ObjectType::Deepsky(crate::game::DeepskyType::PlanetaryNebula) => matches!(object.object_type, crate::game::ObjectType::Deepsky(crate::game::DeepskyType::PlanetaryNebula)),
            crate::game::ObjectType::Deepsky(crate::game::DeepskyType::OpenCluster) => matches!(object.object_type, crate::game::ObjectType::Deepsky(crate::game::DeepskyType::OpenCluster)),
            crate::game::ObjectType::Deepsky(crate::game::DeepskyType::GlobularCluster) => matches!(object.object_type, crate::game::ObjectType::Deepsky(crate::game::DeepskyType::GlobularCluster)),
            crate::game::ObjectType::Deepsky(crate::game::DeepskyType::Galaxy) => matches!(object.object_type, crate::game::ObjectType::Deepsky(crate::game::DeepskyType::Galaxy)),
            crate::game::ObjectType::Deepsky(crate::game::DeepskyType::SupernovaRemnant) => matches!(object.object_type, crate::game::ObjectType::Deepsky(crate::game::DeepskyType::SupernovaRemnant)),
            crate::game::ObjectType::Deepsky(crate::game::DeepskyType::StarCloud) => matches!(object.object_type, crate::game::ObjectType::Deepsky(crate::game::DeepskyType::StarCloud)),
            crate::game::ObjectType::Deepsky(crate::game::DeepskyType::Unknown) => matches!(object.object_type, crate::game::ObjectType::Deepsky(crate::game::DeepskyType::Unknown)),
        }),
        &parser::Keyword::Mag(min, max) => {
            if let Some(mag) = object.mag {
                (min..=max).contains(&mag)
            } else {
                false
            }
        }
        &parser::Keyword::MagAbove(val) => {
            if let Some(mag) = object.mag {
                val < mag
            } else {
                false
            }
        }
        &parser::Keyword::MagBelow(val) => {
            if let Some(mag) = object.mag {
                val > mag
            } else {
                false
            }
        }
        &parser::Keyword::ObjectId(object_id) => object.object_id == object_id,
    }
}

use crate::game::QuestionObject;
use angle::Angle;

use super::questions::{find_this_object, which_object_is_here, QuestionType};

pub mod parser;

pub struct QuestionPack {
    pub query: String,
    pub question_objects: Vec<(crate::game::questions::QuestionType, Vec<u64>)>,
    pub description: String,
    pub file_path: Option<String>,
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
            parser::Catalogue::Bayer => object.bayer_designation_full.is_some(),
            parser::Catalogue::Flamsteed => object.flamsteed_designation_full.is_some(),
            parser::Catalogue::Caldwell => object.caldwell_number.is_some(),
            parser::Catalogue::Messier => object.messier_number.is_some(),
            parser::Catalogue::Ic => object.ic_number.is_some(),
            parser::Catalogue::Ngc => object.ngc_number.is_some(),
            parser::Catalogue::Hd => object.hd_number.is_some(),
            parser::Catalogue::Hip => object.hipparcos_number.is_some(),
            parser::Catalogue::ProperName => !object.proper_names_all.is_empty(),
        }),
        parser::Keyword::CatalogueDesignation(catalogue_designations) => catalogue_designations.iter().any(|(catalogue, designation)| match *catalogue {
            parser::Catalogue::Bayer => {
                if let Some(raw) = &object.bayer_designation_raw {
                    let names: Vec<String> = crate::rendering::caspr::generate_name_combinations(raw, crate::rendering::caspr::SpecificName::None)
                        .iter()
                        .map(|name| name.to_lowercase())
                        .collect();
                    names.contains(&designation.to_lowercase())
                } else {
                    false
                }
            }
            parser::Catalogue::Flamsteed => {
                if let Some(raw) = &object.flamsteed_designation_raw {
                    let names: Vec<String> = crate::rendering::caspr::generate_name_combinations(raw, crate::rendering::caspr::SpecificName::None)
                        .iter()
                        .map(|name| name.to_lowercase())
                        .collect();
                    names.contains(&designation.to_lowercase())
                } else {
                    false
                }
            }
            parser::Catalogue::Caldwell => object.caldwell_number == designation.parse().ok(),
            parser::Catalogue::Messier => object.messier_number == designation.parse().ok(),
            parser::Catalogue::Ic => object.ic_number == designation.parse().ok(),
            parser::Catalogue::Ngc => object.ngc_number == designation.parse().ok(),
            parser::Catalogue::Hd => object.hd_number == designation.parse().ok(),
            parser::Catalogue::Hip => object.hipparcos_number == designation.parse().ok(),
            parser::Catalogue::ProperName => object.proper_names_all.iter().any(|s| s.trim().to_lowercase() == designation.trim().to_lowercase()),
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

pub fn default_packs() -> [(String, QuestionPack); 9] {
    [
        (
            String::from(r#"Mark Messiers (accurately)"#),
            QuestionPack {
                query: String::from(concat!(
                    r#"FIND_THIS_OBJECT({"correctness_threshold":1.0,"rotate_to_answer":true,"replay_incorrect":true,"ask_messier":true,"ask_caldwell":false,"ask_ic":false,"ask_ngc":false,"ask_hd":false,"ask_hip":false,"ask_bayer":false,"ask_flamsteed":false,"ask_proper":false}): CATALOGUE(MESSIER)"#
                )),
                question_objects: vec![(
                    QuestionType::FindThisObject(find_this_object::SmallSettings {
                        correctness_threshold: angle::Deg(1.0),
                        rotate_to_answer: true,
                        replay_incorrect: true,
                        ask_messier: true,
                        ask_caldwell: false,
                        ask_ic: false,
                        ask_ngc: false,
                        ask_hd: false,
                        ask_hip: false,
                        ask_bayer: false,
                        ask_flamsteed: false,
                        ask_proper: false,
                    }),
                    vec![
                        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
                        47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
                        91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
                    ],
                )],
                description: String::from(r#"Asks the player to mark all Messier objects with 1 degree of tolerance"#),
                file_path: None,
            },
        ),
        (
            String::from(r#"Mark Messiers (reasonably accurately)"#),
            QuestionPack {
                query: String::from(
                    r#"FIND_THIS_OBJECT({"correctness_threshold":2.0,"rotate_to_answer":true,"replay_incorrect":true,"ask_messier":true,"ask_caldwell":false,"ask_ic":false,"ask_ngc":false,"ask_hd":false,"ask_hip":false,"ask_bayer":false,"ask_flamsteed":false,"ask_proper":false}): CATALOGUE(MESSIER)"#,
                ),
                question_objects: vec![(
                    QuestionType::FindThisObject(find_this_object::SmallSettings {
                        correctness_threshold: angle::Deg(2.0),
                        rotate_to_answer: true,
                        replay_incorrect: true,
                        ask_messier: true,
                        ask_caldwell: false,
                        ask_ic: false,
                        ask_ngc: false,
                        ask_hd: false,
                        ask_hip: false,
                        ask_bayer: false,
                        ask_flamsteed: false,
                        ask_proper: false,
                    }),
                    vec![
                        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
                        47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
                        91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
                    ],
                )],
                description: String::from(r#"Asks the player to mark all Messier objects with 2 degrees of tolerance"#),
                file_path: None,
            },
        ),
        (
            String::from(r#"Mark Messiers (roughly)"#),
            QuestionPack {
                query: String::from(
                    r#"FIND_THIS_OBJECT({"correctness_threshold":4.0,"rotate_to_answer":true,"replay_incorrect":true,"ask_messier":true,"ask_caldwell":false,"ask_ic":false,"ask_ngc":false,"ask_hd":false,"ask_hip":false,"ask_bayer":false,"ask_flamsteed":false,"ask_proper":false}): CATALOGUE(MESSIER)"#,
                ),
                question_objects: vec![(
                    QuestionType::FindThisObject(find_this_object::SmallSettings {
                        correctness_threshold: angle::Deg(4.0),
                        rotate_to_answer: true,
                        replay_incorrect: true,
                        ask_messier: true,
                        ask_caldwell: false,
                        ask_ic: false,
                        ask_ngc: false,
                        ask_hd: false,
                        ask_hip: false,
                        ask_bayer: false,
                        ask_flamsteed: false,
                        ask_proper: false,
                    }),
                    vec![
                        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
                        47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
                        91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
                    ],
                )],
                description: String::from(r#"Asks the player to mark all Messier objects with 4 degrees of tolerance"#),
                file_path: None,
            },
        ),
        (
            String::from(r#"Mark Messiers (very accurately)"#),
            QuestionPack {
                query: String::from(
                    r#"FIND_THIS_OBJECT({"correctness_threshold":0.5,"rotate_to_answer":true,"replay_incorrect":true,"ask_messier":true,"ask_caldwell":false,"ask_ic":false,"ask_ngc":false,"ask_hd":false,"ask_hip":false,"ask_bayer":false,"ask_flamsteed":false,"ask_proper":false}): CATALOGUE(MESSIER)"#,
                ),
                question_objects: vec![(
                    QuestionType::FindThisObject(find_this_object::SmallSettings {
                        correctness_threshold: angle::Deg(0.5),
                        rotate_to_answer: true,
                        replay_incorrect: true,
                        ask_messier: true,
                        ask_caldwell: false,
                        ask_ic: false,
                        ask_ngc: false,
                        ask_hd: false,
                        ask_hip: false,
                        ask_bayer: false,
                        ask_flamsteed: false,
                        ask_proper: false,
                    }),
                    vec![
                        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
                        47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
                        91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
                    ],
                )],
                description: String::from(r#"Asks the player to mark all Messier objects with 0.5 degrees of tolerance"#),
                file_path: None,
            },
        ),
        (
            String::from(r#"Recognise marked Messiers"#),
            QuestionPack {
                query: String::from(
                    r#"WHAT_IS_THIS_OBJECT({"rotate_to_point":true,"replay_incorrect":true,"accept_messier":true,"accept_caldwell":false,"accept_ngc":false,"accept_ic":false,"accept_hip":false,"accept_hd":false,"accept_proper":false,"accept_bayer":false,"accept_flamsteed":false}): CATALOGUE(MESSIER)"#,
                ),
                question_objects: vec![(
                    QuestionType::WhatIsThisObject(which_object_is_here::SmallSettings {
                        rotate_to_point: true,
                        replay_incorrect: true,
                        accept_messier: true,
                        accept_caldwell: false,
                        accept_ngc: false,
                        accept_ic: false,
                        accept_hip: false,
                        accept_hd: false,
                        accept_proper: false,
                        accept_bayer: false,
                        accept_flamsteed: false,
                    }),
                    vec![
                        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
                        47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
                        91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110,
                    ],
                )],
                description: String::from(r#"Tests the ability to recognise which Messier object is marked in the sky"#),
                file_path: None,
            },
        ),
        (
            String::from(r#"Stars (advanced)"#),
            QuestionPack {
                query: String::from(concat!(
                    r#"FIND_THIS_OBJECT({"correctness_threshold":1.0,"rotate_to_answer":true,"replay_incorrect":true,"ask_messier":false,"ask_caldwell":false,"ask_ic":false,"ask_ngc":false,"ask_hd":false,"ask_hip":false,"ask_bayer":true,"ask_flamsteed":false,"ask_proper":true}): AND(TYPE(STAR), MAG_BELOW(2.0), OR(CATALOGUE(PROPER_NAME), CATALOGUE(BAYER), CATALOGUE(FLAMSTEED)))"#,
                    "\n",
                    r#"WHAT_IS_THIS_OBJECT({"rotate_to_point":true,"replay_incorrect":true,"accept_messier":false,"accept_caldwell":false,"accept_ngc":false,"accept_ic":false,"accept_hip":true,"accept_hd":true,"accept_proper":true,"accept_bayer":true,"accept_flamsteed":true}): AND(TYPE(STAR), MAG_BELOW(2.0), OR(CATALOGUE(PROPER_NAME), CATALOGUE(BAYER), CATALOGUE(FLAMSTEED)))"#
                )),
                question_objects: vec![
                    (
                        QuestionType::FindThisObject(find_this_object::SmallSettings {
                            correctness_threshold: angle::Deg(1.0),
                            rotate_to_answer: true,
                            replay_incorrect: true,
                            ask_messier: false,
                            ask_caldwell: false,
                            ask_ic: false,
                            ask_ngc: false,
                            ask_hd: false,
                            ask_hip: false,
                            ask_bayer: true,
                            ask_flamsteed: false,
                            ask_proper: true,
                        }),
                        vec![
                            7793, 11953, 16044, 21587, 24597, 24768, 25494, 25585, 26465, 26881, 28138, 28507, 30470, 30584, 31821, 32484, 33712, 34573, 36964, 37392, 37938, 40059, 41140, 43013,
                            45325, 46479, 49748, 54127, 60753, 61116, 62460, 62979, 65490, 67309, 68704, 69672, 71675, 71678, 80740, 82243, 85886, 86186, 90127, 91200, 97559, 100646, 101988, 109144,
                            113229,
                        ],
                    ),
                    (
                        QuestionType::WhatIsThisObject(which_object_is_here::SmallSettings {
                            rotate_to_point: true,
                            replay_incorrect: true,
                            accept_messier: false,
                            accept_caldwell: false,
                            accept_ngc: false,
                            accept_ic: false,
                            accept_hip: true,
                            accept_hd: true,
                            accept_proper: true,
                            accept_bayer: true,
                            accept_flamsteed: true,
                        }),
                        vec![
                            7793, 11953, 16044, 21587, 24597, 24768, 25494, 25585, 26465, 26881, 28138, 28507, 30470, 30584, 31821, 32484, 33712, 34573, 36964, 37392, 37938, 40059, 41140, 43013,
                            45325, 46479, 49748, 54127, 60753, 61116, 62460, 62979, 65490, 67309, 68704, 69672, 71675, 71678, 80740, 82243, 85886, 86186, 90127, 91200, 97559, 100646, 101988, 109144,
                            113229,
                        ],
                    ),
                ],
                description: String::from(r#"Tests the knowledge of any designation of stars of up to magnitude 2.0"#),
                file_path: None,
            },
        ),
        (
            String::from(r#"Stars (basic)"#),
            QuestionPack {
                query: String::from(concat!(
                    r#"FIND_THIS_OBJECT({"correctness_threshold":1.0,"rotate_to_answer":true,"replay_incorrect":true,"ask_messier":false,"ask_caldwell":false,"ask_ic":false,"ask_ngc":false,"ask_hd":false,"ask_hip":false,"ask_bayer":false,"ask_flamsteed":false,"ask_proper":true}): AND(TYPE(STAR), MAG_BELOW(1), CATALOGUE(PROPER_NAME))"#,
                    "\n",
                    r#"WHAT_IS_THIS_OBJECT({"rotate_to_point":true,"replay_incorrect":true,"accept_messier":false,"accept_caldwell":false,"accept_ngc":false,"accept_ic":false,"accept_hip":true,"accept_hd":true,"accept_proper":true,"accept_bayer":true,"accept_flamsteed":true}): AND(TYPE(STAR), MAG_BELOW(1), CATALOGUE(PROPER_NAME))"#
                )),
                question_objects: vec![
                    (
                        QuestionType::FindThisObject(find_this_object::SmallSettings {
                            correctness_threshold: angle::Deg(1.0),
                            rotate_to_answer: true,
                            replay_incorrect: true,
                            ask_messier: false,
                            ask_caldwell: false,
                            ask_ic: false,
                            ask_ngc: false,
                            ask_hd: false,
                            ask_hip: false,
                            ask_bayer: false,
                            ask_flamsteed: false,
                            ask_proper: true,
                        }),
                        vec![7793, 21587, 24597, 24768, 28138, 30584, 32484, 37392, 60753, 65490, 68704, 69672, 71678, 91200, 97559],
                    ),
                    (
                        QuestionType::WhatIsThisObject(which_object_is_here::SmallSettings {
                            rotate_to_point: true,
                            replay_incorrect: true,
                            accept_messier: false,
                            accept_caldwell: false,
                            accept_ngc: false,
                            accept_ic: false,
                            accept_hip: true,
                            accept_hd: true,
                            accept_proper: true,
                            accept_bayer: true,
                            accept_flamsteed: true,
                        }),
                        vec![7793, 21587, 24597, 24768, 28138, 30584, 32484, 37392, 60753, 65490, 68704, 69672, 71678, 91200, 97559],
                    ),
                ],
                description: String::from(r#"Tests the knowledge of any designation of stars of up to magnitude 1"#),
                file_path: None,
            },
        ),
        (
            String::from(r#"Stars (intermediate)"#),
            QuestionPack {
                query: String::from(concat!(
                    r#"FIND_THIS_OBJECT({"correctness_threshold":1.0,"rotate_to_answer":true,"replay_incorrect":true,"ask_messier":false,"ask_caldwell":false,"ask_ic":false,"ask_ngc":false,"ask_hd":false,"ask_hip":false,"ask_bayer":false,"ask_flamsteed":false,"ask_proper":true}): AND(TYPE(STAR), MAG_BELOW(1.5), OR(CATALOGUE(PROPER_NAME), CATALOGUE(BAYER), CATALOGUE(FLAMSTEED)))"#,
                    "\n",
                    r#"WHAT_IS_THIS_OBJECT({"rotate_to_point":true,"replay_incorrect":true,"accept_messier":false,"accept_caldwell":false,"accept_ngc":false,"accept_ic":false,"accept_hip":true,"accept_hd":true,"accept_proper":true,"accept_bayer":true,"accept_flamsteed":true}): AND(TYPE(STAR), MAG_BELOW(1.5), OR(CATALOGUE(PROPER_NAME), CATALOGUE(BAYER), CATALOGUE(FLAMSTEED)))"#
                )),
                question_objects: vec![
                    (
                        QuestionType::FindThisObject(find_this_object::SmallSettings {
                            correctness_threshold: angle::Deg(1.0),
                            rotate_to_answer: true,
                            replay_incorrect: true,
                            ask_messier: false,
                            ask_caldwell: false,
                            ask_ic: false,
                            ask_ngc: false,
                            ask_hd: false,
                            ask_hip: false,
                            ask_bayer: false,
                            ask_flamsteed: false,
                            ask_proper: true,
                        }),
                        vec![
                            7793, 21587, 24597, 24768, 28138, 30584, 32484, 37392, 37938, 49748, 60753, 62460, 65490, 68704, 69672, 71675, 71678, 80740, 91200, 97559, 101988, 113229,
                        ],
                    ),
                    (
                        QuestionType::WhatIsThisObject(which_object_is_here::SmallSettings {
                            rotate_to_point: true,
                            replay_incorrect: true,
                            accept_messier: false,
                            accept_caldwell: false,
                            accept_ngc: false,
                            accept_ic: false,
                            accept_hip: true,
                            accept_hd: true,
                            accept_proper: true,
                            accept_bayer: true,
                            accept_flamsteed: true,
                        }),
                        vec![
                            7793, 21587, 24597, 24768, 28138, 30584, 32484, 37392, 37938, 49748, 60753, 62460, 65490, 68704, 69672, 71675, 71678, 80740, 91200, 97559, 101988, 113229,
                        ],
                    ),
                ],
                description: String::from(r#"Tests the knowledge of any designation of stars of up to magnitude 1.5"#),
                file_path: None,
            },
        ),
        (
            String::from(r#"Stars (IOAA)"#),
            QuestionPack {
                query: String::from(concat!(
                    r#"FIND_THIS_OBJECT({"correctness_threshold":1.0,"rotate_to_answer":true,"replay_incorrect":true,"ask_messier":false,"ask_caldwell":false,"ask_ic":false,"ask_ngc":false,"ask_hd":false,"ask_hip":false,"ask_bayer":true,"ask_flamsteed":false,"ask_proper":true}): AND(OR(CATALOGUE(PROPER_NAME), CATALOGUE(BAYER)), OR(AND(TYPE(STAR), MAG_BELOW(2.21)), CATALOGUE_DESIGNATION(PROPER_NAME:Sadr, PROPER_NAME:Albireo, PROPER_NAME:Sheliak, PROPER_NAME:Sulafat, PROPER_NAME:Vindemiatrix, PROPER_NAME:Porrima, PROPER_NAME:Alcor, PROPER_NAME:Mizar, PROPER_NAME:Izar, HIP:79593, HIP:79882, PROPER_NAME:Sabik, PROPER_NAME:Dschubba, PROPER_NAME:Acrab, PROPER_NAME:Ankaa, PROPER_NAME:Menkar, PROPER_NAME:Thuban, PROPER_NAME:Phecda, PROPER_NAME:Merak, PROPER_NAME:Megrez, PROPER_NAME:Caph, HIP:3179, PROPER_NAME:Ruchbah, PROPER_NAME:Segin, PROPER_NAME:Mintaka, HIP:89931, HIP:90496, HIP:88635, HIP:92041, HIP:93506, HIP:105199, HIP:87833, HIP:85670, HIP:75097, HIP:110130, HIP:63125)))"#,
                    "\n",
                    r#""#,
                    "\n",
                    r#"WHAT_IS_THIS_OBJECT({"rotate_to_point":true,"replay_incorrect":true,"accept_messier":false,"accept_caldwell":false,"accept_ngc":false,"accept_ic":false,"accept_hip":true,"accept_hd":true,"accept_proper":true,"accept_bayer":false,"accept_flamsteed":false}): AND(CATALOGUE(PROPER_NAME), OR(AND(TYPE(STAR), MAG_BELOW(2.21)), CATALOGUE_DESIGNATION(PROPER_NAME:Sadr, PROPER_NAME:Albireo, PROPER_NAME:Sheliak, PROPER_NAME:Sulafat, PROPER_NAME:Vindemiatrix, PROPER_NAME:Porrima, PROPER_NAME:Alcor, PROPER_NAME:Mizar, PROPER_NAME:Izar, HIP:79593, HIP:79882, PROPER_NAME:Sabik, PROPER_NAME:Dschubba, PROPER_NAME:Acrab, PROPER_NAME:Ankaa, PROPER_NAME:Menkar, PROPER_NAME:Thuban, PROPER_NAME:Phecda, PROPER_NAME:Merak, PROPER_NAME:Megrez, PROPER_NAME:Caph, HIP:3179, PROPER_NAME:Ruchbah, PROPER_NAME:Segin, PROPER_NAME:Mintaka, HIP:89931, HIP:90496, HIP:88635, HIP:92041, HIP:93506, HIP:105199, HIP:87833, HIP:85670, HIP:75097, HIP:110130, HIP:63125)))"#,
                    "\n",
                    r#""#,
                    "\n",
                    r#"WHAT_IS_THIS_OBJECT({"rotate_to_point":true,"replay_incorrect":true,"accept_messier":false,"accept_caldwell":false,"accept_ngc":false,"accept_ic":false,"accept_hip":true,"accept_hd":true,"accept_proper":false,"accept_bayer":true,"accept_flamsteed":false}): AND(CATALOGUE(BAYER), OR(AND(TYPE(STAR), MAG_BELOW(2.21)), CATALOGUE_DESIGNATION(PROPER_NAME:Sadr, PROPER_NAME:Albireo, PROPER_NAME:Sheliak, PROPER_NAME:Sulafat, PROPER_NAME:Vindemiatrix, PROPER_NAME:Porrima, PROPER_NAME:Alcor, PROPER_NAME:Mizar, PROPER_NAME:Izar, HIP:79593, HIP:79882, PROPER_NAME:Sabik, PROPER_NAME:Dschubba, PROPER_NAME:Acrab, PROPER_NAME:Ankaa, PROPER_NAME:Menkar, PROPER_NAME:Thuban, PROPER_NAME:Phecda, PROPER_NAME:Merak, PROPER_NAME:Megrez, PROPER_NAME:Caph, HIP:3179, PROPER_NAME:Ruchbah, PROPER_NAME:Segin, PROPER_NAME:Mintaka, HIP:89931, HIP:90496, HIP:88635, HIP:92041, HIP:93506, HIP:105199, HIP:87833, HIP:85670, HIP:75097, HIP:110130, HIP:63125)))"#
                )),
                question_objects: vec![
                    (
                        QuestionType::FindThisObject(find_this_object::SmallSettings {
                            correctness_threshold: angle::Deg(1.0),
                            rotate_to_answer: true,
                            replay_incorrect: true,
                            ask_messier: false,
                            ask_caldwell: false,
                            ask_ic: false,
                            ask_ngc: false,
                            ask_hd: false,
                            ask_hip: false,
                            ask_bayer: true,
                            ask_flamsteed: false,
                            ask_proper: true,
                        }),
                        vec![
                            896, 963, 2295, 3391, 3632, 4636, 5655, 6891, 7793, 9087, 9838, 10080, 11953, 14319, 14759, 16044, 21587, 24597, 24768, 25494, 25585, 26086, 26465, 26881, 27517, 28138,
                            28507, 30470, 30584, 31821, 32484, 33712, 34573, 36964, 37392, 37938, 40059, 41140, 43013, 45325, 46479, 49748, 50660, 53976, 54127, 57680, 58049, 59813, 60753, 61116,
                            61960, 61969, 62460, 62979, 63148, 63627, 65394, 65490, 65493, 67309, 68704, 68758, 68935, 69672, 71675, 71678, 72100, 72601, 75089, 78386, 78801, 79573, 79862, 80740,
                            82243, 83976, 85631, 85886, 85990, 86186, 87782, 88582, 89874, 90127, 90438, 91200, 91976, 92355, 92785, 93123, 93434, 95869, 97559, 100349, 100646, 101988, 105082,
                            109144, 110005, 111990, 113229,
                        ],
                    ),
                    (
                        QuestionType::WhatIsThisObject(which_object_is_here::SmallSettings {
                            rotate_to_point: true,
                            replay_incorrect: true,
                            accept_messier: false,
                            accept_caldwell: false,
                            accept_ngc: false,
                            accept_ic: false,
                            accept_hip: true,
                            accept_hd: true,
                            accept_proper: true,
                            accept_bayer: false,
                            accept_flamsteed: false,
                        }),
                        vec![
                            896, 963, 2295, 3391, 3632, 5655, 6891, 7793, 9087, 9838, 10080, 11953, 14319, 14759, 16044, 21587, 24597, 24768, 25494, 25585, 26086, 26465, 26881, 27517, 28138, 28507,
                            30470, 30584, 31821, 32484, 33712, 34573, 36964, 37392, 37938, 40059, 41140, 43013, 45325, 46479, 49748, 50660, 53976, 54127, 57680, 58049, 59813, 60753, 61116, 61969,
                            62460, 62979, 63148, 63627, 65394, 65490, 65493, 67309, 68704, 68758, 68935, 69672, 71675, 71678, 72100, 72601, 75089, 78386, 78801, 79573, 79862, 80740, 82243, 83976,
                            85631, 85886, 85990, 86186, 87782, 88582, 89874, 90127, 90438, 91200, 92355, 92785, 93123, 93434, 95869, 97559, 100349, 100646, 101988, 105082, 109144, 111990, 113229,
                        ],
                    ),
                    (
                        QuestionType::WhatIsThisObject(which_object_is_here::SmallSettings {
                            rotate_to_point: true,
                            replay_incorrect: true,
                            accept_messier: false,
                            accept_caldwell: false,
                            accept_ngc: false,
                            accept_ic: false,
                            accept_hip: true,
                            accept_hd: true,
                            accept_proper: false,
                            accept_bayer: true,
                            accept_flamsteed: false,
                        }),
                        vec![
                            896, 963, 2295, 3391, 3632, 4636, 5655, 6891, 7793, 9087, 9838, 10080, 11953, 14319, 14759, 16044, 21587, 24597, 24768, 25494, 25585, 26086, 26465, 26881, 27517, 28138,
                            28507, 30470, 30584, 31821, 32484, 33712, 34573, 36964, 37392, 37938, 40059, 41140, 43013, 45325, 46479, 49748, 50660, 53976, 54127, 57680, 58049, 59813, 60753, 61116,
                            61960, 61969, 62460, 62979, 63148, 63627, 65394, 65490, 65493, 67309, 68704, 68758, 68935, 69672, 71675, 71678, 72100, 72601, 75089, 78386, 78801, 79573, 79862, 80740,
                            82243, 83976, 85631, 85886, 85990, 86186, 87782, 88582, 89874, 90127, 90438, 91200, 91976, 92355, 92785, 93123, 93434, 95869, 97559, 100349, 100646, 101988, 105082,
                            109144, 110005, 111990, 113229,
                        ],
                    ),
                ],
                description: String::from(r#"Tests the knowledge of proper name and Bayer designations of stars of up to magnitude 2.2 and some other ones deemed "important". Should be enough for the IOAA."#),
                file_path: None,
            },
        ),
    ]
}

use std::str::Chars;

#[derive(Debug)]
enum KeywordRaw {
    And,
    Or,
    Not,
    Dec,
    RaDeg,
    Ra,
    Constellation,
    ConstellationGroup, // Transforms into CONSTELLATION(list of constellations in the group)
    Catalogue,
    Type,
    MagBelow,
    MagAbove,
    Mag,
    ObjectId,
    CatalogueDesignation,
}

#[derive(Debug)]
pub enum Keyword {
    And(Vec<Box<Keyword>>),
    Or(Vec<Box<Keyword>>),
    Not(Box<Keyword>),
    Dec(f32, f32),
    RaDeg(f32, f32),
    Ra(f32, f32),
    Constellation(Vec<String>),
    Catalogue(Vec<Catalogue>),
    Type(Vec<crate::game::ObjectType>),
    MagBelow(f32),
    MagAbove(f32),
    Mag(f32, f32),
    ObjectId(u64),
    CatalogueDesignation(Vec<(Catalogue, String)>),
}

impl Keyword {
    fn from_raw(keyword_raw: KeywordRaw, args: Vec<Node>, ident_pos: usize, constellation_groups: &std::collections::HashMap<String, std::collections::HashMap<String, bool>>) -> Result<Self, String> {
        let keyword = match keyword_raw {
            KeywordRaw::And => {
                let mut new_args = Vec::new();
                for arg in args {
                    match arg {
                        Node::Keyword(keyword) => new_args.push(Box::new(keyword)),
                        Node::Value(_) => return Err(format!("Keyword 'AND' can only take other keywords, not values (position {})", ident_pos)),
                    }
                }
                if new_args.is_empty() {
                    return Err(format!("Keyword 'AND' at position {} expects at least 1 argument, found 0", ident_pos));
                }
                Self::And(new_args)
            }
            KeywordRaw::Or => {
                let mut new_args = Vec::new();
                for arg in args {
                    match arg {
                        Node::Keyword(keyword) => new_args.push(Box::new(keyword)),
                        Node::Value(_) => return Err(format!("Keyword 'OR' can only take other keywords, not values (position {})", ident_pos)),
                    }
                }
                if new_args.is_empty() {
                    return Err(format!("Keyword 'OR' at position {} expects at least 1 argument, found 0", ident_pos));
                }
                Self::Or(new_args)
            }
            KeywordRaw::Not => {
                let mut new_args = Vec::new();
                for arg in args {
                    match arg {
                        Node::Keyword(keyword) => new_args.push(Box::new(keyword)),
                        Node::Value(_) => return Err(format!("Keyword 'NOT' can only take other keywords, not values (position {})", ident_pos)),
                    }
                }
                if new_args.len() != 1 {
                    return Err(format!("Keyword 'NOT' at position {} expects exactly 1 argument, found {}", ident_pos, new_args.len()));
                }
                Self::Not(new_args.remove(0))
            }
            KeywordRaw::Dec => {
                let mut new_args = Vec::new();
                for arg in args {
                    match arg {
                        Node::Keyword(_) => return Err(format!("Keyword 'DEC' can only take values, not other keywords (position {})", ident_pos)),
                        Node::Value(value) => new_args.push(value),
                    }
                }
                if new_args.len() != 2 {
                    return Err(format!("Keyword 'DEC' at position {} expects exactly 2 arguments, found {}", ident_pos, new_args.len()));
                }
                let (min, max) = (new_args.remove(0), new_args.remove(0));
                let mut min = match min.parse() {
                    Ok(min) => min,
                    Err(err) => return Err(format!("Keyword 'DEC' at position {} expects numbers as arguments, found '{}' ('{}')", ident_pos, min, err)),
                };
                let mut max = match max.parse() {
                    Ok(max) => max,
                    Err(err) => return Err(format!("Keyword 'DEC' at position {} expects numbers as arguments, found '{}' ('{}')", ident_pos, max, err)),
                };
                if min > max {
                    std::mem::swap(&mut min, &mut max);
                }
                Self::Dec(min, max)
            }
            KeywordRaw::RaDeg => {
                let mut new_args = Vec::new();
                for arg in args {
                    match arg {
                        Node::Keyword(_) => return Err(format!("Keyword 'RA_DEG' can only take values, not other keywords (position {})", ident_pos)),
                        Node::Value(value) => new_args.push(value),
                    }
                }
                if new_args.len() != 2 {
                    return Err(format!("Keyword 'RA_DEG' at position {} expects exactly 2 arguments, found {}", ident_pos, new_args.len()));
                }
                let (min, max) = (new_args[0].clone(), new_args[1].clone());
                let mut min = match min.parse() {
                    Ok(min) => min,
                    Err(err) => return Err(format!("Keyword 'RA_DEG' at position {} expects numbers as arguments, found '{}' ('{}')", ident_pos, min, err)),
                };
                let mut max = match max.parse() {
                    Ok(max) => max,
                    Err(err) => return Err(format!("Keyword 'RA_DEG' at position {} expects numbers as arguments, found '{}' ('{}')", ident_pos, max, err)),
                };
                if min > max {
                    std::mem::swap(&mut min, &mut max);
                }
                Self::RaDeg(min, max)
            }
            KeywordRaw::Ra => {
                let mut new_args = Vec::new();
                for arg in args {
                    match arg {
                        Node::Keyword(_) => return Err(format!("Keyword 'RA' can only take values, not other keywords (position {})", ident_pos)),
                        Node::Value(value) => new_args.push(value),
                    }
                }
                if new_args.len() != 2 {
                    return Err(format!("Keyword 'RA' at position {} expects exactly 2 arguments, found {}", ident_pos, new_args.len()));
                }
                let (min, max) = (new_args[0].clone(), new_args[1].clone());
                let mut min = match min.parse() {
                    Ok(min) => min,
                    Err(err) => return Err(format!("Keyword 'RA' at position {} expects numbers as arguments, found '{}' ('{}')", ident_pos, min, err)),
                };
                let mut max = match max.parse() {
                    Ok(max) => max,
                    Err(err) => return Err(format!("Keyword 'RA' at position {} expects numbers as arguments, found '{}' ('{}')", ident_pos, max, err)),
                };
                if min > max {
                    std::mem::swap(&mut min, &mut max);
                }
                Self::Ra(min, max)
            }
            KeywordRaw::Constellation => {
                // TODO: Check for valid constellations
                let mut new_args = Vec::new();
                for arg in args {
                    match arg {
                        Node::Keyword(_) => return Err(format!("Keyword 'CONSTELLATION' can only take values, not other keywords (position {})", ident_pos)),
                        Node::Value(value) => new_args.push(value),
                    }
                }
                if new_args.is_empty() {
                    return Err(format!("Keyword 'CONSTELLATION' at position {} expects at least 1 argument, found 0", ident_pos));
                }
                Self::Constellation(new_args)
            }
            KeywordRaw::ConstellationGroup => {
                let mut group_names = Vec::new();
                for arg in args {
                    match arg {
                        Node::Keyword(_) => return Err(format!("Keyword 'CONSTELLATION_GROUP' can only take values, not other keywords (position {})", ident_pos)),
                        Node::Value(group_name) => {
                            group_names.push(group_name);
                        }
                    }
                }
                if group_names.is_empty() {
                    return Err(format!("Keyword 'CONSTELLATION_GROUP' at position {} expects at least 1 argument, found 0", ident_pos));
                }
                let mut constellations = std::collections::HashSet::new();
                for group_name in group_names {
                    if let Some(group) = constellation_groups.get(&group_name) {
                        for (name, enabled) in group.iter() {
                            if *enabled {
                                constellations.insert(name.to_uppercase());
                            }
                        }
                    } else {
                        return Err(format!("Could not find constellation group named '{}'", group_name));
                    }
                }
                let mut constellations = constellations.into_iter().collect::<Vec<String>>();
                constellations.sort();
                Self::Constellation(constellations)
            }
            KeywordRaw::Catalogue => {
                let mut new_args = Vec::new();
                for arg in args {
                    match arg {
                        Node::Keyword(_) => return Err(format!("Keyword 'CATALOGUE' can only take values, not other keywords (position {})", ident_pos)),
                        Node::Value(value) => {
                            let catalogue = Catalogue::from_string(&value)?;
                            new_args.push(catalogue);
                        }
                    }
                }
                if new_args.is_empty() {
                    return Err(format!("Keyword 'CATALOGUE' at position {} expects at least 1 argument, found 0", ident_pos));
                }
                Self::Catalogue(new_args)
            }
            KeywordRaw::CatalogueDesignation => {
                let mut new_args_raw = Vec::new();
                for arg in args {
                    match arg {
                        Node::Keyword(_) => return Err(format!("Keyword 'CATALOGUE_DESIGNATION' can only take values, not other keywords (position {})", ident_pos)),
                        Node::Value(value) => {
                            new_args_raw.push(value);
                        }
                    }
                }
                if new_args_raw.is_empty() {
                    return Err(format!("Keyword 'CATALOGUE_DESIGNATION' at position {} expects at least 1 argument, found 0", ident_pos));
                }
                let mut new_args = Vec::with_capacity(new_args_raw.len());
                for s in new_args_raw {
                    let spl = s.split(":").collect::<Vec<&str>>();
                    if spl.len() != 2 {
                        return Err(format!(
                            "Each argument of 'CATALOGUE_DESIGNATION' has to be in the format <catalogue>:<catalogue number>, did not get the correct amount of colons at position {} ",
                            ident_pos
                        ));
                    };
                    let catalogue = Catalogue::from_string(&spl[0])?;
                    match catalogue {
                        Catalogue::Caldwell | Catalogue::Hd | Catalogue::Hip | Catalogue::Messier | Catalogue::Ngc => {
                            if let Err(err) = spl[1].parse::<u32>() {
                                return Err(format!(
                                    "The catalogue number has to be a whole number, found {} ({}) as an argument of 'CATALOGUE_DESIGNATION' at position {}",
                                    spl[1], err, ident_pos
                                ));
                            }
                        }
                        Catalogue::Bayer | Catalogue::Flamsteed | Catalogue::ProperName => {}
                    };
                    new_args.push((catalogue, spl[1].to_owned()));
                }
                Self::CatalogueDesignation(new_args)
            }
            KeywordRaw::Type => {
                let mut new_args = Vec::new();
                for arg in args {
                    match arg {
                        Node::Keyword(_) => return Err(format!("Keyword 'TYPE' can only take values, not other keywords (position {})", ident_pos)),
                        Node::Value(value) => {
                            let object_type = crate::game::ObjectType::from_string(&value)?;
                            new_args.push(object_type);
                        }
                    }
                }
                if new_args.is_empty() {
                    return Err(format!("Keyword 'TYPE' at position {} expects at least 1 argument, found 0", ident_pos));
                }
                Self::Type(new_args)
            }
            KeywordRaw::MagBelow => {
                let mut new_args = Vec::new();
                for arg in args {
                    match arg {
                        Node::Keyword(_) => return Err(format!("Keyword 'MAG_BELOW' can only take values, not other keywords (position {})", ident_pos)),
                        Node::Value(value) => new_args.push(value),
                    }
                }
                if new_args.len() != 1 {
                    return Err(format!("Keyword 'MAG_BELOW' at position {} expects exactly 1 argument, found {}", ident_pos, new_args.len()));
                }
                let val = new_args[0].clone();
                let val = match val.parse() {
                    Ok(val) => val,
                    Err(err) => return Err(format!("Keyword 'MAG_BELOW' at position {} expects a numbers as argument, found '{}' ('{}')", ident_pos, val, err)),
                };
                Self::MagBelow(val)
            }
            KeywordRaw::MagAbove => {
                let mut new_args = Vec::new();
                for arg in args {
                    match arg {
                        Node::Keyword(_) => return Err(format!("Keyword 'MAG_ABOVE' can only take values, not other keywords (position {})", ident_pos)),
                        Node::Value(value) => new_args.push(value),
                    }
                }
                if new_args.len() != 1 {
                    return Err(format!("Keyword 'MAG_ABOVE' at position {} expects exactly 1 argument, found {}", ident_pos, new_args.len()));
                }
                let val = new_args[0].clone();
                let val = match val.parse() {
                    Ok(val) => val,
                    Err(err) => return Err(format!("Keyword 'MAG_ABOVE' at position {} expects a numbers as argument, found '{}' ('{}')", ident_pos, val, err)),
                };
                Self::MagAbove(val)
            }
            KeywordRaw::Mag => {
                let mut new_args = Vec::new();
                for arg in args {
                    match arg {
                        Node::Keyword(_) => return Err(format!("Keyword 'MAG' can only take values, not other keywords (position {})", ident_pos)),
                        Node::Value(value) => new_args.push(value),
                    }
                }
                if new_args.len() != 2 {
                    return Err(format!("Keyword 'MAG' at position {} expects exactly 2 arguments, found {}", ident_pos, new_args.len()));
                }
                let (min, max) = (new_args[0].clone(), new_args[1].clone());
                let mut min = match min.parse() {
                    Ok(min) => min,
                    Err(err) => return Err(format!("Keyword 'MAG' at position {} expects numbers as arguments, found '{}' ('{}')", ident_pos, min, err)),
                };
                let mut max = match max.parse() {
                    Ok(max) => max,
                    Err(err) => return Err(format!("Keyword 'MAG' at position {} expects numbers as arguments, found '{}' ('{}')", ident_pos, max, err)),
                };
                if min > max {
                    std::mem::swap(&mut min, &mut max);
                }
                Self::Mag(min, max)
            }
            KeywordRaw::ObjectId => {
                let mut new_args = Vec::new();
                for arg in args {
                    match arg {
                        Node::Keyword(_) => return Err(format!("Keyword 'OBJECT_ID' can only take values, not other keywords (position {})", ident_pos)),
                        Node::Value(value) => new_args.push(value),
                    }
                }
                if new_args.len() != 1 {
                    return Err(format!("Keyword 'OBJECT_ID' at position {} expects exactly 1 argument, found {}", ident_pos, new_args.len()));
                }
                let val = new_args[0].clone();
                let val = match val.parse() {
                    Ok(val) => val,
                    Err(err) => {
                        return Err(format!(
                            "Keyword 'OBJECT_ID' at position {} expects a whole numbers as argument, found '{}' ('{}')",
                            ident_pos, val, err
                        ))
                    }
                };
                Self::ObjectId(val)
            }
        };
        Ok(keyword)
    }
}

#[derive(Debug)]
pub enum Node {
    Keyword(Keyword),
    Value(String),
}

pub struct Parser<'a> {
    chars: Chars<'a>,
    pos: usize,
}

const VALID_CATALOGUES: [&str; 8] = ["BAYER", "FLAMSTEED", "MESSIER", "CALDWELL", "NGC", "HD", "HIP", "PROPER_NAME"];
#[derive(Debug)]
pub enum Catalogue {
    Bayer,
    Flamsteed,
    Messier,
    Caldwell,
    Ngc,
    Hd,
    Hip,
    ProperName,
}

impl Catalogue {
    pub fn from_string(catalogue: &str) -> Result<Self, String> {
        let val = catalogue.to_uppercase();
        let catalogue = match val.as_str() {
            "BAYER" => Self::Bayer,
            "FLAMSTEED" => Self::Flamsteed,
            "MESSIER" => Self::Messier,
            "CALDWELL" => Self::Caldwell,
            "NGC" => Self::Ngc,
            "HD" => Self::Hd,
            "HIP" => Self::Hip,
            "PROPER_NAME" => Self::ProperName,
            _ => return Err(format!("Invalid catalogue, the only accepted ones are: {}", VALID_CATALOGUES.join(", "))),
        };
        Ok(catalogue)
    }
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { chars: input.chars(), pos: 0 }
    }

    pub fn parse(&mut self, constellation_groups: &std::collections::HashMap<String, std::collections::HashMap<String, bool>>) -> Result<Option<Node>, String> {
        self.parse_expression(constellation_groups)
    }

    fn parse_expression(&mut self, constellation_groups: &std::collections::HashMap<String, std::collections::HashMap<String, bool>>) -> Result<Option<Node>, String> {
        self.consume_whitespace();

        if let Some(ident) = self.parse_identifier() {
            self.consume_whitespace();

            if self.peek() == Some('(') {
                let ident_pos = self.pos - ident.len();
                let keyword_raw = match ident.as_str() {
                    "AND" => KeywordRaw::And,
                    "OR" => KeywordRaw::Or,
                    "NOT" => KeywordRaw::Not,
                    "DEC" => KeywordRaw::Dec,
                    "RA_DEG" => KeywordRaw::RaDeg,
                    "RA" => KeywordRaw::Ra,
                    "CONSTELLATION" => KeywordRaw::Constellation,
                    "CONSTELLATION_GROUP" => KeywordRaw::ConstellationGroup,
                    "CATALOGUE" => KeywordRaw::Catalogue,
                    "TYPE" => KeywordRaw::Type,
                    "MAG_BELOW" => KeywordRaw::MagBelow,
                    "MAG_ABOVE" => KeywordRaw::MagAbove,
                    "MAG" => KeywordRaw::Mag,
                    "OBJECT_ID" => KeywordRaw::ObjectId,
                    "CATALOGUE_DESIGNATION" => KeywordRaw::CatalogueDesignation,
                    _ => return Err(format!("Unknown keyword '{}' at position {}", ident, ident_pos)),
                };
                self.chars.next(); // Consume '('
                self.pos += 1;

                let args = self.parse_arguments(constellation_groups)?;

                let keyword = Keyword::from_raw(keyword_raw, args, ident_pos, constellation_groups)?;

                return Ok(Some(Node::Keyword(keyword)));
            } else {
                return Ok(Some(Node::Value(ident)));
            }
        }
        Ok(None)
    }

    fn parse_arguments(&mut self, constellation_groups: &std::collections::HashMap<String, std::collections::HashMap<String, bool>>) -> Result<Vec<Node>, String> {
        let mut args = Vec::new();

        while let Some(node) = self.parse_expression(constellation_groups)? {
            args.push(node);
            self.consume_whitespace();

            match self.peek() {
                Some(',') => {
                    self.chars.next();
                    self.pos += 1;
                } // Consume ','
                Some(')') => {
                    self.chars.next();
                    self.pos += 1;
                    break;
                } // Consume ')'
                _ => break,
            }
        }
        Ok(args)
    }

    fn parse_identifier(&mut self) -> Option<String> {
        let mut ident = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' || c == '.' || c == ':' {
                ident.push(self.chars.next().unwrap());
                self.pos += 1;
            } else {
                break;
            }
        }
        if !ident.is_empty() {
            Some(ident)
        } else {
            None
        }
    }

    fn consume_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.chars.next();
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.clone().next()
    }
}

/*fn main() {
    let input = "OR(AND(CONSTELLATION(AND, VIR, LEP, ORI), DEC(17, 98), NOT(RA_DEG(170, 321))), RA(0, 12.5), CATALOGUE(Messier, Caldwell, HIP, PROPER_NAME))";
    let mut parser = Parser::new(input);
    match parser.parse() {
        Ok(Some(Node::Keyword(ast))) => println!("{:?}", ast),
        Ok(Some(Node::Value(ast))) => println!("Value: {:?}", ast),
        Ok(None) => println!("none..."),
        Err(e) => println!("Error: {}", e),
    }
}*/

pub fn parse_question_type_and_settings(question_type: &str, question_settings: &str) -> Result<crate::game::questions::QuestionType, String> {
    match question_type.to_uppercase().as_str() {
        "ANGULAR_SEPARATION" => match serde_json::from_str(question_settings) {
            Ok(question_settings) => Ok(crate::game::questions::QuestionType::AngularSeparation(question_settings)),
            Err(err) => Err(format!("Error when parsing question settings ({err})")),
        },
        "FIND_THIS_OBJECT" => match serde_json::from_str(question_settings) {
            Ok(question_settings) => Ok(crate::game::questions::QuestionType::FindThisObject(question_settings)),
            Err(err) => Err(format!("Error when parsing question settings ({err})")),
        },
        "GUESS_DEC" => match serde_json::from_str(question_settings) {
            Ok(question_settings) => Ok(crate::game::questions::QuestionType::GuessDec(question_settings)),
            Err(err) => Err(format!("Error when parsing question settings ({err})")),
        },
        "GUESS_RA" => match serde_json::from_str(question_settings) {
            Ok(question_settings) => Ok(crate::game::questions::QuestionType::GuessRa(question_settings)),
            Err(err) => Err(format!("Error when parsing question settings ({err})")),
        },
        "GUESS_THE_MAGNITUDE" => match serde_json::from_str(question_settings) {
            Ok(question_settings) => Ok(crate::game::questions::QuestionType::GuessTheMagnitude(question_settings)),
            Err(err) => Err(format!("Error when parsing question settings ({err})")),
        },
        "WHAT_IS_THIS_OBJECT" => match serde_json::from_str(question_settings) {
            Ok(question_settings) => Ok(crate::game::questions::QuestionType::WhatIsThisObject(question_settings)),
            Err(err) => Err(format!("Error when parsing question settings ({err})")),
        },
        "WHICH_CONSTELLATION_IS_THIS_POINT_IN" => match serde_json::from_str(question_settings) {
            Ok(question_settings) => Ok(crate::game::questions::QuestionType::WhichConstellationIsThisPointIn(question_settings)),
            Err(err) => Err(format!("Error when parsing question settings ({err})")),
        },
        _ => Err(String::from("Error when parsing the query, the question type could not be matched")),
    }
}

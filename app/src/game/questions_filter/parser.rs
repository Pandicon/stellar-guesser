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
    Catalogue,
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
}

impl Keyword {
    pub fn from_raw(keyword_raw: KeywordRaw, args: Vec<Node>, ident_pos: usize) -> Result<Self, String> {
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
                        Node::Keyword(_) => return Err(format!("Keyword 'DEC' can only take values, not other keywords (position {})", ident_pos)),
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
                        Node::Keyword(_) => return Err(format!("Keyword 'DEC' can only take values, not other keywords (position {})", ident_pos)),
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
        };
        Ok(keyword)
    }
}

#[derive(Debug)]
enum Node {
    Keyword(Keyword),
    Value(String),
}

struct Parser<'a> {
    chars: Chars<'a>,
    pos: usize,
}

const VALID_CATALOGUES: [&str; 6] = ["MESSIER", "CALDWELL", "NGC", "HD", "HIP", "PROPER_NAME"];
#[derive(Debug)]
pub enum Catalogue {
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
    fn new(input: &'a str) -> Self {
        Self { chars: input.chars(), pos: 0 }
    }

    fn parse(&mut self) -> Result<Option<Node>, String> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<Option<Node>, String> {
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
                    "CATALOGUE" => KeywordRaw::Catalogue,
                    _ => return Err(format!("Unknown keyword '{}' at position {}", ident, ident_pos)),
                };
                self.chars.next(); // Consume '('
                self.pos += 1;

                let args = self.parse_arguments()?;

                let keyword = Keyword::from_raw(keyword_raw, args, ident_pos)?;

                return Ok(Some(Node::Keyword(keyword)));
            } else {
                return Ok(Some(Node::Value(ident)));
            }
        }
        Ok(None)
    }

    fn parse_arguments(&mut self) -> Result<Vec<Node>, String> {
        let mut args = Vec::new();

        while let Some(node) = self.parse_expression()? {
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
            if c.is_alphanumeric() || c == '_' || c == '.' {
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

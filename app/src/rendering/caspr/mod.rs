pub mod constellation;
pub mod deepsky;
pub mod lines;
pub mod markers;
pub mod renderer;
pub mod sky_settings;
pub mod star_names;
pub mod stars;

pub enum SpecificName {
    /// Gives back all name combinations
    None,
    /// Gives back only the name without any optional parts
    NoOptional,
    /// Gives back only the name with all optional parts
    AllOptional,
}

/// Takes the name containing optional parts inside {} and turns it into all possible variants of that name.
/// For example "1{ 2}{ 3}" -> ["1", "1 2", "1 3", "1 2 3"]
/// The parts are always included in this order, so the name without any optional part is the first one, and the one with all optional parts is the last one.
pub fn generate_name_combinations(s: &str, specific_name: SpecificName) -> Vec<String> {
    use regex::Regex;

    // Finds text inside { } and captures it.
    let re = Regex::new(r"\{([^}]+)\}").unwrap();

    // Holds the fixed parts (between the {} bits)
    let mut segments = Vec::new();
    // Holds the inner text of the optional parts (inside {})
    let mut options = Vec::new();
    let mut last_index = 0;

    // Iterate over each match (the entire "{...}" and the captured text).
    for cap in re.captures_iter(s) {
        let m = cap.get(0).unwrap();
        segments.push(&s[last_index..m.start()]);
        // Capture the text inside the braces.
        if let Some(text) = cap.get(1) {
            options.push(text.as_str());
        } else {
            options.push("");
        }
        last_index = m.end();
    }
    segments.push(&s[last_index..]); // trailing fixed part

    let n = options.len();
    let mut results = Vec::new();
    let range = match specific_name {
        SpecificName::None => 0..(1 << n), // There are 2^n combinations.
        SpecificName::NoOptional => 0..1,
        SpecificName::AllOptional => (1 << n) - 1..(1 << n), // Use only the one will all 1s
    };
    for mask in range {
        let mut candidate = String::new();
        // Iterate over each segment and, if available, decide whether to include the corresponding option.
        // Since each option comes with a corresponding segment in front of it, alternating segments and options reconstructs the string in the correct order.
        for i in 0..n {
            candidate.push_str(segments[i]);
            if (mask & (1 << i)) != 0 {
                candidate.push_str(options[i]);
            }
        }
        candidate.push_str(segments[n]);
        // Trim any extra whitespace and add to results.
        results.push(candidate.trim().to_string());
    }
    results
}

#[cfg(test)]
mod tests {
    #[test]
    fn name_combinations() {
        let input = "1{ 2}{ 3}";
        let combinations = super::generate_name_combinations(input, super::SpecificName::None);
        assert_eq!(combinations, vec![String::from("1"), String::from("1 2"), String::from("1 3"), String::from("1 2 3")]);

        let input = "1{ 2}{ 3}";
        let combinations = super::generate_name_combinations(input, super::SpecificName::NoOptional);
        assert_eq!(combinations, vec![String::from("1")]);

        let input = "1{ 2}{ 3}";
        let combinations = super::generate_name_combinations(input, super::SpecificName::AllOptional);
        assert_eq!(combinations, vec![String::from("1 2 3")]);

        let input = "{Optional starting text }Some text{ inner text}{ more} ending text";
        let combinations = super::generate_name_combinations(input, super::SpecificName::None);
        assert_eq!(
            combinations,
            vec![
                String::from("Some text ending text"),
                String::from("Optional starting text Some text ending text"),
                String::from("Some text inner text ending text"),
                String::from("Optional starting text Some text inner text ending text"),
                String::from("Some text more ending text"),
                String::from("Optional starting text Some text more ending text"),
                String::from("Some text inner text more ending text"),
                String::from("Optional starting text Some text inner text more ending text"),
            ]
        );

        let input = "{Optional starting text }Some text{ inner text}{ more} ending text";
        let combinations = super::generate_name_combinations(input, super::SpecificName::NoOptional);
        assert_eq!(combinations, vec![String::from("Some text ending text"),]);

        let input = "{Optional starting text }Some text{ inner text}{ more} ending text";
        let combinations = super::generate_name_combinations(input, super::SpecificName::AllOptional);
        assert_eq!(combinations, vec![String::from("Optional starting text Some text inner text more ending text"),]);
    }
}

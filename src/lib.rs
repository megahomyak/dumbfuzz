pub type IsMatching = bool; 

pub type Difference = Vec<(char, IsMatching)>;

pub fn compare(source: &str, pattern: &str) -> Difference {
    let mut pattern = pattern.chars();
    let mut pattern_char = pattern.next();
    let mut processed = Vec::new();
    for source_char in source.chars() {
        processed.push((source_char, if pattern_char == Some(source_char) {
            pattern_char = pattern.next();
            true
        } else {
            false
        }));
    }
    processed
}

pub fn best(sources: Vec<&str>, pattern: &str) -> Vec<Difference> {
    let mut matched = Vec::new();
    for source in sources {
        let difference = compare(source, pattern);
        let count = difference.iter().filter(|(_char, is_match)| *is_match).count();
        if count > 0 {
            matched.push((count, difference));
        }
    }
    matched.sort_by_key(|(count, _comparison)| std::cmp::Reverse(*count));
    matched.into_iter().map(|(_count, difference)| difference).collect()
}

pub type IsMatch = bool;

pub struct MatchChars<'a> {
    matches: &'a Matches<'a>,
    indexes_iterator: std::slice::Iter<'a, usize>,
}

unsafe fn index(s: &str, index: usize) -> char {
    unsafe { *((s.as_ptr().add(index)) as *const char) }
}

impl<'a> Iterator for MatchChars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.indexes_iterator
            .next()
            .map(|index_| unsafe { index(self.matches.source, *index_) })
    }
}

pub struct Matches<'a> {
    source: &'a str,
    match_indexes: Vec<usize>,
}

pub struct SourceChars<'a> {
    matches: &'a Matches<'a>,
    indexes_iterator: std::slice::Iter<'a, usize>,
    source_iterator: std::str::CharIndices<'a>,
    last_index: Option<usize>,
}

impl<'a> Iterator for SourceChars<'a> {
    type Item = (char, IsMatch);

    fn next(&mut self) -> Option<Self::Item> {
        self.source_iterator.next().map(|source_char| {
            if self.last_index == sour
        })
    }
}

impl<'a> Matches<'a> {
    pub fn len(&self) -> usize {
        self.match_indexes.len()
    }

    pub fn chars(&self) -> MatchChars {
        MatchChars {
            matches: self,
            indexes_iterator: self.match_indexes.iter(),
        }
    }

    pub fn source_chars(&self) -> SourceChars {
        let mut indexes_iterator = self.match_indexes.iter();
        SourceChars {
            last_index: indexes_iterator.next().copied(),
            indexes_iterator,
            matches: self,
            source_iterator: self.source.char_indices(),
        }
    }
}

pub fn compare(source: &str, pattern: &str) -> Matches {
    let matches_amount = 0;
    let mut matches = Vec::new();
    let mut pattern = pattern.chars();
    let mut last_pattern_char = pattern.next();
    for source_char in source.chars() {}
    (matches_amount, matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub type IsMatch = bool;

unsafe fn index(s: &str, index: usize) -> char {
    unsafe { *((s.as_ptr().add(index)) as *const char) }
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

pub fn compare<'a, 'b>(source: &'a str, pattern: &'b str) -> Matches<'a> {
    let mut pattern = pattern.chars();
    let mut pattern_char = pattern.next();
    let mut match_indexes = Vec::new();
    for (index, source_char) in source.char_indices() {
        if pattern_char == Some(source_char) {
            pattern_char = pattern.next();
            match_indexes.push(index);
        }
    }
    Matches { match_indexes, source }
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

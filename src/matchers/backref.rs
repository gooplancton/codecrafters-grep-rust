use super::{literal::LiteralSubstringCharacterClass, Match, Matcher};

pub struct Backref {
    idx: usize,
}

impl Backref {
    pub fn new(idx: usize) -> Self {
        Self { idx }
    }
}

impl Matcher for Backref {
    fn len(&self) -> usize {
        2 // NOTE: up to 9 backrefs are supported => only 1 digit + backslash
    }

    fn extend_from(&self, input_line: &str, previous_match: Match) -> super::MatchResult {
        let range = previous_match
            .captures
            .get(self.idx)
            .expect("backref index out of range");

        let substring = input_line.get(range.start..range.end.unwrap()).unwrap();

        LiteralSubstringCharacterClass(substring).extend_from(input_line, previous_match)
    }
}

use super::{Match, Matcher};

pub struct LiteralCharCharacterClass(pub char);

impl Matcher for LiteralCharCharacterClass {
    fn len(&self) -> usize {
        1
    }

    fn extend_from(&self, input_line: &str, previous_match: Match) -> super::MatchResult {
        let char = input_line.chars().nth(previous_match.offset);
        if char == Some(self.0) {
            return Ok(previous_match + 1);
        }

        Err(previous_match)
    }
}

pub struct LiteralSubstringCharacterClass<'substring>(pub &'substring str);

impl Matcher for LiteralSubstringCharacterClass<'_> {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn extend_from(&self, input_line: &str, previous_match: Match) -> super::MatchResult {
        let substring = input_line.get(previous_match.offset..);
        if substring.is_some_and(|substring| substring.starts_with(self.0)) {
            return Ok(previous_match + self.0.len());
        }

        Err(previous_match)
    }
}

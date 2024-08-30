use super::{Match, MatchResult, Matcher};

pub struct AlphanumericCharacterClass;

impl Matcher for AlphanumericCharacterClass {
    fn len(&self) -> usize {
        2
    }

    fn extend_from(&self, input_line: &str, previous_match: Match) -> MatchResult {
        let char = input_line.chars().nth(previous_match.offset);

        if char.is_some_and(|char| char.is_ascii_alphanumeric()) {
            return Ok(previous_match + 1);
        }

        Err(previous_match)
    }
}

use super::{Match, MatchResult, Matcher};

pub struct WildcardCharacterClass;

impl Matcher for WildcardCharacterClass {
    fn len(&self) -> usize {
        1
    }

    fn extend_from(&self, input_line: &str, previous_match: Match) -> MatchResult {
        let char = input_line.chars().nth(previous_match.offset);
        if char.is_none() || char.unwrap() == '\n' {
            return Err(previous_match);
        }

        Ok(previous_match + 1)
    }
}

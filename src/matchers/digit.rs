use super::{Match, Matcher};

pub struct DigitCharacterClass;

impl Matcher for DigitCharacterClass {
    fn len(&self) -> usize {
        2
    }

    fn extend_from(&self, input_line: &str, previous_match: Match) -> super::MatchResult {
        let char = input_line.chars().nth(previous_match.offset);
        if char.is_some_and(|char| char.is_ascii_digit()) {
            return Ok(previous_match + 1);
        }

        Err(previous_match)
    }
}

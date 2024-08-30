use super::{Match, Matcher};

pub struct StartOfLineCharacterClass;

impl Matcher for StartOfLineCharacterClass {
    fn len(&self) -> usize {
        1
    }

    fn extend_from(&self, _input_line: &str, previous_match: Match) -> super::MatchResult {
        if previous_match.offset == 0 {
            return Ok(previous_match);
        }

        Err(previous_match)
    }
}

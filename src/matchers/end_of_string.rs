use super::{Match, Matcher};

pub struct EndOfLineCharacterClass;

impl Matcher for EndOfLineCharacterClass {
    fn len(&self) -> usize {
        1
    }

    fn extend_from(&self, input_line: &str, previous_match: Match) -> super::MatchResult {
        let end_of_line = input_line.get(previous_match.offset..);
        if end_of_line == Some("") || end_of_line == Some("\n") {
            Ok(previous_match)
        } else {
            Err(previous_match)
        }
    }
}

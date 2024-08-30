use itertools::Itertools;

use super::{Match, Matcher};

pub struct CharGroupCharacterClass<'group> {
    pub chars: &'group str,
    pub is_positive: bool,
}

impl Matcher for CharGroupCharacterClass<'_> {
    fn len(&self) -> usize {
        if self.is_positive {
            self.chars.len() + 2
        } else {
            self.chars.len() + 3
        }
    }

    fn extend_from(&self, input_line: &str, previous_match: Match) -> super::MatchResult {
        let char = input_line.chars().nth(previous_match.offset);
        if char.is_some_and(|char| self.chars.chars().contains(&char) == self.is_positive) {
            return Ok(previous_match + 1);
        }

        Err(previous_match)
    }
}

use super::{alternation::Alternation, InsertAt, Match, Matcher};

pub struct CapturingGroupCharacterClass<'outer> {
    idx: usize,
    inner_scope: &'outer str,
    rest_of_outer_scope: &'outer str,
}

impl<'regex> CapturingGroupCharacterClass<'regex> {
    pub fn new(idx: usize, inner_regex: &'regex str, rest_of_outer_scope: &'regex str) -> Self {
        Self {
            inner_scope: inner_regex,
            rest_of_outer_scope,
            idx,
        }
    }
}

impl Matcher for CapturingGroupCharacterClass<'_> {
    fn len(&self) -> usize {
        self.inner_scope.len() + 2
    }

    fn extend_from(&self, input_line: &str, previous_match: Match) -> super::MatchResult {
        let group_matcher =
            Alternation::new(self.inner_scope, self.rest_of_outer_scope, Some(self.idx));

        let mut group_match = group_matcher.extend_from(input_line, previous_match)?;
        group_match.captures.insert_at(self.idx, group_match.offset);

        Ok(group_match)
    }
}

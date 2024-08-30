use super::{
    alternation::Alternation, pattern::Pattern, AnyMatcher, InsertAt, Match, MatchResult, Matcher,
};

pub struct ZeroOrOneQuantifier<'outer> {
    pub matcher: AnyMatcher<'outer>,
    pub rest_of_inner_scope: &'outer str,
    pub group_idx: Option<usize>,
    pub rest_of_outer_scope: &'outer str,
}

impl Matcher for ZeroOrOneQuantifier<'_> {
    fn len(&self) -> usize {
        self.matcher.len() + 1
    }

    fn extend_from(&self, input_line: &str, previous_match: Match) -> MatchResult {
        let previous_match_offset = previous_match.offset;
        let previous_match_captures = previous_match.captures.len();
        let first_submatch = self.matcher.extend_from(input_line, previous_match);
        if let Err(previous_match) = first_submatch {
            return Ok(previous_match.rollback(previous_match_offset, previous_match_captures));
        }

        let submatch = first_submatch.unwrap();
        let submatch_offset = submatch.offset;
        let submatch_captures = submatch.captures.len();

        let inner_matcher = Alternation::new(
            self.rest_of_inner_scope,
            self.rest_of_outer_scope,
            self.group_idx,
        );

        match inner_matcher.extend_from(input_line, submatch) {
            Err(previous_match) => Ok(previous_match.rollback(submatch_offset, submatch_captures)),
            Ok(mut inner_match) => {
                if let Some(group_idx) = self.group_idx {
                    inner_match
                        .captures
                        .insert_at(group_idx, inner_match.offset);
                }

                let outer_matcher = Pattern::new(self.rest_of_outer_scope, "", None);
                match outer_matcher.extend_from(input_line, inner_match) {
                    Ok(outer_match) => Ok(outer_match.rollback(submatch_offset, submatch_captures)),
                    Err(previous_match) => {
                        Ok(previous_match.rollback(submatch_offset, submatch_captures))
                    }
                }
            }
        }
    }
}

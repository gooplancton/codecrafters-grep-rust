use super::{
    alternation::Alternation, pattern::Pattern, AnyMatcher, InsertAt, Match, Matcher,
};

pub struct OneOrMoreQuantifer<'outer> {
    pub inner_matcher: AnyMatcher<'outer>,
    pub rest_of_inner_scope: &'outer str,
    pub group_idx: Option<usize>,
    pub rest_of_outer_scope: &'outer str,
}

impl Matcher for OneOrMoreQuantifer<'_> {
    fn len(&self) -> usize {
        self.inner_matcher.len() + 1
    }

    fn extend_from(&self, input_line: &str, previous_match: Match) -> super::MatchResult {
        let first_submatch = self.inner_matcher.extend_from(input_line, previous_match)?;

        let first_increment = first_submatch.offset;
        let mut increments = vec![first_increment];

        let mut whole_match = first_submatch;
        let mut max_match = loop {
            let old_offset = whole_match.offset;
            let old_captures = whole_match.captures.len();
            match self.inner_matcher.extend_from(input_line, whole_match) {
                Err(max_match) => break max_match.rollback(old_offset, old_captures),
                Ok(extended_match) => {
                    let new_offset = extended_match.offset;
                    increments.push(new_offset - old_offset);

                    whole_match = extended_match;
                }
            }
        };

        let inner_matcher = Alternation::new(
            self.rest_of_inner_scope,
            self.rest_of_outer_scope,
            self.group_idx,
        );

        for past_increment in increments.into_iter().rev() {
            let max_match_offset = max_match.offset;
            let max_match_captures = max_match.captures.len();

            max_match = match inner_matcher.extend_from(input_line, max_match) {
                Err(same_match) => same_match.rollback(max_match_offset, max_match_captures),

                Ok(mut inner_match) => {
                    if let Some(group_idx) = self.group_idx {
                        inner_match
                            .captures
                            .insert_at(group_idx, inner_match.offset);
                    }

                    let outer_matcher = Pattern::new(self.rest_of_outer_scope, "", None);
                    match outer_matcher.extend_from(input_line, inner_match) {
                        Err(same_match) => {
                            same_match.rollback(max_match_offset, max_match_captures)
                        }

                        Ok(outer_match) => {
                            let final_match =
                                outer_match.rollback(max_match_offset, max_match_captures);

                            return Ok(final_match);
                        }
                    }
                }
            };

            max_match -= past_increment;
        }

        Err(max_match)
    }
}

use super::{pattern::Pattern, Match, MatchResult, Matcher};

pub struct Alternation<'regex> {
    subpatterns: Vec<&'regex str>,
    rest_of_outer_scope: &'regex str,
    group_idx: Option<usize>
}

impl Matcher for Alternation<'_> {
    fn extend_from(&self, input_line: &str, previous_match: Match) -> MatchResult {
        let mut previous_match = previous_match;
        for subpattern in self.subpatterns.iter() {
            let submatcher = Pattern::new(subpattern, self.rest_of_outer_scope, self.group_idx);
            match submatcher.extend_from(input_line, previous_match) {
                Ok(submatch) => return Ok(submatch),
                Err(_previous_match) => previous_match = _previous_match,
            }
        }

        Err(previous_match)
    }

    fn len(&self) -> usize {
        let n_patterns = self.subpatterns.len();
        let n_pipes = n_patterns - 1;
        let subpatterns_len = self
            .subpatterns
            .iter()
            .map(|pattern| pattern.len())
            .sum::<usize>();

        n_pipes + subpatterns_len
    }
}

impl<'regex> Alternation<'regex> {
    // TODO: rewrite using find_matching_paren util
    pub fn new(regex: &'regex str, rest_of_outer_scope: &'regex str, group_idx: Option<usize>) -> Self {
        let mut subpatterns = vec![];
        let mut innestation = 0;

        let mut subpattern_start = 0;
        for idx in 0..regex.len() {
            let char = regex.chars().nth(idx).unwrap();

            if char == '(' && (idx == 0 || regex.chars().nth(idx - 1) != Some('\\')) {
                innestation += 1;
                continue;
            } else if char == ')' && (idx == 0 || regex.chars().nth(idx - 1) != Some('\\')) {
                innestation -= 1;
                continue;
            }

            if innestation == 0
                && char == '|'
                && (idx == 0 || regex.chars().nth(idx - 1) != Some('\\'))
            {
                subpatterns.push(regex.get(subpattern_start..idx).unwrap());
                subpattern_start = idx + 1;
            }
        }

        subpatterns.push(regex.get(subpattern_start..).unwrap());

        Self { subpatterns, rest_of_outer_scope, group_idx }
    }
}

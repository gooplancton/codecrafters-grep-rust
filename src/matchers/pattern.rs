use crate::{
    matchers::{capturing_group::CapturingGroupCharacterClass, AnyMatcher},
    utils::{find_matching_bracket, find_matching_paren},
};

use super::{
    alphanumeric::AlphanumericCharacterClass, backref::Backref,
    char_group::CharGroupCharacterClass, digit::DigitCharacterClass,
    end_of_string::EndOfLineCharacterClass, literal::LiteralCharCharacterClass,
    one_or_more_quantifier::OneOrMoreQuantifer, start_of_string::StartOfLineCharacterClass,
    wildcard::WildcardCharacterClass, zero_or_more_quantifier::ZeroOrMoreQuantifier,
    zero_or_one_quantifier::ZeroOrOneQuantifier, Capture, Match, MatchResult, Matcher,
};

#[derive(PartialEq, Eq, Debug)]
pub struct Pattern<'regex> {
    regex: &'regex str,
    rest_of_outer_scope: &'regex str,
    group_idx: Option<usize>,
}

impl<'regex> Pattern<'regex> {
    pub fn new(
        regex: &'regex str,
        rest_of_outer_scope: &'regex str,
        group_idx: Option<usize>,
    ) -> Self {
        Self {
            regex,
            rest_of_outer_scope,
            group_idx,
        }
    }
}

impl Matcher for Pattern<'_> {
    fn extend_from(&self, input_line: &str, mut previous_match: Match) -> MatchResult {
        let adj_idx = self
            .group_idx
            .map(|group_idx| group_idx + 1)
            .unwrap_or_default();

        let closed_captures: usize = previous_match
            .captures
            .iter()
            .map(|capture| if capture.end.is_some() { 1 } else { 0 })
            .sum();

        let next_group_idx = adj_idx + closed_captures;

        let first_matcher = self.next_matcher(next_group_idx);
        let inner_match = match first_matcher {
            None => previous_match,
            Some((first_matcher, is_in_group)) => {
                if is_in_group {
                    previous_match.captures.push(Capture {
                        start: previous_match.offset,
                        end: None,
                    })
                }

                let first_match = first_matcher.extend_from(input_line, previous_match)?;
                let rest_of_pattern = self.regex.get(first_matcher.len()..);

                if rest_of_pattern.is_none() || rest_of_pattern.unwrap().is_empty() {
                    first_match
                } else {
                    let rest_of_pattern = rest_of_pattern.unwrap();

                    Pattern::new(rest_of_pattern, self.rest_of_outer_scope, self.group_idx)
                        .extend_from(input_line, first_match)?
                }
            }
        };

        Ok(inner_match)
    }

    fn len(&self) -> usize {
        self.regex.len()
    }
}

impl<'regex> Pattern<'regex> {
    fn next_matcher(&'regex self, next_group_idx: usize) -> Option<(AnyMatcher<'regex>, bool)> {
        let mut chars = self.regex.char_indices().peekable();
        let (matcher_start, next_char) = chars.next()?;

        let mut is_in_group = false;

        let next_matcher: AnyMatcher = match next_char {
            '\\' => {
                let (_, next_char) = chars.next().expect("unterminated escape in pattern string");
                match next_char {
                    'd' => Box::new(DigitCharacterClass),
                    'w' => Box::new(AlphanumericCharacterClass),
                    '\\' => Box::new(LiteralCharCharacterClass('\\')),
                    '?' => Box::new(LiteralCharCharacterClass('?')),
                    '^' => Box::new(LiteralCharCharacterClass('^')),
                    '+' => Box::new(LiteralCharCharacterClass('+')),
                    '.' => Box::new(LiteralCharCharacterClass('.')),
                    '*' => Box::new(LiteralCharCharacterClass('*')),
                    '[' => Box::new(LiteralCharCharacterClass('[')),
                    ']' => Box::new(LiteralCharCharacterClass(']')),
                    '(' => Box::new(LiteralCharCharacterClass('(')),
                    ')' => Box::new(LiteralCharCharacterClass(')')),
                    '|' => Box::new(LiteralCharCharacterClass('|')),
                    '1'..'9' => {
                        let backref_idx = next_char as usize - '1' as usize;
                        Box::new(Backref::new(backref_idx))
                    }
                    _ => panic!("unrecognized escape sequence in pattern string"),
                }
            }
            '(' => {
                let group_end_idx = find_matching_paren(self.regex, 0)
                    .expect("unterminated capturing group in pattern string");

                is_in_group = true;

                let inner_scope = self.regex.get(matcher_start + 1..group_end_idx).unwrap();
                let rest_of_outer_scope = self.regex.get(group_end_idx + 1..).unwrap();

                Box::new(CapturingGroupCharacterClass::new(
                    next_group_idx,
                    inner_scope,
                    rest_of_outer_scope,
                ))
            }
            '[' => {
                let is_positive = chars
                    .peek()
                    .expect("unterminated group in pattern string")
                    .1
                    != '^';

                let group_start = matcher_start + if is_positive { 1 } else { 2 };
                let group_end = find_matching_bracket(self.regex, 0)
                    .expect("unterminated capturing group in pattern string");

                let group_chars = self.regex.get(group_start..group_end).unwrap();
                let group_class = CharGroupCharacterClass {
                    chars: group_chars,
                    is_positive,
                };

                Box::new(group_class)
            }
            '.' => Box::new(WildcardCharacterClass),
            '^' => Box::new(StartOfLineCharacterClass),
            '$' => Box::new(EndOfLineCharacterClass),
            ')' => panic!("unterminated capturing group in pattern string"),
            ']' => panic!("unterminated group in pattern string"),
            char => Box::new(LiteralCharCharacterClass(char)),
        };

        let class_len = next_matcher.as_ref().len();
        let quantifier_symbol = self.regex.chars().nth(class_len);

        let quantified_matcher = match quantifier_symbol {
            Some('+') | Some('?') | Some('*') => {
                let rest_of_regex = self.regex.get(class_len + 1..).unwrap_or_default();

                let quantified_class: AnyMatcher = match quantifier_symbol.unwrap() {
                    '+' => Box::new(OneOrMoreQuantifer {
                        inner_matcher: next_matcher,
                        rest_of_inner_scope: rest_of_regex,
                        group_idx: self.group_idx,
                        rest_of_outer_scope: self.rest_of_outer_scope,
                    }),
                    '?' => Box::new(ZeroOrOneQuantifier {
                        matcher: next_matcher,
                        rest_of_inner_scope: rest_of_regex,
                        group_idx: self.group_idx,
                        rest_of_outer_scope: self.rest_of_outer_scope,
                    }),
                    '*' => Box::new(ZeroOrMoreQuantifier {
                        inner_matcher: next_matcher,
                        rest_of_inner_scope: rest_of_regex,
                        group_idx: self.group_idx,
                        rest_of_outer_scope: self.rest_of_outer_scope,
                    }),
                    _ => unreachable!(),
                };

                quantified_class
            }
            _ => next_matcher,
        };

        Some((quantified_matcher, is_in_group))
    }
}

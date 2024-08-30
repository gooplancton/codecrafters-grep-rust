use std::ops::{Add, AddAssign, Sub, SubAssign};

pub mod alphanumeric;
pub mod alternation;
pub mod backref;
pub mod capturing_group;
pub mod char_group;
pub mod digit;
pub mod end_of_string;
pub mod literal;
pub mod one_or_more_quantifier;
pub mod pattern;
pub mod start_of_string;
pub mod wildcard;
pub mod zero_or_more_quantifier;
pub mod zero_or_one_quantifier;

pub trait Matcher {
    fn len(&self) -> usize;
    fn extend_from(&self, input_line: &str, previous_match: Match) -> MatchResult;
}

pub type AnyMatcher<'pattern> = Box<dyn Matcher + 'pattern>;

#[derive(Debug, Default)]
pub struct Capture {
    pub start: usize,
    pub end: Option<usize>,
}

#[derive(Default, Debug)]
pub struct Match {
    pub offset: usize,
    pub captures: Vec<Capture>,
}

pub trait InsertAt<T: Default> {
    fn insert_at(&mut self, idx: usize, item: T);
}

impl InsertAt<usize> for Vec<Capture> {
    fn insert_at(&mut self, idx: usize, offset: usize) {
        if idx >= self.len() {
            self.extend((0..idx - self.len() + 1).map(|_| Capture::default()));
        }

        self[idx].end = Some(offset);
    }
}

type ExtendedMatch = Match;
// NOTE: it's essential to keep the old match to repeatedly test different substring
type OldMatch = Match;
pub type MatchResult = Result<ExtendedMatch, OldMatch>;

impl Add<usize> for Match {
    type Output = Match;

    fn add(mut self, rhs: usize) -> Self::Output {
        self.offset += rhs;

        self
    }
}

impl AddAssign<usize> for Match {
    fn add_assign(&mut self, rhs: usize) {
        self.offset += rhs;
    }
}

impl AddAssign<Match> for Match {
    fn add_assign(&mut self, rhs: Match) {
        self.offset += rhs.offset;
        self.captures.extend(rhs.captures);
    }
}

impl Add<Match> for Match {
    type Output = Match;

    fn add(mut self, rhs: Match) -> Match {
        self.offset += rhs.offset;
        self.captures.extend(rhs.captures);

        self
    }
}

impl Sub<usize> for Match {
    type Output = Match;

    fn sub(mut self, rhs: usize) -> Self::Output {
        self.offset -= rhs;

        self
    }
}

impl SubAssign<usize> for Match {
    fn sub_assign(&mut self, rhs: usize) {
        self.offset -= rhs;
    }
}

impl Match {
    pub fn new(offset: usize) -> Self {
        Self {
            offset,
            captures: vec![],
        }
    }

    pub fn rollback(mut self, previous_offset: usize, previous_captures_len: usize) -> Self {
        self.offset = previous_offset;
        self.captures.drain(previous_captures_len..);

        self
    }
}

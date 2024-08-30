use itertools::Itertools;

pub fn find_matching_paren(line: &str, open_paren_idx: usize) -> Option<usize> {
    let mut innestation = 0;
    let mut chars = line.char_indices().get(open_paren_idx..);
    let (_, open_paren) = chars.next()?;
    if open_paren != '(' {
        return None;
    }

    for (idx, char) in chars {
        match char {
            '(' => innestation += 1,
            ')' if innestation > 0 => innestation -= 1,
            ')' if innestation == 0 => return Some(idx),
            _ => continue
        }
    }

    None
}

pub fn find_matching_bracket(line: &str, open_bracket_idx: usize) -> Option<usize> {
    let mut innestation = 0;
    let mut chars = line.char_indices().get(open_bracket_idx..);
    let (_, open_bracket) = chars.next()?;
    if open_bracket != '[' {
        return None;
    }

    for (idx, char) in chars {
        match char {
            '[' => innestation += 1,
            ']' if innestation > 0 => innestation -= 1,
            ']' if innestation == 0 => return Some(idx),
            _ => continue
        }
    }


    None
}

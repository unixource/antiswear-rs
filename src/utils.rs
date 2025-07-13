pub fn split(string: &str) -> Vec<String> {
    let mut splitted = string
        .split_whitespace()
        .map(str::to_string)
        .collect::<Vec<String>>();
    splitted.pop_if(|x| x.is_empty());
    splitted
}

pub fn add(a: Vec<String>, b: Vec<String>) -> Vec<String> {
    let mut out = b.clone();
    for first in a {
        for second in &b {
            out.push(first.clone() + second);
        }
    }
    out
}

pub fn utf8_slice(s: &str, start: usize, end: usize) -> Option<&str> {
    let mut iter = s.char_indices()
        .map(|(pos, _)| pos)
        .chain(Some(s.len()))
        .skip(start)
        .peekable();
    let start_pos = *iter.peek()?;
    for _ in start..end { iter.next(); }
    Some(&s[start_pos..*iter.peek()?])
}


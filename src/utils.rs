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

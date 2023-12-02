use std::fs;

pub fn read_lines(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).unwrap();
    let mut lines = contents.split('\n').map(|s| s.to_string()).collect::<Vec<_>>();
    lines.remove(lines.len() - 1);
    lines
}


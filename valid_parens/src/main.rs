use std::vec::Vec;

fn main() {
    println!("Hello, world!");
}

enum ParenType {
    Paren,
    Brace,
    Bracket
}

// This was my first proper attempt at the problem
pub fn is_valid(s: String) -> bool {
    let mut open_parens: Vec<ParenType> = Vec::new();

    for c in s.chars() {
        match c {
            '(' => open_parens.push(ParenType::Paren),
            ')' => {
                if matches!(open_parens.pop(), Some(ParenType::Paren)) {
                    continue;
                } else {
                    return false;
                }
            },
            '{' => open_parens.push(ParenType::Brace),
            '}' => {
                if matches!(open_parens.pop(), Some(ParenType::Brace)) {
                    continue;
                } else {
                    return false;
                }
            },
            '[' => open_parens.push(ParenType::Bracket),
            ']' => {
                if matches!(open_parens.pop(), Some(ParenType::Bracket)) {
                    continue;
                } else {
                    return false;
                }
            },
            _ => continue,
        }
    }
    open_parens.len() == 0
}

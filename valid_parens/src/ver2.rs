use std::vec::Vec;
// According to leetcode this is a 0ms solution!
fn main() {
    println!("Hello, world!");
}

// This was my second attempt at the problem
// I removed the use of an enum as it wasn't necessary
// I should have just used chars instead of i8s and would've been far more readable
// I also should have used match instead of if statements in the close paren matches
pub fn is_valid(s: String) -> bool {
    let mut open_parens: Vec<i8> = Vec::new();
    // 0 is paren
    // 1 is brace
    // 2 is bracket
    for c in s.chars() {
        match c {
            '(' => open_parens.push(0i8),
            ')' => {
                if open_parens.pop() == Some(0i8) {
                    continue;
                } else {
                    return false;
                }
            },
            '{' => open_parens.push(1i8),
            '}' => {
                if open_parens.pop() == Some(1i8) {
                    continue;
                } else {
                    return false;
                }
            },
            '[' => open_parens.push(2i8),
            ']' => {
                if open_parens.pop() == Some(2i8) {
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

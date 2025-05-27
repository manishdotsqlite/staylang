/// This function returns the nth character of a string.
pub fn nth_char(input: &str, n: i32) -> char {
        input.chars().nth(n as usize).unwrap_or('\0')
}
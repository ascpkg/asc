pub fn reverse_string(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.reverse();
    chars.iter().collect()
}

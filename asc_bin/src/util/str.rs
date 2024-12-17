pub fn reverse_string(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.reverse();
    chars.iter().collect()
}

pub fn longest_common_prefix(s1: &str, s2: &str) -> String {
    let mut prefix = s1;

    while !s2.starts_with(prefix) {
        prefix = &prefix[0..prefix.len() - 1];
        if prefix.is_empty() {
            return String::new();
        }
    }

    prefix.to_string()
}

pub fn longest_common_substring(s1: &str, s2: &str) -> String {
    let m = s1.len();
    let n = s2.len();
    let mut end_index = 0;
    let mut longest_length = 0;

    let mut prev = vec![0; n + 1];
    let mut curr = vec![0; n + 1];

    for i in 1..=m {
        for j in 1..=n {
            if s1.as_bytes()[i - 1] == s2.as_bytes()[j - 1] {
                curr[j] = prev[j - 1] + 1;
                if curr[j] > longest_length {
                    longest_length = curr[j];
                    end_index = i;
                }
            } else {
                curr[j] = 0;
            }
        }

        std::mem::swap(&mut prev, &mut curr);
    }

    s1[end_index - longest_length..end_index].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let test_cases = vec![
            ("flower", "flow-", "flow"),
            ("dog", "racecar", ""),
            ("apple", "applepie", "apple"),
            ("", "apple", ""),
            ("abc", "", ""),
            ("a", "a", "a"),
        ];

        for (s1, s2, expected) in test_cases {
            let result = longest_common_prefix(s1, s2);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_longest_common_substring() {
        let test_cases = vec![
            ("abcdef", "zcdemf", "cde"),
            ("abcdef", "abcdef", "abcdef"),
            ("abcdef", "xyzabc", "abc"),
            ("abc", "def", ""),
            ("abc", "abcabc", "abc"),
            ("a", "a", "a"),
            ("a", "b", ""),
            ("", "nonempty", ""),
            ("nonempty", "", ""),
            ("hello world", "helloworld", "hello"),
        ];

        for (s1, s2, expected) in test_cases {
            let result = longest_common_substring(s1, s2);
            assert_eq!(result, expected);
        }
    }
}

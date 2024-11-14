pub fn reverse_string(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.reverse();
    chars.iter().collect()
}

pub fn longest_common_substring(s1: &str, s2: &str) -> String {
    let m = s1.len();
    let n = s2.len();
    let mut end_index = 0;
    let mut longest_length = 0;

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if s1.as_bytes()[i - 1] == s2.as_bytes()[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
                if dp[i][j] > longest_length {
                    longest_length = dp[i][j];
                    end_index = i;
                }
            }
        }
    }

    s1[end_index - longest_length..end_index].to_string()
}

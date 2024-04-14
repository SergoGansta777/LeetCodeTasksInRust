impl Solution {
    pub fn reverse_words(s: String) -> String {
        let result: Vec<&str> = s.trim().split_whitespace().rev().collect();
        result.join(" ")
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_case() {
        let s = "the sky is blue".to_string();
        let expected = "blue is sky the".to_string();

        let result = Solution::reverse_words(s);

        assert_eq!(result, expected);
    }

    #[test]
    fn handle_untrimmed() {
        let s = "test this now".to_string();
        let expected = "now this test".to_string();

        let result = Solution::reverse_words(s);

        assert_eq!(result, expected);
    }
}

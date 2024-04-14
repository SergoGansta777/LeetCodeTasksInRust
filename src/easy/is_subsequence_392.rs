use std::char;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if t.len() < s.len() {
            return false;
        }
        if s.len() == 0 {
            return true;
        }

        let s_chars: Vec<char> = s.chars().collect();
        let mut cur_s_position = 0usize;
        for t_ch in t.chars() {
            if t_ch == s_chars[cur_s_position] {
                cur_s_position += 1;
                if cur_s_position == s.len() {
                    break;
                }
            }
        }

        cur_s_position == s.len()
    }
}

struct Solution;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn handle_true_case() {
        let s = "abc".to_string();
        let t = "ahbgdc".to_string();
        let expected = true;

        let result = Solution::is_subsequence(s, t);

        assert_eq!(result, expected);
    }

    #[test]
    fn handle_false_case() {
        let s = "axc".to_string();
        let t = "ahbgdc".to_string();
        let expected = false;

        let result = Solution::is_subsequence(s, t);

        assert_eq!(result, expected);
    }
}

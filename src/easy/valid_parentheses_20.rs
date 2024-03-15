use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let opening_from_closing = HashMap::from([(']', '['), (')', '('), ('}', '{')]);

        for c in s.chars() {
            match opening_from_closing.get(&c) {
                Some(&matching_char) if stack.last() == Some(&matching_char) => {
                    stack.pop();
                }
                None => stack.push(c),
                _ => return false,
            }
        }
        stack.is_empty()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_correct_answer() {
        let s = "()".to_string();
        let expected = true;

        let result = Solution::is_valid(s);

        assert_eq!(result, expected);
    }

    #[test]
    fn correctly_handle_different_type_of_brackets() {
        let s = "(){}[]".to_string();
        let expected = true;

        let result = Solution::is_valid(s);

        assert_eq!(result, expected);
    }

    #[test]
    fn different_type_of_brackents_dont_close_up() {
        let s = "(]".to_string();
        let expected = false;

        let result = Solution::is_valid(s);

        assert_eq!(result, expected);
    }

    #[test]
    fn all_brackets_need_to_be_closed() {
        let s = "())".to_string();
        let expected = false;

        let result = Solution::is_valid(s);

        assert_eq!(result, expected);
    }

    #[test]
    fn all_brackets_need_to_be_closed_sequentially() {
        let s = "([[)]".to_string();
        let expected = false;

        let result = Solution::is_valid(s);

        assert_eq!(result, expected);
    }
}

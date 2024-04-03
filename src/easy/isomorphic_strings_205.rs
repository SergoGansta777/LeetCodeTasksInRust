use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let chars = s.chars().zip(t.chars());
        let mut mapping: HashMap<char, char> = HashMap::new();
        let mut mapped_values: HashSet<char> = HashSet::new();

        for (s_ch, t_ch) in chars {
            match mapping.insert(s_ch, t_ch) {
                None => {
                    if mapped_values.contains(&t_ch) {
                        return false;
                    }
                    mapped_values.insert(t_ch);
                }
                Some(mapped_value) => {
                    if t_ch != mapped_value {
                        return false;
                    }
                }
            }
        }

        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_happy_case_1() {
        let s = "egg".to_string();
        let t = "add".to_string();
        let expected = true;

        let result = Solution::is_isomorphic(s, t);

        assert_eq!(result, expected);
    }

    #[test]
    fn handle_happy_case_2() {
        let s = "paper".to_string();
        let t = "title".to_string();
        let expected = true;

        let result = Solution::is_isomorphic(s, t);

        assert_eq!(result, expected);
    }

    #[test]
    fn handle_unhappy_case_2() {
        let s = "foo".to_string();
        let t = "bar".to_string();
        let expected = false;

        let result = Solution::is_isomorphic(s, t);

        assert_eq!(result, expected);
    }

    #[test]
    fn handle_edge_case_2() {
        let s = "badc".to_string();
        let t = "baba".to_string();
        let expected = false;

        let result = Solution::is_isomorphic(s, t);

        assert_eq!(result, expected);
    }
}

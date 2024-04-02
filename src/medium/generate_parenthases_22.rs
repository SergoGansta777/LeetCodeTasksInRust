impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res: Vec<String> = vec![];

        Self::fallback(&mut res, "".to_string(), n, n);
        res
    }

    fn fallback(res: &mut Vec<String>, cur_seq: String, open: i32, close: i32) {
        if open == 0 && close == 0 {
            res.push(cur_seq);
            return;
        }

        if open == close {
            Self::fallback(res, cur_seq.clone() + "(", open - 1, close);
        } else {
            if open > 0 {
                Self::fallback(res, cur_seq.clone() + "(", open - 1, close);
            }
            if close > 0 {
                Self::fallback(res, cur_seq.clone() + ")", open, close - 1);
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_generating_1() {
        let n = 3;
        let expected = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];

        let result = Solution::generate_parenthesis(n);

        assert_eq!(result, expected);
    }

    #[test]
    fn correct_generating_2() {
        let n = 1;
        let expected = vec!["()"];

        let result = Solution::generate_parenthesis(n);

        assert_eq!(result, expected);
    }
}

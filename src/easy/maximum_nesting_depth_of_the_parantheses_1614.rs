impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let (mut max_depth, mut cur_depth) = (0, 0);

        for ch in s.chars() {
            if ch == '(' {
                cur_depth += 1;
            } else if ch == ')' {
                cur_depth -= 1;
            }

            max_depth = max_depth.max(cur_depth);
        }

        max_depth
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s = "(1+(2*3)+((8)/4))+1".to_string();
        let expected = 3;

        let result = Solution::max_depth(s);

        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let s = "(1)+((2))+(((3)))".to_string();
        let expected = 3;

        let result = Solution::max_depth(s);

        assert_eq!(result, expected);
    }
}

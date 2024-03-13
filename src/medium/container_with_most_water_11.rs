use std::cmp::{max, min};

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut first = 0;
        let mut last = height.len() - 1;
        let mut max_area = 0;

        while first < last {
            let current_area = min(height[first], height[last]) * (last - first) as i32;
            max_area = max(max_area, current_area);
            if height[first] > height[last] {
                last -= 1;
            } else {
                first += 1;
            }
        }

        max_area
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_corrects_answer() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let expected = 49;

        let result = Solution::max_area(height);

        assert_eq!(result, expected);
    }

    #[test]
    fn handle_edge_case() {
        let height = vec![1, 1];
        let expected = 1;

        let result = Solution::max_area(height);

        assert_eq!(result, expected);
    }
}

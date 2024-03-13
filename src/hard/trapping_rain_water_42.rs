impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let (mut max_left, mut max_right) = (height[left], height[right]);
        let mut area = 0;

        while left < right {
            if max_left > max_right {
                right -= 1;
                max_right = max_right.max(height[right]);
                area += max_right - height[right];
            } else {
                left += 1;
                max_left = max_left.max(height[left]);
                area += max_left - height[left];
            }
        }

        area
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_correct_answer_1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let expected = 6;

        let result = Solution::trap(height);

        assert_eq!(result, expected);
    }

    #[test]
    fn return_correct_answer_2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        let expected = 9;

        let result = Solution::trap(height);

        assert_eq!(result, expected);
    }
}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }

        let mut max_jump = nums[0];
        for cur_jump_opportunity in nums {
            if max_jump == 0 {
                return false;
            }

            max_jump = cur_jump_opportunity.max(max_jump - 1);
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_true_case() {
        let nums = vec![2, 3, 1, 1, 4];
        let expected = true;

        let result = Solution::can_jump(nums);

        assert_eq!(result, expected);
    }

    #[test]
    fn handle_false_case() {
        let nums = vec![3, 2, 1, 0, 4];
        let expected = false;

        let result = Solution::can_jump(nums);

        assert_eq!(result, expected);
    }
}

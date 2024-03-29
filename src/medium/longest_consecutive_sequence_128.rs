use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set_nums: HashSet<i32> = nums.into_iter().collect();
        let mut max_length = 0;

        for num in &set_nums {
            if set_nums.contains(&(num - 1)) {
                continue;
            }
            let mut next_num = num + 1;
            let mut cur_length = 1;
            while set_nums.contains(&next_num) {
                cur_length += 1;
                next_num += 1
            }
            max_length = max_length.max(cur_length);
        }
        max_length
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_correct_answer_1() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        let expected = 4;

        let result = Solution::longest_consecutive(nums);

        assert_eq!(result, expected);
    }

    #[test]
    fn returns_correct_answer_2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        let expected = 9;

        let result = Solution::longest_consecutive(nums);

        assert_eq!(result, expected);
    }

    #[test]
    fn correctly_hande_empty_input() {
        let nums = vec![];
        let expected = 0;

        let result = Solution::longest_consecutive(nums);

        assert_eq!(result, expected);
    }
}

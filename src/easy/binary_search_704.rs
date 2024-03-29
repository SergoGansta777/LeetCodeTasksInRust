use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut first, mut last) = (0i32, nums.len() as i32 - 1);

        while first <= last {
            let mid = first + (last - first) / 2;
            match nums[mid as usize].cmp(&target) {
                Equal => return mid as i32,
                Less => first = mid + 1,
                Greater => last = mid - 1,
            }
        }
        -1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn returns_correct_answer_1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        let expected = 4;

        let result = Solution::search(nums, target);

        assert_eq!(result, expected);
    }

    #[test]
    fn returns_correct_answer_2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        let expected = -1;

        let result = Solution::search(nums, target);

        assert_eq!(result, expected);
    }

    #[test]
    fn correctly_handles_one_element_array() {
        let nums = vec![5];
        let target = 5;
        let expected = 0;

        let result = Solution::search(nums, target);

        assert_eq!(result, expected);
    }
}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let number_of_possible_dublicats = 2;
        if nums.len() <= number_of_possible_dublicats {
            return nums.len() as i32;
        }

        let mut cur_position = number_of_possible_dublicats;
        for i in number_of_possible_dublicats..nums.len() {
            let num = nums[i];
            nums[cur_position] = num;
            if nums[cur_position - 1] < num
                || nums[cur_position - 1] == num && nums[cur_position - 2] < num
            {
                cur_position += 1;
            }
        }

        cur_position as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let expected = 5;

        let result = Solution::remove_duplicates(&mut nums);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_case_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let expected = 7;

        let result = Solution::remove_duplicates(&mut nums);

        assert_eq!(result, expected);
    }
}

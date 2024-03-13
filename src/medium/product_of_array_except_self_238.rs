impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut products = vec![1; n];
        for i in 1..n {
            products[i] = nums[i - 1] * products[i - 1];
        }

        let mut right_prod = 1;
        for i in (0..n).rev() {
            products[i] *= right_prod;
            right_prod *= nums[i];
        }

        products
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_correct_answer_1() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![24, 12, 8, 6];

        let result = Solution::product_except_self(nums);

        assert_eq!(result, expected);
    }

    #[test]
    fn returns_correct_answer_2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let expected = vec![0, 0, 9, 0, 0];

        let result = Solution::product_except_self(nums);

        assert_eq!(result, expected);
    }
}

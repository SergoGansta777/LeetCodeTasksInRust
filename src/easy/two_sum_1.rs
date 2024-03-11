use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut have_at = HashMap::with_capacity(nums.len());
    for (i, num) in nums.into_iter().enumerate() {
        let needed = target - num;
        if let Some(&j) = have_at.get(&needed) {
            return vec![i as i32, j as i32];
        }
        have_at.insert(num, i);
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn happy_test_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        let result = two_sum(nums.clone(), target);

        assert!(result.len() == 2);
        assert_eq!(nums[result[0] as usize] + nums[result[1] as usize], target);
    }

    #[test]
    fn happy_test_2() {
        let nums = vec![3, 2, 4];
        let target = 6;

        let result = two_sum(nums.clone(), target);

        assert!(result.len() == 2);
        assert_eq!(nums[result[0] as usize] + nums[result[1] as usize], target);
    }

    #[test]
    fn happy_test_3() {
        let nums = vec![3, 3];
        let target = 6;

        let result = two_sum(nums.clone(), target);

        assert!(result.len() == 2);
        assert_eq!(nums[result[0] as usize] + nums[result[1] as usize], target);
    }
}

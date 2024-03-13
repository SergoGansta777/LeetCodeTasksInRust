use std::cmp::Ordering::{Equal, Greater, Less};

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut first = 0;
    let mut last = numbers.len() - 1;
    while first < last {
        let sum = numbers[first] + numbers[last];
        match (sum).cmp(&target) {
            Less => first += 1,
            Greater => last -= 1,
            Equal => return vec![first as i32 + 1, last as i32 + 1],
        }
    }
    unreachable!("Test did not follow the constraints!")
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn returns_correct_answer_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        let result = two_sum(nums.clone(), target);

        assert!(result.len() == 2);
        assert_eq!(
            nums[result[0] as usize - 1] + nums[result[1] as usize - 1],
            target
        );
    }

    #[test]
    fn returns_correct_answer_2() {
        let nums = vec![2, 3, 4];
        let target = 6;

        let result = two_sum(nums.clone(), target);

        assert!(result.len() == 2);
        assert_eq!(
            nums[result[0] as usize - 1] + nums[result[1] as usize - 1],
            target
        );
    }

    #[test]
    fn zero_hadnles_too() {
        let nums = vec![-1, 0];
        let target = -1;

        let result = two_sum(nums.clone(), target);

        assert!(result.len() == 2);
        assert_eq!(
            nums[result[0] as usize - 1] + nums[result[1] as usize - 1],
            target
        );
    }
}

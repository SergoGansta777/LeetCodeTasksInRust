fn single_number(nums: Vec<i32>) -> i32 {
    let mut single_number = 0;
    for num in nums {
        single_number ^= num;
    }
    single_number
}


#[cfg(test)]
mod tests {
    use super::single_number;

    #[test]
    fn returns_correct_single_element_1() {
        let nums = vec![2, 2, 1];
        let expected = 1;

        let result = single_number(nums);

        assert_eq!(result, expected);
    }
    #[test]
    fn returns_correct_single_element_2() {
        let nums = vec![4, 1, 2, 1, 2];
        let expected = 4;

        let result = single_number(nums);

        assert_eq!(result, expected);
    }

    #[test]
    fn returns_correct_single_element_3() {
        let nums = vec![1];
        let expected = 1;

        let result = single_number(nums);

        assert_eq!(result, expected);
    }
}

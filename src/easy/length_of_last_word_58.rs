// More brute forced solution
// impl Solution {
//     pub fn length_of_last_word(s: String) -> i32 {
//         let mut length = 0;
//         for ch in s.chars().into_iter().rev() {
//             if length != 0 && ch == ' ' {
//                 break;
//             }
//             if ch != ' ' {
//                 length += 1
//             }
//         }

//         length
//     }
// }

// More idiomatic solution
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim().split(' ').last().unwrap().len() as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_basic_case_1() {
        let s = "Hello World".to_string();
        let expected = 5;

        let result = Solution::length_of_last_word(s);

        assert_eq!(result, expected);
    }

    #[test]
    fn handle_basic_case_2() {
        let s = "uffy is still joyboy".to_string();
        let expected = 6;

        let result = Solution::length_of_last_word(s);

        assert_eq!(result, expected);
    }

    #[test]
    fn skip_spaces_in_the_end() {
        let s = "   fly me   to   the moon  ".to_string();
        let expected = 4;

        let result = Solution::length_of_last_word(s);

        assert_eq!(result, expected);
    }
}

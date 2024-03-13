/// My first solution
// pub fn is_palindrome(s: String) -> bool {
//     let chars: Vec<char> = s
//         .chars()
//         .filter(|x| x.is_alphanumeric())
//         .map(|x| x.to_lowercase().next().unwrap())
//         .collect();
//     let len = chars.len();

//     for i in 0..(len / 2) {
//         if chars[i] != chars[len - 1 - i] {
//             return false;
//         }
//     }
//     true
//}

/// More ideological way to solve this task from leetcode tops! And yeah here
/// two pointers too, but more rusty
pub fn is_palindrome(s: String) -> bool {
    let s = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();
    let mut first = s.chars();
    let mut last = s.chars().rev();
    while let (Some(c1), Some(c2)) = (first.next(), last.next()) {
        if c1 != c2 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::easy::valid_palindrome_125::is_palindrome;

    #[test]
    fn returns_correct_despite_spec_symbols_1() {
        let s = "A man, a plan, a canal: Panama".to_string();
        let expected = true;

        let result = is_palindrome(s);

        assert_eq!(result, expected);
    }

    #[test]
    fn returns_correct_despite_spec_symbols_2() {
        let s = "race a car".to_string();
        let expected = false;

        let result = is_palindrome(s);

        assert_eq!(result, expected);
    }

    #[test]
    fn for_white_space_only_line_returns_true() {
        let s = " ".to_string();
        let expected = true;

        let result = is_palindrome(s);

        assert_eq!(result, expected);
    }

    #[test]
    fn for_empty_line_returns_true() {
        let s = "".to_string();
        let expected = true;

        let result = is_palindrome(s);

        assert_eq!(result, expected);
    }

    #[test]
    fn correctly_handles_digits() {
        let s = "P0".to_string();
        let expected = false;

        let result = is_palindrome(s);

        assert_eq!(result, expected);
    }
}

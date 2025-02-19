/*
    Palindrome Check
    Given a string, check if it is a palindrome (i.e., it reads the same forward and backward).
    The solution should ignore case differences and non-alphabetical characters.

    You need to implement the function `is_palindrome(s: String) -> bool`.
    The function should return `true` if the string is a palindrome, and `false` otherwise.

    Hint: Consider normalizing the string by converting it to lowercase and removing non-alphabetical characters before checking.
*/

use std::fmt::{self, Display, Formatter};

fn remove_non_alphabetic(s: &str) -> String {
    s.chars().filter(|c| c.is_alphabetic()).collect::<String>()
}
pub fn is_palindrome(s: String) -> bool {
    // TODO: Implement the logic to check if the string is a palindrome

    let mut st = remove_non_alphabetic(s.as_str()).to_lowercase();
    if st.len() <= 1 {
        return true;
    }
    let mut res = true;
    let chars: Vec<char> = st.as_str().chars().collect();
    for i in 0..chars.len() / 2 {
        if chars[i] != chars[chars.len() - 1 - i] {
            res = false;
            break;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_1() {
        let s = "A man, a plan, a canal, Panama".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_2() {
        let s = "Racecar".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_3() {
        let s = "Hello, World!".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_palindrome_4() {
        let s = "No 'x' in Nixon".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_5() {
        let s = "Was it a car or a cat I saw?".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }
}

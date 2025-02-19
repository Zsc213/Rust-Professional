/*
    Longest Substring Without Repeating Characters
    Given a string, find the length of the longest substring without repeating characters.
    The substring must not contain any duplicate characters, and its length should be maximized.

    You need to implement the function `longest_substring_without_repeating_chars(s: String) -> i32`.
    The function should return the length of the longest substring without repeating characters.

    Hint: Consider using the sliding window technique to efficiently solve this problem in O(n) time complexity.
*/

use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};

fn not_repeated(string_s: &Vec<char>, l: usize, r: usize) -> bool {
    if l == r {
        return true;
    }
    let mut record = HashSet::new();
    let mut res = true;
    for i in l..r + 1 {
        if record.contains(&string_s[i]) {
            res = false;
            break;
        } else {
            record.insert(string_s[i].clone());
        }
    }
    res
}

pub fn longest_substring_without_repeating_chars(s: String) -> i32 {
    // TODO: Implement the logic to find the longest substring without repeating characters
    let st = s.as_str().chars().collect::<Vec<char>>();
    let mut l = 0;
    let mut r = 0;
    let mut res = 0;
    while r < st.len() {
        if not_repeated(&st, l, r) {
            r += 1;
            res += 1;
        } else {
            l += 1;
            r += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring_1() {
        let s = "abcabcbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3); // "abc"
    }

    #[test]
    fn test_longest_substring_2() {
        let s = "bbbbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 1); // "b"
    }

    #[test]
    fn test_longest_substring_3() {
        let s = "pwwkew".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3); // "wke"
    }

    #[test]
    fn test_longest_substring_4() {
        let s = "".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 0); // Empty string
    }

    #[test]
    fn test_longest_substring_5() {
        let s = "abcde".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 5); // "abcde"
    }
}

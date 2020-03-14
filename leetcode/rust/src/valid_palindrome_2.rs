use std::fs::read;
use std::intrinsics::fabsf32;

pub struct Solution {}

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        if Solution::is_palindrome(&s) {
            return true;
        }

        let mut i = 0;
        let mut j = s.len() - 1;

        let bytes = s.as_bytes();
        while i <= j {
            if bytes[i] != bytes[j] {
                if Solution::is_palindrome(&Solution::chop(&s, i))
                    || Solution::is_palindrome(&Solution::chop(&s, j)) {
                    return true;
                } else {
                    return false;
                }
            }
            i += 1;
            j -= 1;
        }

        false
    }

    fn is_palindrome(s: &String) -> bool {
        let reversed = s.chars().rev().collect::<String>();
        reversed == *s
    }

    fn chop(s: &String, i: usize) -> String {
        let mut result: Vec<u8> = Vec::from(s.as_bytes());
        result.remove(i);
        String::from_utf8(result).unwrap()
    }
}
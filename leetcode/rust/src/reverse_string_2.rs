use std::{cmp::min, collections::VecDeque};

pub struct Solution {}

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let k = k as usize;

        let mut result = String::with_capacity(s.len());

        let mut chars: VecDeque<char> = s.chars().collect();

        let mut reverse = true;
        let mut stack = VecDeque::new();
        while !chars.is_empty() {
            if reverse {
                for _ in 0..min(k, chars.len()) {
                    stack.push_front(chars.pop_front().unwrap());
                }
                while !stack.is_empty() {
                    result.push(stack.pop_front().unwrap());
                }
            } else {
                for _ in 0..min(k, chars.len()) {
                    result.push(chars.pop_front().unwrap());
                }
            }
            reverse = !reverse;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("bacdfeg", Solution::reverse_str("abcdefg".into(), 2));
        assert_eq!("bacd", Solution::reverse_str("abcd".into(), 2));
    }
}

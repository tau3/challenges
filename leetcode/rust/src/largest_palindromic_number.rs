use std::collections::{BTreeMap, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let mut counters = BTreeMap::new();
        for i in num.chars() {
            let value = counters.entry(i).or_insert_with(|| 0);
            *value += 1;
        }

        let mut right_half = VecDeque::new();
        for (digit, count) in counters.iter_mut() {
            let half = *count / 2;
            for _ in 0..half {
                right_half.push_back(*digit);
            }
            *count -= half * 2;
        }

        let maybe_middle = counters
            .iter()
            .filter(|(_, count)| **count != 0)
            .map(|(digit, _)| digit)
            .max();

        let mut result: VecDeque<char> = right_half.iter().copied().rev().collect();
        if let Some(middle) = maybe_middle {
            result.push_back(*middle);
        }
        result.append(&mut right_half);

        while let Some('0') = result.back() {
            result.pop_back();
            result.pop_front();
        }
        if result.is_empty() {
            result.push_back('0');
        }
        result.iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            "7449447".to_string(),
            Solution::largest_palindromic("444947137".into())
        );

        assert_eq!(
            "9".to_string(),
            Solution::largest_palindromic("00009".into())
        );
    }

    #[test]
    fn test_only_zeros() {
        assert_eq!(
            "0".to_string(),
            Solution::largest_palindromic("00000".into())
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            "1009001".to_string(),
            Solution::largest_palindromic("11000094".into())
        );
    }
}

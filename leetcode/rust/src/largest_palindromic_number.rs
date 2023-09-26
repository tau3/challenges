use std::collections::BTreeMap;

pub struct Solution {}

impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let mut counters = BTreeMap::new();
        for i in num.chars() {
            let value = counters.entry(i).or_insert_with(|| 0);
            *value += 1;
        }

        let mut right_half = String::from("");
        for (digit, count) in counters.iter_mut() {
            let half = *count / 2;
            for _ in 0..half {
                right_half.push(*digit);
            }
            *count -= half * 2;
        }

        let maybe_middle = counters
            .iter()
            .filter(|(_, count)| **count != 0)
            .map(|(digit, _)| digit)
            .max();

        let mut result: String = right_half.chars().rev().collect();
        if let Some(middle) = maybe_middle {
            result.push(*middle);
        }
        result.push_str(&right_half);
        result
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
}

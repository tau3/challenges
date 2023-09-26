use std::collections::{BTreeMap, HashMap};

pub struct Solution {}

impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let mut counter = BTreeMap::new();
        for i in num.chars() {
            let value = counter.entry(i).or_insert_with(|| 0);
            *value += 1;
        }

        let mut result = String::from("");
        for (k, v) in counter.iter() {
            let c = v / 2;
            for i in 0..c {
                result.push(*k);
            }
            counter.insert(*k, v - c * 2);
        }
        let maybe_middle = counter
            .iter()
            .filter(|(k, v)| **v != 0)
            .map(|(k, v)| k)
            .max();
        if let Some(x) = maybe_middle {
            let mut xxx: String = result.chars().rev().collect();
            // let xxx: String = result.chars().rev().collect::<Vec<char>>().into();
            xxx.push(*x);
            xxx.push_str(&result);
            return xxx;
        } else {
            let mut xxx: String = result.chars().rev().collect();
            xxx.push_str(&result);
            return xxx;
        }
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
    }
}

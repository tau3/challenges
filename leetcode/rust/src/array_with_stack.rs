use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let target_len = target.len();
        let mut result = Vec::with_capacity(target_len);
        let mut state = VecDeque::new();
        for i in 1..=n {
            state.push_back(i);
            result.push("Push".to_owned());
            if state.back().unwrap() != &target[state.len() - 1] {
                state.pop_back();
                result.push("Pop".to_owned());
            }
            if state.len() == target_len && target.last().unwrap() == state.back().unwrap() {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            vec!["Push", "Push", "Push"],
            Solution::build_array(vec![1, 2, 3], 3)
        );
        assert_eq!(vec!["Push", "Push"], Solution::build_array(vec![1, 2], 4));
        assert_eq!(
            vec!["Push", "Pop", "Push", "Pop", "Push", "Push"],
            Solution::build_array(vec![3, 4], 4)
        );
    }
}

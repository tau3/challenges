use std::{cell::RefCell, collections::HashMap};

pub struct Solution {}

impl Solution {
    pub fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
        let memo = RefCell::new(HashMap::new());
        let mut all_paths = Self::solve(destination[0], destination[1], &memo);
        all_paths.sort();
        all_paths[k as usize - 1].clone()
    }

    fn solve(
        row: i32,
        column: i32,
        memo: &RefCell<HashMap<(i32, i32), Vec<String>>>,
    ) -> Vec<String> {
        if row < 0 || column < 0 {
            return vec![];
        }

        if row == 0 && column == 1 {
            return vec!["H".to_string()];
        }
        if row == 1 && column == 0 {
            return vec!["V".to_string()];
        }

        if !memo.borrow().contains_key(&(row - 1, column)) {
            let paths = Self::solve(row - 1, column, memo);
            memo.borrow_mut().insert((row - 1, column), paths);
        }
        let binding = memo.borrow();
        let from_left = binding.get(&(row - 1, column)).unwrap();

        if !memo.borrow().contains_key(&(row, column - 1)) {
            let paths = Self::solve(row, column - 1, memo);
            memo.borrow_mut().insert((row, column - 1), paths);
        }
        let binding = memo.borrow();
        let from_top = binding.get(&(row, column - 1)).unwrap();

        let mut result = Vec::with_capacity(from_top.len() + from_left.len());
        for path in from_top.iter() {
            let mut path = path.clone();
            path.push('V');
            result.push(path);
        }
        for path in from_left.iter() {
            let mut path = path.clone();
            path.push('H');
            result.push(path);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("HHHVV", Solution::kth_smallest_path(vec![2, 3], 1));
    }
}

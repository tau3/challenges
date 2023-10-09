use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Solution {}

type Memo = Rc<RefCell<HashMap<(i32, i32), Vec<String>>>>;

impl Solution {
    pub fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
        let memo = Rc::new(RefCell::new(HashMap::new()));
        let mut all_paths = Self::solve((destination[0], destination[1]), Rc::clone(&memo));
        all_paths.sort();
        all_paths[k as usize - 1].clone()
    }

    fn solve((row, column): (i32, i32), memo: Memo) -> Vec<String> {
        if row < 0 || column < 0 {
            return vec![];
        }

        if row == 0 && column == 0 {
            return vec![];
        }
        if row == 0 && column == 1 {
            return vec!["H".to_string()];
        }
        if row == 1 && column == 0 {
            return vec!["V".to_string()];
        }

        let top = (row - 1, column);
        if !memo.borrow().contains_key(&top) {
            let paths = Self::solve(top, Rc::clone(&memo));
            memo.borrow_mut().insert(top, paths);
        }
        let left = (row, column - 1);
        if !memo.borrow().contains_key(&left) {
            let paths = Self::solve(left, Rc::clone(&memo));
            memo.borrow_mut().insert(left, paths);
        }

        let memo = memo.borrow();
        let from_top = memo.get(&top).unwrap();
        let from_left = memo.get(&left).unwrap();

        let mut result = Vec::with_capacity(from_left.len() + from_top.len());
        for path in from_left.iter() {
            let mut path = path.clone();
            path.push('H');
            result.push(path);
        }
        for path in from_top.iter() {
            let mut path = path.clone();
            path.push('V');
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
        assert_eq!("HHVHV", Solution::kth_smallest_path(vec![2, 3], 2));
        assert_eq!("HHVVH", Solution::kth_smallest_path(vec![2, 3], 3));
    }

    #[test]
    fn test2() {
        assert_eq!("HV", Solution::kth_smallest_path(vec![1, 1], 1));
    }
}

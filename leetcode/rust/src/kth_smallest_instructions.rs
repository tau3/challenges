use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Solution {}

impl Solution {
    pub fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
        let memo = Rc::new(RefCell::new(HashMap::new()));
        let mut all_paths = Self::solve((destination[0], destination[1]), memo);
        all_paths.sort();
        all_paths[k as usize - 1].clone()
    }

    fn solve(
        (row, column): (i32, i32),
        memo: Rc<RefCell<HashMap<(i32, i32), Vec<String>>>>,
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

        let left = (row - 1, column);
        let need_calc_from_left = !memo.borrow().contains_key(&left);
        if need_calc_from_left {
            let paths = Self::solve(left, Rc::clone(&memo));
            memo.borrow_mut().insert((row - 1, column), paths);
        }
        let top = (row, column - 1);
        let need_calc_from_top = !memo.borrow().contains_key(&top);
        if need_calc_from_top {
            let paths = Self::solve(top, Rc::clone(&memo));
            memo.borrow_mut().insert(top, paths);
        }

        let memo = memo.borrow();
        let from_left = memo.get(&left).unwrap();
        let from_top = memo.get(&top).unwrap();

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

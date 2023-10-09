use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
        let mut memo = HashMap::new();
        let mut all_paths = Self::solve(destination[0], destination[1], &mut memo);
        all_paths.sort();
        return all_paths[k as usize - 1];
    }

    fn solve(
        row: i32,
        column: i32,
        mut memo: &mut HashMap<(i32, i32), Vec<String>>,
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

        let from_top = memo
            .entry((row, column))
            .or_insert_with(|| Self::solve(row - 1, column, memo));
        let from_left = memo
            .entry((row, column))
            .or_insert_with(|| Self::solve(row, column - 1, memo));

        let mut result = Vec::with_capacity(from_top.len() + from_left.len());
        for path in from_top {
            let mut path = path.clone();
            path.push('V');
            result.push(path);
        }
        for path in from_left {
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

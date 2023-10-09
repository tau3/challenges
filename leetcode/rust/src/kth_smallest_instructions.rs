pub struct Solution {}

impl Solution {
    pub fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
        let mut all_paths = Self::solve(destination[0], destination[1]);
        all_paths.sort();
        return all_paths[k as usize - 1];
    }

    // TODO memo
    fn solve(row: i32, column: i32) -> Vec<String> {
        if row < 0 || column < 0 {
            return vec![];
        }

        if row == 0 && column == 1 {
            return vec!["H".to_string()];
        }
        if row == 1 && column == 0 {
            return vec!["V".to_string()];
        }

        let from_top = Self::solve(row - 1, column);
        let from_left = Self::solve(row, column - 1);

        let mut result: Vec<String> = from_top.iter().map(|x| x.push('V')).collect();

        from_left
            .iter()
            .map(|x| x.push('H'))
            .for_each(|x| result.push_back(x));

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

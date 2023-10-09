pub struct Solution {}

impl Solution {
    pub fn kth_smallest_path(destination: Vec<i32>, mut k: i32) -> String {
        let (row, column) = (destination[0] as usize, destination[1] as usize);
        let mut memo = vec![vec![0; column + 1]; row + 1];
        for r in (0..=row).rev() {
            for c in (0..=column).rev() {
                if r == row && c == column {
                    continue;
                } else if (r == row && c == column - 1) || (r == row - 1 && c == column) {
                    memo[r][c] = 1;
                } else if r == row {
                    memo[r][c] = memo[r][c + 1]
                } else if c == column {
                    memo[r][c] = memo[r + 1][c];
                } else {
                    memo[r][c] = memo[r + 1][c] + memo[r][c + 1];
                }
            }
        }

        let mut r = 0;
        let mut c = 0;
        let mut result = String::from("");
        while r < row && c < column {
            if k <= memo[r][c + 1] {
                c += 1;
                result.push('H');
            } else {
                k -= memo[r][c + 1];
                r += 1;
                result.push('V');
            }
        }

        for _ in 0..row - r {
            result.push('V');
        }
        for _ in 0..column - c {
            result.push('H');
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

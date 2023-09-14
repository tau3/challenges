pub struct Solution {}

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let mut total = 0;
        for x in 1..=n {
            total += x;
        }

        let mut right = 0;
        for x in (1..=n).rev() {
            right += x;
            if total - right + x == right {
                return x;
            }
        }
        -1
    }
}

#[cfg(test)]
mod p2485_tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::pivot_integer(8), 6);
        assert_eq!(Solution::pivot_integer(1), 1);
        assert_eq!(Solution::pivot_integer(4), -1);
    }
}

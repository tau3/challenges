struct Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(n as usize);
        if n % 2 != 0 {
            result.push(0);
        }
        for i in 0..(n / 2) {
            result.push(i + 1);
            result.push(-(i + 1));
        }
        result
    }
}

#[cfg(test)]
mod p1304_tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![0], Solution::sum_zero(1));
        assert_eq!(vec![0, 1, -1], Solution::sum_zero(3));
        assert_eq!(vec![0, 1, -1, 2, -2], Solution::sum_zero(5));
        assert_eq!(vec![1, -1, 2, -2], Solution::sum_zero(4));
    }
}
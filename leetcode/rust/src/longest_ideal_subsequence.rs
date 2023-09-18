pub struct Solution {}

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        if s.len() == 1 {
            return 1;
        }

        let letters: Vec<char> = s.chars().collect();
        let mut memo: Vec<Vec<u16>> = vec![vec![0; s.len()]; s.len()];

        let mut result = 0;
        for i in 0..s.len() {
            for j in i..s.len() {
                if j == 0 {
                    memo[i][j] = 1;
                    continue;
                }
		
                if Self::diff(letters[j], letters[j - 1]) <= k {
                    memo[i][j] = memo[i][j - 1] + 1;
                } else {
                    memo[i][j] = memo[i][j - 1];
                }

                if memo[i][j] > result {
                    result = memo[i][j];
                }
            }
        }
        result as i32
    }

    fn diff(l: char, r: char) -> i32 {
        (l as i32 - r as i32).abs()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4, Solution::longest_ideal_string("acfgbd".into(), 2));
        assert_eq!(4, Solution::longest_ideal_string("abcd".into(), 3));
        assert_eq!(1, Solution::longest_ideal_string("a".into(), 3));
    }

    #[test]
    fn test_broken() {
        assert_eq!(5, Solution::longest_ideal_string("jxhwaysa".into(), 14));
    }
}

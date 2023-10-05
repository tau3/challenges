pub struct Solution {}

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut counts = [0; 26];

        for c in text.chars() {
            counts[c as usize - 'a' as usize] += 1;
        }

        let result = [
            counts[1],      // b
            counts[0],      // a
            counts[11] / 2, // l
            counts[14] / 2, // o
            counts[13],     // n
        ];
        *result.iter().min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, Solution::max_number_of_balloons("nlaebolko".to_string()));
        assert_eq!(
            2,
            Solution::max_number_of_balloons("loonbalxballpoon".to_string())
        );
        assert_eq!(0, Solution::max_number_of_balloons("leetcode".to_string()));
    }

    #[test]
    fn test_balon() {
        assert_eq!(0, Solution::max_number_of_balloons("balon".to_string()));
    }
}

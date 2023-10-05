use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
	// TODO array
        let mut counts = HashMap::new();

        for c in text.chars() {
            let entry = counts.entry(c).or_insert(0);
            *entry += 1;
        }

        let result: Vec<u16> = vec![
            *counts.get(&'b').unwrap_or(&0),
            *counts.get(&'a').unwrap_or(&0),
            *counts.get(&'l').unwrap_or(&0),
            *counts.get(&'o').unwrap_or(&0),
            *counts.get(&'n').unwrap_or(&0),
        ];
        *result.iter().min().unwrap() as i32
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
    fn test_balon(){
        assert_eq!(0, Solution::max_number_of_balloons("balon".to_string()));
    }
}

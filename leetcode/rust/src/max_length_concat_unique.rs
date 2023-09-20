pub struct Solution {}

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let arr = arr
            .iter()
            .map(|word| Self::str_to_mask(word))
            .filter(|mask| mask.is_some())
            .map(|mask| mask.unwrap())
            .collect();
        Self::solve(&arr, 0, 0) as i32
    }

    fn str_to_mask(input: &str) -> Option<u32> {
        let mut result = 0;
        for c in input.chars() {
            let mask = 1 << (c as usize - 'a' as usize);
            if mask & result != 0 {
                return None;
            }
            result |= 1 << (c as usize - 'a' as usize);
        }
        Some(result)
    }

    fn solve(arr: &Vec<u32>, start: usize, memo: u32) -> u32 {
        if start == arr.len() {
            return memo.count_ones();
        }
        let word = arr[start];
        let without_current_word = Self::solve(arr, start + 1, memo);
        if word & memo != 0 {
            return without_current_word;
        }
        let with_current_word = Self::solve(arr, start + 1, memo | word);
        std::cmp::max(with_current_word, without_current_word)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn to_strings(input: &[&str])->Vec<String>{
        input.iter().map(|word| word.to_string()).collect()
    }

    #[test]
    fn test() {
        let arr = ["un", "iq", "ue"];
        assert_eq!(4, Solution::max_length(to_strings(&arr)));
    }

    #[test]
    fn test2() {
        let arr = ["cha", "r", "act", "ers"];
        assert_eq!(6, Solution::max_length(to_strings(&arr)));
    }

    #[test]
    fn test3() {
        let arr = ["abcdefghijklmnopqrstuvwxyz"];
        assert_eq!(26, Solution::max_length(to_strings(&arr)));
    }

    #[test]
    fn test_duplicate_chars_in_input() {
        let arr = ["aa", "bb"];
        assert_eq!(0, Solution::max_length(to_strings(&arr)));
    }
}

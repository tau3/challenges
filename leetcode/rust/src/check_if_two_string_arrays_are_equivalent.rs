struct Solution {}

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        concat(word1) == concat(word2)
    }
}

fn concat(words: Vec<String>) -> String {
    let mut result = String::new();
    words.iter().for_each(|w| result.push_str(w));
    result
}

#[cfg(test)]
mod test {
    use crate::check_if_two_string_arrays_are_equivalent::Solution;

    #[test]
    fn test_case1() {
        assert!(Solution::array_strings_are_equal(
            vec!["a".into(), "bc".into()],
            vec!["ab".into(), "c".into()],
        ));
    }

    #[test]
    fn test_case2() {
        assert!(!Solution::array_strings_are_equal(
            vec!["a".into(), "cb".into()],
            vec!["ab".into(), "c".into()],
        ));
    }

    #[test]
    fn test_case3() {
        assert!(Solution::array_strings_are_equal(
            vec!["abc".into(), "d".into(), "defg".into()],
            vec!["abcddefg".into()],
        ));
    }
}

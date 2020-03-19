pub struct Solution {}

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut opens = Vec::new();
        let mut closes = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                opens.push(i);
            }
            if c == ')' {
                if let None = opens.pop() {
                    closes.push(i);
                }
            }
        }

        let mut result = Vec::new();
        for (i, c) in s.chars().enumerate() {
            if !opens.contains(&i) && !(closes.contains(&i)) {
                result.push(c);
            }
        }

        result.iter().collect()
    }
}

mod p1249_tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn simple_test() {
        let mut cases = HashMap::new();
        cases.insert("lee(t(c)o)de)", "lee(t(c)o)de");
        cases.insert("", "");
        cases.insert("))((", "");
        cases.insert("(a(b(c)d)", "a(b(c)d)");


        for (input, expected) in cases.iter() {
            let actual = Solution::min_remove_to_make_valid(input.to_string());
            assert_eq!(*expected, actual);
        }
    }
}
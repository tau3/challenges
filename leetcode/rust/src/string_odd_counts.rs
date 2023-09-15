pub struct Solution {}

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        if n == 1 {
            return 'a'.into();
        }
        if n % 2 != 0 {
            let mut result = Solution::generate_the_string(n - 1);
            result.push('c');
            result
        } else {
            let mut result = String::with_capacity(n as usize + 1);
            for _ in 0..n - 1 {
                result.push('a');
            }
            result.push('b');
            result
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test() {
        let odd = Solution::generate_the_string(19);
        assert!(check(&odd));
        assert_eq!(odd.len(), 19);

        let long_even = Solution::generate_the_string(500);
        assert!(check(&long_even));
        assert_eq!(long_even.len(), 500);

        let one = Solution::generate_the_string(1);
        assert!(check(&one));
        assert_eq!(one.len(), 1);
    }

    fn check(input: &String) -> bool {
        let mut counter = HashMap::new();
        for letter in input.chars() {
            counter
                .entry(letter)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        for x in counter.values() {
            if x % 2 == 0 {
                return false;
            }
        }
        return true;
    }
}

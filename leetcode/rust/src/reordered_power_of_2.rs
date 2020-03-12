pub struct Solution;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let key = Solution::i32_to_key(n);

        let mut i = 1;
        while i < 1_000_000_000 {
            if Solution::i32_to_key(i) == key {
                return true;
            }
            i *= 2;
        }
        false
    }

    fn i32_to_key(n: i32) -> String {
        let mut result = Vec::new();
        for c in n.to_string().chars() {
            result.push(c);
        }
        result.sort();
        result.iter().collect()
    }
}
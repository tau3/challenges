pub struct Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let mut result = 0;
        let mut current = n;
        for required in 1..n {
            if current < required {
                break;
            }
            current -= required;
            result += 1;
        }
        result
    }
}
pub struct Solution {}

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let keys_pressed: Vec<char> = keys_pressed.chars().collect();
        let mut result = keys_pressed[0];
        let mut time = release_times[0];

        for i in 1..keys_pressed.len() {
            let current_time = release_times[i] - release_times[i - 1];
            if current_time > time {
                result = keys_pressed[i];
                time = current_time;
            } else if current_time == time && keys_pressed[i] > result {
                result = keys_pressed[i];
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            'c',
            Solution::slowest_key(vec![9, 29, 49, 50], "cbcd".into())
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            'a',
            Solution::slowest_key(vec![12, 23, 36, 46, 62], "spuda".into())
        );
    }
}

pub struct Solution {}

impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        let mut diff = Self::to_minutes(&correct) - Self::to_minutes(&current);

        let mut result = diff / 60;
        diff -= result * 60;
        if diff == 0 {
            return result as i32;
        }

        let mut steps = diff / 15;
        result += steps;
        diff -= steps * 15;
        if diff == 0 {
            return result as i32;
        }

        steps = diff / 5;
        result += steps;
        diff -= steps * 5;
        if diff == 0 {
            return result as i32;
        }

        result += diff;
        result as i32
    }

    fn to_minutes(time: &str) -> u16 {
        let hours: u16 = time[0..2].parse().unwrap();
        let minutes: u16 = time[3..5].parse().unwrap();
        hours * 60 + minutes
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, Solution::convert_time("02:30".into(), "04:35".into()));
    }

    #[test]
    fn test_one() {
        assert_eq!(1, Solution::convert_time("11:00".into(), "11:01".into()));
    }
}

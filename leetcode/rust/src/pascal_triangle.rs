struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![1]];
        for _ in 1..num_rows {
            let line = line(result.last().unwrap());
            result.push(line);
        }
        result
    }
}

fn line(prev: &[i32]) -> Vec<i32> {
    let mut result = vec![1];
    for i in 1..prev.len() {
        result.push(prev[i] + prev[i - 1])
    }
    result.push(1);
    result
}

#[cfg(test)]
mod tests {
    use crate::pascal_triangle::Solution;

    #[test]
    fn test_5() {
        let expected = vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1], vec![1, 4, 6, 4, 1]];
        assert_eq!(expected, Solution::generate(5));
    }

    #[test]
    fn test_1() {
        let expected = vec![vec![1]];
        assert_eq!(expected, Solution::generate(1));
    }
}
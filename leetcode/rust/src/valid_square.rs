struct Solution {}

impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let mut points = vec![p1, p2, p3, p4];
        points.sort_by(|a, b| order(a, b));

        let distance_0_1 = distance(&points[0], &points[1]);
        let distance_1_3 = distance(&points[1], &points[3]);
        let distance_3_2 = distance(&points[3], &points[2]);
        let are_equal_sides = distance_0_1 == distance_1_3
            && distance_1_3 == distance_3_2
            && distance_3_2 == distance(&points[2], &points[0]);

        let are_equal_diagonals =
            distance(&points[0], &points[3]) == distance(&points[1], &points[2]);

        are_equal_sides && are_equal_diagonals && distance_0_1 != 0
    }
}

fn order(p1: &[i32], p2: &[i32]) -> std::cmp::Ordering {
    match p1[0].cmp(&p2[0]) {
        std::cmp::Ordering::Equal => p1[1].cmp(&p2[1]),
        other => other,
    }
}

fn distance(p1: &[i32], p2: &[i32]) -> i32 {
    (p1[0] - p2[0]) * (p1[0] - p2[0]) + (p1[1] - p2[1]) * (p1[1] - p2[1])
}

#[cfg(test)]
mod test {
    use crate::valid_square::Solution;

    #[test]
    fn test_1() {
        assert!(Solution::valid_square(
            vec![0, 0],
            vec![1, 1],
            vec![1, 0],
            vec![0, 1],
        ))
    }

    #[test]
    fn test_2() {
        assert!(!Solution::valid_square(
            vec![0, 0],
            vec![1, 1],
            vec![1, 0],
            vec![0, 12],
        ))
    }

    #[test]
    fn test_3() {
        assert!(Solution::valid_square(
            vec![1, 0],
            vec![-1, 0],
            vec![0, 1],
            vec![0, -1],
        ))
    }
}

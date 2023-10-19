use std::collections::HashMap;

pub struct DetectSquares {
    cache: HashMap<i32, HashMap<i32, i32>>,
}

impl Default for DetectSquares {
    fn default() -> Self {
        Self::new()
    }
}

impl DetectSquares {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn add(&mut self, point: Vec<i32>) {
        let (x, y) = (point[0], point[1]);
        let y_to_count: &mut HashMap<i32, i32> = self.cache.entry(x).or_default();
        let val = y_to_count.entry(y).or_default();
        *val += 1;
    }

    pub fn count(&self, point: Vec<i32>) -> i32 {
        let (x1, y1) = (point[0], point[1]);
        let mut result = 0;
        if let Some(same_column) = self.cache.get(&x1) {
            for (y2, count) in same_column {
                let l = (y2 - y1).abs();

                let x_left = x1 - l;
                if let Some(left) = self.cache.get(&x_left) {
                    if let (Some(y1l), Some(y2l)) = (left.get(&y1), left.get(y2)) {
                        result += count * y1l * y2l;
                    }
                }

                let x_right = x1 + l;
                if let Some(right) = self.cache.get(&x_right) {
                    if let (Some(y1r), Some(y2r)) = (right.get(&y1), right.get(y2)) {
                        result += count * y1r * y2r;
                    }
                }
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
        let mut detect_squares = DetectSquares::new();
        detect_squares.add(vec![3, 10]);
        detect_squares.add(vec![11, 2]);
        detect_squares.add(vec![3, 2]);
        assert_eq!(1, detect_squares.count(vec![11, 10]));
        assert_eq!(0, detect_squares.count(vec![14, 8]));
        detect_squares.add(vec![11, 2]);
        assert_eq!(2, detect_squares.count(vec![11, 10]));
    }
}

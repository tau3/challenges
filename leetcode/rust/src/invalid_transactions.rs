use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let txns: Vec<_> = transactions
            .iter()
            .map(|line| Solution::parse_txn(line))
            .collect();
        let mut result = HashSet::new();
        for i in 0..txns.len() {
            let (current_name, current_time, current_amount, current_city) = txns[i];
            if current_amount > 1000 {
                result.insert(i);
            }
            for (j, other) in txns.iter().enumerate().skip(i + 1) {
                let (other_name, other_time, _, other_city) = other;
                if other_name == &current_name
                    && (other_time - current_time).abs() <= 60
                    && (other_city != &current_city)
                {
                    result.insert(i);
                    result.insert(j);
                }
            }
        }

        result
            .into_iter()
            .map(|i| &transactions[i])
            .cloned()
            .collect()
    }

    fn parse_txn(line: &str) -> (&str, i16, u16, &str) {
        let mut tokens = line.split(',');
        let name = tokens.next().unwrap();
        let time = tokens.next().unwrap().parse().unwrap();
        let amount = tokens.next().unwrap().parse().unwrap();
        let city = tokens.next().unwrap();
        (name, time, amount, city)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq_any_order(
            vec!["alice,50,1200,mtv".to_string()],
            Solution::invalid_transactions(vec![
                "alice,20,800,mtv".to_string(),
                "alice,50,1200,mtv".to_string(),
            ]),
        );
    }

    #[test]
    fn test2() {
        assert_eq_any_order(
            vec![
                "alice,20,800,mtv".to_string(),
                "alice,50,100,beijing".to_string(),
            ],
            Solution::invalid_transactions(vec![
                "alice,50,100,beijing".to_string(),
                "alice,20,800,mtv".to_string(),
            ]),
        );
    }

    #[test]
    fn test3() {
        assert_eq_any_order(
            vec!["alice,50,1200,mtv".to_string()],
            Solution::invalid_transactions(vec![
                "alice,20,800,mtv".to_string(),
                "alice,50,1200,mtv".to_string(),
            ]),
        );
    }

    #[test]
    fn test4() {
        assert_eq_any_order(
            vec![
                "alice,20,800,mtv".to_string(),
                "alice,50,100,mtv".to_string(),
                "alice,51,100,frankfurt".to_string(),
            ],
            Solution::invalid_transactions(vec![
                "alice,20,800,mtv".to_string(),
                "alice,50,100,mtv".to_string(),
                "alice,51,100,frankfurt".to_string(),
            ]),
        );
    }

    #[test]
    fn test5() {
        assert_eq_any_order(
            vec![
                "alice,20,1220,mtv".to_string(),
                "alice,20,1220,mtv".to_string(),
            ],
            Solution::invalid_transactions(vec![
                "alice,20,1220,mtv".to_string(),
                "alice,20,1220,mtv".to_string(),
            ]),
        );
    }

    fn assert_eq_any_order(mut left: Vec<String>, mut right: Vec<String>) {
        if left.len() != right.len() {
            panic!("different len: {:?} and {:?}", left, right);
        }

        left.sort();
        right.sort();

        let mut result = true;
        for i in 0..left.len() {
            if left[i] != right[i] {
                result = false;
                break;
            }
        }

        if !result {
            panic!("{:?} != {:?}", left, right);
        }
    }
}

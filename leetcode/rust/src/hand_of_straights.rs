pub struct Solution {}

impl Solution {
    pub fn is_n_straight_hand(mut hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() % group_size as usize != 0 {
            return false;
        }

        hand.sort();
        let group_size = group_size as usize;
        let mut groups: Vec<Vec<i32>> = vec![vec![]; hand.len() / group_size];

        for card in hand {
            let mut is_pushed = false;
            for group in groups.iter_mut() {
                if group.is_empty() {
                    group.push(card);
                    is_pushed = true;
                    break;
                }

                if group.len() < group_size && group.last().unwrap() + 1 == card {
                    group.push(card);
                    is_pushed = true;
                    break;
                }
            }
            if !is_pushed {
                return false;
            }
        }

        groups.iter().all(|group| group.len() == group_size)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::is_n_straight_hand(
            vec![1, 2, 3, 6, 2, 3, 4, 7, 8],
            3
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5], 4));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_n_straight_hand(vec![8, 10, 12], 3));
    }

    #[test]
    fn test4() {
        assert!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5, 6], 2));
    }
}

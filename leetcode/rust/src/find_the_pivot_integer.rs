pub struct Solution {}

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        for x in 1..=n {
	    let mut before = 0;
	    for i in 1..=x {
		before += i;
	    }
	    let mut after = 0;
	    for i in x..=n {
		after += i;
	    }

	    if after == before {
		return x;
	    }
	}
	-1
    }
}


#[cfg(test)]
mod p2485_tests {
    use super::*;

    #[test]
    fn test(){
	assert_eq!(Solution::pivot_integer(8), 6);
	assert_eq!(Solution::pivot_integer(1), 1);
	assert_eq!(Solution::pivot_integer(4), -1);
    }
}


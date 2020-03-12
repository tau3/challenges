mod arranging_coins;
mod reordered_power_of_2;

fn main() {
    let inputs = [1, 10, 16, 24];
    let expected = [true, false, true, false];
    for (i, expected) in inputs.iter().zip(expected.iter()) {
        let actual = reordered_power_of_2::Solution::reordered_power_of2(*i);
        if actual != *expected {
            println!("fail for {}", i);
        } else {
            println!("success for {}", i)
        }
    }
}

mod arranging_coins;

fn main() {
    let inputs = [8, 0, 1, 2];
    for n in inputs.iter() {
        let actual = arranging_coins::Solution::arrange_coins(*n);
        println!("{} -> {}", n, actual);
    }
}

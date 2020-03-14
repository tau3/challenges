use std::convert::TryFrom;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let t = input.parse().unwrap();
    println!("{}", strange_counter(t));
}

fn strange_counter(t: u32) -> u32 {
    let mut val = 4;
    let mut cycle = 1;
    let mut initial = 3;
    for i in 0..t {
        val -= 1;
        if val == 0 {
            cycle += 1;
            val = 2 * initial;
            initial *= 2;
        }
    }
    return val;
}
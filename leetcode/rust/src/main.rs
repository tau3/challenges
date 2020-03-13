use std::convert::TryFrom;
use std::io;

fn main() {
    let n = read_u32();
    let k = usize::try_from(read_u32()).unwrap();

    let mut arr: Vec<u32> = Vec::new();
    for _i in 0..n {
        arr.push(read_u32());
    }

    println!("{}", max_min(k, &arr));
}

fn read_u32() -> u32 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let result = line.trim().parse().unwrap();
    return result;
}

fn max_min(k: usize, arr: &Vec<u32>) -> u32 {
    let mut sorted = arr.clone();
    sorted.sort();

    let mut diffs = Vec::new();
    for i in 0..sorted.len() - k {
        diffs.push(sorted[i + k - 1] - sorted[i]);
    }
    let result = diffs.iter().min();
    return result.unwrap().to_owned();
}
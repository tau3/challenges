use std::convert::TryFrom;
use std::io;

fn main_max_min() {
    let n = read_u32();
    let k = usize::try_from(read_u32()).unwrap();

    let mut arr: Vec<i32> = Vec::new();
    for _i in 0..n {
        arr.push(read_u32());
    }

    println!("{}", max_min(k, arr));
}

fn read_u32() -> i32 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let line = line.trim();
    line.parse().unwrap()
}

fn max_min(k: usize, mut arr: Vec<i32>) -> i32 {
    arr.sort();

    let mut result = i32::max_value();
    for i in 0..arr.len() - k + 1 {
        let candidate = arr[i + k - 1] - arr[i];
        if candidate < result {
            result = candidate;
        }
    }
    result
}
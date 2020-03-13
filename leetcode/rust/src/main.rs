use std::io;

fn main() {
    let arr = read_input_arr();
    let sorted = sorted_copy(&arr);

    if sorted == arr {
        println!("yes");
        return;
    }

    let unsorted_indexes = get_unsorted_indexes(&arr, &sorted);

    if unsorted_indexes.len() == 2 {
        println!("yes");
        println!("{} {}", unsorted_indexes[0] + 1, unsorted_indexes[1] + 1);
        return;
    } else {
        let l = unsorted_indexes[0];
        let r = unsorted_indexes.last().unwrap().to_owned();

        let segment = slice_vector(&arr, l, r);

        let reverse_sorted = reverse_sort(&segment);
        if segment == reverse_sorted {
            println!("yes");
            println!("reverse {} {}", l, r);
        } else {
            println!("no")
        }
    }
}

fn slice_vector(v: &Vec<u16>, l: usize, r: usize) -> Vec<u16> {
    let mut result = Vec::new();
    result.copy_from_slice(&v[l..r]);
    return result;
}

fn get_unsorted_indexes(arr: &Vec<u16>, sorted: &Vec<u16>) -> Vec<usize> {
    let mut unsorted_indexes: Vec<usize> = Vec::new();
    for (i, val) in arr.iter().enumerate() {
        if *val != sorted[i] {
            unsorted_indexes.push(i);
        }
    }
    return unsorted_indexes;
}

fn reverse_sort(v: &Vec<u16>) -> Vec<u16> {
    let mut result = sorted_copy(v);
    result.reverse();
    return result;
}

fn sorted_copy(v: &Vec<u16>) -> Vec<u16> {
    let mut result = v.clone();
    result.sort();
    return result;
}

fn read_input_arr() -> Vec<u16> {
    let mut line = String::new();
    let std_in = io::stdin();
    std_in.read_line(&mut line).unwrap();
    std_in.read_line(&mut line).unwrap();

    let arr: Vec<u16> = line.split_whitespace()
        .map(|token| { token.parse().unwrap() })
        .collect();
    return arr;
}
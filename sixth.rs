use std::io::stdin;

fn main() {
    let mut inp1: String = String::new();
    stdin().read_line(&mut inp1).expect("Problem with reading 1st input");
    let mut inp2: String = String::new();
    stdin().read_line(&mut inp2).expect("Problem with reading 2nd input");
    let mut arr: Vec<i32> = inp2.trim().split(' ')
            .map(|s| s.parse()
            .expect("can't turn string into numbers")).collect::<Vec<i32>>();
    arr = merge_sort(&arr);
    for elem in arr {
        print!("{elem} ");
    }
}

fn merge_sort(vec: &Vec<i32>) -> Vec<i32> {
    if vec.len() == 1 {
        vec.to_vec()
    } else {
        let size = vec.len() / 2;
        let left = merge_sort(&vec[0..size].to_vec());
        let right = merge_sort(&vec[size..].to_vec());
        let merged = merge(&left, &right);
        merged
    }
}

fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut merged: Vec<i32> = Vec::new();

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i += 1;
        }
    }
    if j < right.len() {
        while j < right.len() {
            merged.push(right[j]);
            j += 1;
        }
    }
    merged
}

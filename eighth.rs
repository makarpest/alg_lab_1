use std::io::stdin;

fn merge_sort(vec: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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

fn merge(left: &Vec<Vec<i32>>, right: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut i = 0;
    let mut j = 0;
    let mut merged: Vec<Vec<i32>> = Vec::new();

    while i < left.len() && j < right.len() {
        if left[i][0] < right[j][0] {
            merged.push(left[i].to_vec());
            i += 1;
        } else {
            merged.push(right[j].to_vec());
            j += 1;
        }
    }

    if i < left.len() {
        while i < left.len() {
            merged.push(left[i].to_vec());
            i += 1;
        }
    }
    if j < right.len() {
        while j < right.len() {
            merged.push(right[j].to_vec());
            j += 1;
        }
    }
    merged
}

fn main() {
    let mut s1: String = String::new();
    stdin().read_line(&mut s1).unwrap();
    let n1: i32 = s1.trim().parse().unwrap();

    let mut lens: Vec<Vec<i32>> = Vec::new();
    for _i in 0..n1 {
        {
            let mut s2: String = String::new();
            stdin().read_line(&mut s2).unwrap();
            let v: Vec<i32> = s2.trim().split_whitespace()
                .map(|s| s.parse().unwrap()).collect();
            lens.push(v);
        }
    }

    let lens = merge_sort(&mut lens);

    let mut count: i32 = lens[0][1] - lens[0][0] + 1;
    let mut cur_max: i32 = lens[0][1];
    for j in 1..lens.len() {
        if lens[j][0] > cur_max {
            count += lens[j][1] - lens[j][0] + 1;
            cur_max = lens[j][1];
        } else {
            if lens[j][1] > cur_max {
                count += lens[j][1] - cur_max;
                cur_max = lens[j][1];
            }
        }
    }
    
    println!("{count}");
}

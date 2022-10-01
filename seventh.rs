use std::io::stdin;

fn main() {
    let mut inp1: String = String::new();
    stdin().read_line(&mut inp1).expect("Problem with reading 1st input");

    let mut inp2: String = String::new();
    stdin().read_line(&mut inp2).expect("Problem with reading 2nd input");
    let mut arr1: Vec<i32> = inp2.trim().split(' ')
            .map(|s| s.parse()
            .expect("can't turn string into numbers")).collect::<Vec<i32>>();

    let mut inp3: String = String::new();
    stdin().read_line(&mut inp3).expect("Problem with reading 1st input"); 
    
    let mut inp4: String = String::new();
    stdin().read_line(&mut inp4).expect("Problem with reading 2nd input");
    let mut arr2: Vec<i32> = inp4.trim().split(' ')
        .map(|s| s.parse()
            .expect("can't turn string into numbers")).collect::<Vec<i32>>();

    arr1 = merge_sort(&arr1);
    arr2 = merge_sort(&arr2);

    let answer = compare_arrs(arr1, arr2);
    if answer { println!("YES"); } else { println!("NO") };
}

fn compare_arrs(arr1: Vec<i32>, arr2: Vec<i32>) -> bool {
    let mut ans = true;
    let mut v1: Vec<i32> = Vec::new();
    v1.push(arr1[0]);
    let mut v2: Vec<i32> = Vec::new();
    v2.push(arr2[0]);
    if arr1.len() != 1 {
        for i in 1..arr1.len() {
            if arr1[i] != arr1[i - 1] {
                v1.push(arr1[i]);
            }
        }
    }
    if arr2.len() != 1 {
        for j in 1..arr2.len() {
            if arr2[j] != arr2[j - 1] {
                v2.push(arr2[j]);
            }
        }
    }
    if v1 != v2 { ans = false };
    ans
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

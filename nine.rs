use std::io::stdin;

fn qwick_sort(mut arr: &mut Vec<i32>, left: i32, right: i32) {
    if left < right {
        let pivot: i32 = partision(&mut arr, left, right);
        qwick_sort(&mut arr, left, pivot - 1);
        qwick_sort(&mut arr, pivot + 1, right);
    }
}

fn partision(arr: &mut Vec<i32>, left: i32, right: i32) -> i32 {
    let pivot: i32 = arr[right as usize];
    let mut cur_ind: i32 = left - 1;
    for i in left..right {
        if arr[i as usize] <= pivot {
            cur_ind += 1;
            arr.swap(i as usize, cur_ind as usize);
        }
    }
    arr.swap(right as usize, (cur_ind + 1) as usize);

    cur_ind + 1
}

fn print_arr(arr: Vec<i32>) {
    for elem in arr {
        print!("{elem} ");
    }
}

fn main() {
    let mut inp1 = String::new();
    stdin().read_line(&mut inp1).unwrap();
    let num: i32 = inp1.trim().parse().unwrap();
    let mut inp2 = String::new();
    stdin().read_line(&mut inp2).unwrap();
    let mut arr = inp2
        .trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
    qwick_sort(&mut arr, 0, num - 1);
    print_arr(arr);
}

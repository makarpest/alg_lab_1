use std::io::stdin;

fn main() {
    let mut inp1: String = String::new();
    stdin().read_line(&mut inp1).expect("error!");
    let mut arr: Vec<[i32; 2]> = Vec::new();
    for _i in 0..inp1.trim().parse().unwrap() {
        let mut inp2: String = String::new();
        stdin().read_line(&mut inp2).expect("error!");
        let part: [i32; 2] = [inp2.trim().split(' ').collect::<Vec<&str>>()[0].parse().unwrap(),
            inp2.trim().split(' ').collect::<Vec<&str>>()[1].parse().unwrap()];
        arr.push(part);
    }

    arr = insertion_sort(arr);
    arr = in_sort(arr);

    for j in (0..arr.len()).rev() {
        println!("{} {}", arr[j][0], arr[j][1]);
    }
}

fn insertion_sort(mut arr: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
    for i in 1..arr.len() {
        let key = arr[i][1];
        let key_arr = arr[i];
        let mut j = i - 1;
        while j > 0 && arr[j][1] > key {
            arr[j + 1] = arr[j];
            j -= 1;
        }
        arr[j + 1] = key_arr;

        if arr[0][1] > arr[1][1] {
            arr.swap(0, 1);
        }

    }

    arr
}
fn in_sort(mut arr: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
    for i in 1..arr.len() {
        if arr[i][1] == arr[i - 1][1] {
            if arr[i][0] > arr[i - 1][0] {
                let mut j = i - 1;
                let key_el = arr[i];
                let key = arr[i][0];
                while key > arr[j][0] && key_el[1] == arr[j][1] && j > 0 {
                    arr[j + 1] = arr[j];
                    j -= 1;
                }
                arr[j + 1] = key_el;
            }
        }
        if arr[0][1] == arr[1][1] && arr[0][0] < arr[1][0] { // а это надо?
            arr.swap(0, 1);
        }
    }

    arr
}

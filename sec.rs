use std::io::stdin;

fn main() {
    // входныке данные
    let mut inp = String::new();
    stdin().read_line(&mut inp).expect(" error ");
    let mut input = String::new();
    stdin().read_line(&mut input).expect(" error ");
    let mut vec: Vec<i32> = input
        .trim().split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    // алгоритм
    for i in 1..vec.len() {
        let mut gr = i - 1;
        let key = vec[i];
        while gr > 0 && vec[gr] > key {
            vec[gr + 1] = vec[gr];
            gr -= 1;
        }
        vec[gr + 1] = key;
        if gr == 0 && vec[0] > vec[1] {
            let helper = vec[1];
            vec[1] = vec[0];
            vec[0] = helper;
        }
    }
    for elem in vec {
        print!("{elem} ");
    }
}

use std::io::stdin;

fn main() {
    let mut inp1 = String::new();
    stdin().read_line(&mut inp1).unwrap();
    let num: i32 = inp1.trim().parse().unwrap();
    let mut arr: Vec<i32> = Vec::new();

    for i in 1..num + 1 {
        arr.push(i);
    };

    for i in 2..arr.len() {
        arr.swap(i, i / 2);
    }

    for elem in arr {
        print!("{elem} ");
    }
}

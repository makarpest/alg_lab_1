use std::io::stdin;

fn main() {
    let mut inp = String::new();
    stdin().read_line(&mut inp).expect(" error ");
    let mut input = String::new();
    stdin().read_line(&mut input).expect(" error ");
    let mut vec: Vec<i32> = input
        .trim().split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    vec.push(0);
    let mut inp2 = String::new();
    stdin().read_line(&mut inp2).expect(" error ");
    let cor = inp2
        .trim().split(' ')
        .map(|s| s.parse().unwrap()).collect::<Vec<i32>>();

    let index = cor[1] - 1;
    let num = cor[0];
    let mut point: usize = vec.len() - 1;
    loop {
        if point == index as usize {
            break
        };
        vec[point] = vec[point - 1];
        point -= 1;
    }
    vec[index as usize] = num;
    for elem in vec {
        print!("{elem} ");
    }
}

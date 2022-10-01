use std::io::stdin;

fn main() {
    // ввод данных
    let mut inp1: String = String::new();
    stdin().read_line(&mut inp1).expect("error!");
    let n: i32 = inp1.trim().split(' ').collect::<Vec<&str>>()[0].parse().unwrap();
    //let m: i32 = inp1.trim().split(' ').collect::<Vec<&str>>()[1].parse().unwrap();
    let mut plots: Vec<i32> = Vec::new();
    for _i in 0..n {
        let mut inp2: String = String::new();
        stdin().read_line(&mut inp2).expect("error!");
        for elem in inp2
            .trim().split(' ').map(|s| s.parse().unwrap()) {
            plots.push(elem);
        }
    }
    let mut inp3: String = String::new();
    stdin().read_line(&mut inp3).expect("error!");
    let mut people: String = String::new();
    stdin().read_line(&mut people).expect("error!");
    let mut people: Vec<i32> = people
        .trim().split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    // алгоритм
    for k in 1..plots.len() {
        let mut gr = k - 1;
        let key = plots[k];
        while gr > 0 && plots[gr] > key {
            plots[gr + 1] = plots[gr];
            gr -= 1;
        }
        plots[gr + 1] = key;
        if gr == 0 && plots[0] > plots[1] {
            let helper = plots[1];
            plots[1] = plots[0];
            plots[0] = helper;
        }
    }
    for g in 1..people.len() {
        let mut gr = g - 1;
        let key = people[g];
        while gr > 0 && people[gr] > key {
            people[gr + 1] = people[gr];
            gr -= 1;
        }
        people[gr + 1] = key;
        if gr == 0 && people[0] > people[1] {
            let helper = people[1];
            people[1] = people[0];
            people[0] = helper;
        }
    }
    let mut counter: usize = 0;
    let mut pointer_plots: usize = 0;
    let mut pointer_people: usize = 0;
    loop {
        if pointer_plots == plots.len() || pointer_people == people.len() { break };
        if people[pointer_people] <= plots[pointer_plots] {
            counter += 1;
            pointer_plots += 1;
            pointer_people += 1;
        } else { pointer_plots += 1; }
    }
    println!("{counter}");
}

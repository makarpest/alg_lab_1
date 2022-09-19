fn main() {
    let mut strok = String::new();
    std::io::stdin().read_line(&mut strok).expect("err");
    let out = strok.trim().chars().rev().nth(0).unwrap()
        .to_string().parse::<i32>().unwrap();
    print!("{}", out);
}

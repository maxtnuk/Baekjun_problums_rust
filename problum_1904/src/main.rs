use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("wrong");
    let number = input.trim().parse::<u32>().unwrap();
    let (i, j, t) = (&mut 1, &mut 0, &mut 0);
    for _ in 0..number {
        *t = *i;
        *i = (*i + *j) % 15746;
        *j = *t;
    }
    println!("{}", i);
}

use std::io;
enum Cell {
    No,
    Been(u64),
}
fn func(memory: &mut Vec<Cell>, num: u64) -> u64 {
    if let Cell::Been(temp) = memory[num as usize] {
        temp
    } else {
        let mut result = 0u64;
        let mut count = 0u64;
        while count < num {
            let (num1, num2) = (func(memory, count), func(memory, num - count - 1));
            result += num1 * num2;
            {
                let t_m1 = memory.get_mut(count as usize).unwrap();
                *t_m1 = Cell::Been(num1);
            }
            {
                let t_m2 = memory.get_mut((num - count - 1) as usize).unwrap();
                *t_m2 = Cell::Been(num2);
            }
            count += 1;
        }
        result
    }
}
fn main() {
    let mut target = String::new();
    io::stdin().read_line(&mut target).expect("error");
    let target: u64 = target.trim().parse().unwrap();
    let mut memory: Vec<Cell> = Vec::with_capacity(36);
    memory.push(Cell::Been(1));
    let mut count = 0;
    while count < 35 {
        memory.push(Cell::No);
        count += 1;
    }
    println!("{}", func(&mut memory, target));
}

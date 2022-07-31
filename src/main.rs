use rand::Rng;
use std::{cmp::Ordering, io, env};

fn main() {
    let aaa :Vec<String> = env::args().collect();
    println!("Hello, world，{:?}", aaa);

    let raw = String::from("Simple");
    println!("String is {}", raw);

    let rand_num = rand::thread_rng().gen_range(1, 100);
    println!("ran_num:{}", rand_num);

    loop {
        let mut guess = String::new();
        println!("Input A Number:");
        let data: usize = match io::stdin().read_line(&mut guess) {
            Ok(abc) => abc,
            Err(_) => continue,
        };

        println!("Your Input Size is:{}", data);

        let num: u32 = match guess.trim().parse() {
            Ok(aaa) => aaa,
            Err(msg) => {
                println!("Parse Error: {}", msg);
                continue;
            }
        };

        match num.cmp(&rand_num) {
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("OK");
                break;
            }
            Ordering::Less => println!("Less"),
        }
    }
}

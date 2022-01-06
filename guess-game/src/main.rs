use std::io;
use std::cmp::Ordering;
use rand::Rng;

const RNG_START: u32 = 1;
const RNG_END: u32 = 100;

fn main() {
    let target = rand::thread_rng()
        .gen_range(RNG_START..RNG_END);

    loop {
        let mut guess = String::new();

        println!("pick a number ({} - {}):", RNG_START, RNG_END);
        io::stdin()
            .read_line(&mut guess)
            .expect("could not read input.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("not a valid number, try again.");
                continue
            }
        };

        match guess.cmp(&target) {
            Ordering::Less => println!("too small."),
            Ordering::Greater => println!("too big."),
            Ordering::Equal => {
                println!("you win.");
                break;
            }
        }
    }
}

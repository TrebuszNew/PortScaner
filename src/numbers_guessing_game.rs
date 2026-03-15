use rand::RngExt;
use colored::Colorize;
fn main() {
    let haslo = rand::rng().random_range(0..500);
    for _ in 0..10 {
        let input = input();
        if input == haslo {
            println!("Nice!");
            return;
        } else if input > haslo {
            println!("za duże!");
        } else if input < haslo {
            println!("za małe!");
        }
    }
    println!("Przegrałeś :/\nHasłem było: {}", haslo);
}

fn input() -> i32 {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<i32>() {
            Ok(value) => {
                break value;
            }
            Err(_) => {
                println!("{}", "to nie jest liczba!".color(colored::Color::Red));
                continue
            }
        }
    }
}
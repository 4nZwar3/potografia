mod functions;
mod num_tools;
use {
    std::{
        io::{self, Write},
        time::Instant
    }, 
};
fn main() {
    println!("\tProblems.");
    print_problems();
    print!("Select a problem number\n:= ");
    io::stdout().flush().unwrap();
    let mut opc = String::new();
    io::stdin().read_line(&mut opc).unwrap();
    let opc = opc.trim().parse::<u32>().unwrap_or(0);
    let now = Instant::now();
    let res = match opc {
        27 => functions::problem_27(),
        28 => functions::problem_28(),
        29 => functions::problem_29(),
        30 => functions::problem_30(),
        _ => 0
    };
    let later = now.elapsed();
    println!("\n-> {} <-\n", res);
    println!("Time: {:?}\n", later);
}
fn print_problems() {
    println!("- 27: Cuadratic primes.");
    println!("- 28: Number spiral diagonals.");
    println!("- 29: Distinct powers.");
    println!("- 30: Digit fifth powers.");
}
use std::{env, process, str::FromStr};

use croon::schedule::Schedule;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Failed to run: croon expects one argument!");
        process::exit(1);
    }

    let schedule = Schedule::from_str(&args[1]).unwrap_or_else(|e| {
        eprint!("Failed to parse: {:?}", e.to_string());
        process::exit(1);
    });

    println!("{:?}", schedule);
}

use std::{env, process, str::FromStr};

use croon::cron_table::CronTab;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Failed to run: croon expects one argument!");
        process::exit(1);
    }

    let schedule = CronTab::from_str(&args[1]).unwrap_or_else(|e| {
        eprint!("Failed to parse: {:?}", e.to_string());
        process::exit(1);
    });

    print!("minute\t\t");
    print_formatted_vec(schedule.minute);
    print!("hour\t\t");
    print_formatted_vec(schedule.hour);
    print!("day of month\t");
    print_formatted_vec(schedule.day_of_month);
    print!("month\t\t");
    print_formatted_vec(schedule.month);
    print!("day of week\t");
    print_formatted_vec(schedule.day_of_week);
    println!("command\t\t{:?}", schedule.command);
}

fn print_formatted_vec(vec: Vec<u32>) {
    for item in vec {
        print!("{:?} ", item)
    }
    println!("")
}

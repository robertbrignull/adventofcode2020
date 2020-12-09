mod day1;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: cargo run --release [day|-]");
        std::process::exit(1);
    }
    let selection = &args[1];

    let mut days = std::collections::HashMap::new();
    days.insert("1", || { day1::run(); });

    for (day, implementation) in &days {
        if day == selection || selection == "-" {
            println!("Day {}:", day);
            implementation();
            println!();
        }
    } 
}

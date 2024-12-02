mod day1;
mod day2;

fn main() {
    println!("Advent of Code 2024");
    println!("===================");
    println!("Select a day to run:");
    let mut day = String::new();
    std::io::stdin().read_line(&mut day).unwrap();
    let day = day.trim();
    match day {
        "1" => day1::day1(),
        "2" => day2::day2(),
        _ => println!("Invalid day"),
    }
}

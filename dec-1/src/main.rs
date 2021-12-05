use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};

fn main() {
    let mut current = 0;
    let mut increases = 0;
    let mut decreases = 0;

    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            let value = line.unwrap();
            let number: u32 = value.parse().unwrap();

            if current == 0 {
                println!("(N/A - no previous measurement)");
            } else if current > number {
                decreases += 1;
                println!("(decreased)");
            } else if current < number {
                increases += 1;
                println!("(increased)");
            }

            current = number;
        }
    }
    println!("");
    println!("Total number of increases: {}", increases);
    println!("Total number of decreses: {}", decreases);
}

fn read_lines(filename: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

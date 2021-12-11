use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};

fn main() {
    let increases = sonar_sweep("./src/input.txt");
    println!("part one, increases: {}", increases);

    let increases = sonar_sweep_part_two("./src/input.txt");
    println!("part two, increases: {}", increases);
}

fn read_lines(filename: &str) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn sonar_sweep(filename: &str) -> i32 {
    let mut current = 0;
    let mut increases = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let value = line.unwrap();
            let number: u32 = value.parse().unwrap();

            if current < number {
                increases += 1;
            }

            current = number;
        }
    }
    increases
}

fn sonar_sweep_part_two(filename: &str) -> i32 {
    let mut lastSum = 0;
    let mut increases = 0;

    let mut window: [u32; 3] = [0; 3];

    if let Ok(lines) = read_lines(filename) {
        let lines: Vec<u32> = lines
            .map(|x| x.unwrap())
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        for i in 0..lines.len() - 2 {
            window[0] = lines[i];
            window[1] = lines[i + 1];
            window[2] = lines[i + 2];

            let sum = window[0] + window[1] + window[2];
            if current < sum {
                increases += 1;
            }
            lastSum = sum;
        }
    }
    increases
}

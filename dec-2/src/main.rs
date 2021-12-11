use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::str::FromStr;

#[derive(Debug)]
enum Directions {
    FORWARD,
    DOWN,
    UP,
}

impl FromStr for Directions {
    type Err = ();

    fn from_str(input: &str) -> Result<Directions, Self::Err> {
        match input {
            "forward" => Ok(Directions::FORWARD),
            "down" => Ok(Directions::DOWN),
            "up" => Ok(Directions::UP),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Directions,
    value: u32,
}

fn main() {
    let final_sum = dive_part_one("./src/input.txt");
    println!("{}", final_sum);
}

fn read_lines(filename: &str) -> std::io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn dive_part_one(filename: &str) -> u32 {
    let sum = if let Ok(lines) = read_lines(filename) {
        let instructions: Vec<Instruction> = lines
            .map(|l| l.unwrap())
            .map(|l| {
                let mut values = l.split_whitespace();
                let direction = Directions::from_str(values.next().unwrap()).unwrap();
                let value = values.next().unwrap().parse::<u32>().unwrap();
                Instruction { direction, value }
            })
            .collect();

        let mut horizontal = 0;
        let mut depth = 0;

        for instr in instructions {
            match instr.direction {
                Directions::FORWARD => horizontal += instr.value,
                Directions::DOWN => depth += instr.value,
                Directions::UP => depth -= instr.value,
            }
        }
        horizontal * depth
    } else {
        0
    };
    sum
}

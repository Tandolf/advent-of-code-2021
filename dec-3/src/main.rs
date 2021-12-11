use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn main() {
    if let Ok(lines) = read_lines("./src/input.txt") {
        let values: Vec<_> = lines.map(|x| x.unwrap()).collect();

        let mut gamma_rate = Vec::new();
        let mut epsilon_rate = Vec::new();

        for (i, _) in values[0].chars().enumerate() {
            let mut ones = 0;
            let mut zeros = 0;

            for value in &values {
                let v = value.chars().nth(i).unwrap();

                match v {
                    '1' => ones += 1,
                    '0' => zeros += 1,
                    _ => (),
                }
            }

            if ones > zeros {
                gamma_rate.push("1");
                epsilon_rate.push("0");
            } else {
                gamma_rate.push("0");
                epsilon_rate.push("1");
            }
        }

        let gamma_rate = gamma_rate.join("");
        let gamma_rate = u32::from_str_radix(&gamma_rate, 2).unwrap();
        let epsilon_rate = epsilon_rate.join("");
        let epsilon_rate = u32::from_str_radix(&epsilon_rate, 2).unwrap();

        println!("total power consumption is: {}", gamma_rate * epsilon_rate);
    }
}

fn read_lines(filename: &str) -> std::io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

fn main() {
    let _power_consumption = get_power_consumption("./src/input.txt");
    //println!("total power consumption is: {}", power_consumption);

    if let Ok(lines) = read_lines("./src/input.txt") {
        let strings: Vec<_> = lines.map(|v| v.unwrap()).collect();

        let oxygen_generator_rating = get_oxygen_generator_rating(&strings, 0);
        let oxygen_generator_rating: u32 =
            u32::from_str_radix(&oxygen_generator_rating, 2).unwrap();

        let co2_scrubber_rating = get_co2_scrubber_rating(&strings, 0);
        let co2_scrubber_rating: u32 = u32::from_str_radix(&co2_scrubber_rating, 2).unwrap();

        println!("oxygen generator rating: {}", oxygen_generator_rating);
        println!("CO2 scrubber rating: {}", co2_scrubber_rating);

        println!(
            "Life support rating: {}",
            oxygen_generator_rating * &co2_scrubber_rating
        );
    }
}

fn read_lines(filename: &str) -> std::io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn get_power_consumption(filename: &str) -> u32 {
    if let Ok(lines) = read_lines(filename) {
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

        let sum = epsilon_rate * gamma_rate;
        sum
    } else {
        0
    }
}

fn get_oxygen_generator_rating(values: &Vec<String>, mut index: usize) -> String {
    if values.len() == 1 || values[0].chars().count() == index {
        return values[0].clone();
    }

    let mut ones = Vec::new();
    let mut zeros = Vec::new();

    for value in values {
        let v: Vec<_> = value.chars().collect();
        if v[index] == '1' {
            ones.push(value.clone());
        } else if v[index] == '0' {
            zeros.push(value.clone());
        }
    }

    index += 1;
    let mut values = if ones.len() > zeros.len() {
        ones
    } else if ones.len() < zeros.len() {
        zeros
    } else {
        ones
    };

    get_oxygen_generator_rating(&mut values, index)
}

fn get_co2_scrubber_rating(values: &Vec<String>, mut index: usize) -> String {
    if values.len() == 1 || values[0].chars().count() == index {
        return values[0].clone();
    }

    let mut ones = Vec::new();
    let mut zeros = Vec::new();

    for value in values {
        let v: Vec<_> = value.chars().collect();
        if v[index] == '1' {
            ones.push(value.clone());
        } else if v[index] == '0' {
            zeros.push(value.clone());
        }
    }

    index += 1;
    let mut values = if ones.len() < zeros.len() {
        ones
    } else if ones.len() > zeros.len() {
        zeros
    } else {
        zeros
    };

    get_co2_scrubber_rating(&mut values, index)
}
